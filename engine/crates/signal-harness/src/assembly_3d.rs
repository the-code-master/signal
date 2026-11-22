// Module: engine/crates/signal-harness/src/assembly_3d.rs — Multi-Board 3D Mate Solver, Clipping Plane & Net Tracer (FEAT-061)
// Invariants: Solves coplanar/coaxial mates in exact nm; traces end-to-end nets across mated connectors (FEAT-061).
// Spec: architecture/17-extended-feature-specifications-ui-and-rust.md, architecture/19-cad-cam-3d-printing-and-plm-operations.md
use crate::HarnessError;
use signal_core::Nanometers;
use signal_geom::{BBox2Nm, BBox3Nm, Point2Nm, Point3Nm};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubBoardInstance {
    pub board_id: String,
    pub local_size_nm: Point3Nm,
    pub origin_nm: Point3Nm,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AssemblyMateConstraint {
    CoplanarZ {
        target_board_id: String,
        reference_board_id: String,
        standoff_z_nm: i64,
    },
    CoaxialXY {
        target_board_id: String,
        center_x_nm: i64,
        center_y_nm: i64,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConnectorMatePinPair {
    pub from_board: String,
    pub from_net: String,
    pub to_board: String,
    pub to_net: String,
    pub pin_number: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MultiBoardAssemblyReport {
    pub solved_board_bounds: BTreeMap<String, BBox3Nm>,
    pub enclosure_collision_count: u32,
    pub board_to_board_collision_count: u32,
}

fn boxes_intersect_strict(a: &BBox3Nm, b: &BBox3Nm) -> bool {
    a.min.x.0 < b.max.x.0
        && a.max.x.0 > b.min.x.0
        && a.min.y.0 < b.max.y.0
        && a.max.y.0 > b.min.y.0
        && a.min.z.0 < b.max.z.0
        && a.max.z.0 > b.min.z.0
}

/// Solves coplanar & coaxial 3D assembly mates across multiple PCBs and checks enclosure collisions (`FEAT-061`).
pub fn solve_multiboard_mates(
    boards: &[SubBoardInstance],
    mates: &[AssemblyMateConstraint],
    enclosure_keepouts_nm: &[BBox3Nm],
) -> Result<MultiBoardAssemblyReport, HarnessError> {
    if boards.is_empty() {
        return Err(HarnessError::EmptyAssembly);
    }
    let mut state: BTreeMap<String, SubBoardInstance> = boards
        .iter()
        .map(|b| (b.board_id.clone(), b.clone()))
        .collect();

    for mate in mates {
        match mate {
            AssemblyMateConstraint::CoplanarZ {
                target_board_id,
                reference_board_id,
                standoff_z_nm,
            } => {
                let ref_top_z = state
                    .get(reference_board_id)
                    .map(|r| r.origin_nm.z.0.saturating_add(r.local_size_nm.z.0))
                    .ok_or_else(|| HarnessError::UnknownBoard(reference_board_id.clone()))?;
                let target = state
                    .get_mut(target_board_id)
                    .ok_or_else(|| HarnessError::UnknownBoard(target_board_id.clone()))?;
                target.origin_nm.z = Nanometers(ref_top_z.saturating_add(*standoff_z_nm));
            }
            AssemblyMateConstraint::CoaxialXY {
                target_board_id,
                center_x_nm,
                center_y_nm,
            } => {
                let target = state
                    .get_mut(target_board_id)
                    .ok_or_else(|| HarnessError::UnknownBoard(target_board_id.clone()))?;
                target.origin_nm.x =
                    Nanometers(center_x_nm.saturating_sub(target.local_size_nm.x.0 / 2));
                target.origin_nm.y =
                    Nanometers(center_y_nm.saturating_sub(target.local_size_nm.y.0 / 2));
            }
        }
    }

    let mut bounds_map = BTreeMap::new();
    for (id, b) in &state {
        bounds_map.insert(
            id.clone(),
            BBox3Nm {
                min: b.origin_nm,
                max: Point3Nm {
                    x: Nanometers(b.origin_nm.x.0.saturating_add(b.local_size_nm.x.0)),
                    y: Nanometers(b.origin_nm.y.0.saturating_add(b.local_size_nm.y.0)),
                    z: Nanometers(b.origin_nm.z.0.saturating_add(b.local_size_nm.z.0)),
                },
            },
        );
    }

    let vals: Vec<&BBox3Nm> = bounds_map.values().collect();
    let mut b2b_collisions = 0_u32;
    for i in 0..vals.len() {
        for j in (i + 1)..vals.len() {
            if boxes_intersect_strict(vals[i], vals[j]) {
                b2b_collisions += 1;
            }
        }
    }

    let mut enc_collisions = 0_u32;
    for b_box in &vals {
        for keepout in enclosure_keepouts_nm {
            if boxes_intersect_strict(b_box, keepout) {
                enc_collisions += 1;
            }
        }
    }

    Ok(MultiBoardAssemblyReport {
        solved_board_bounds: bounds_map,
        enclosure_collision_count: enc_collisions,
        board_to_board_collision_count: b2b_collisions,
    })
}

/// Computes 2D cross-section rectangles formed by slicing assembly 3D bounding boxes at plane $z = z_{\text{clip}}$ (`FEAT-061`).
pub fn slice_assembly_z_clipping_plane(
    board_bounds: &BTreeMap<String, BBox3Nm>,
    z_clip_nm: i64,
) -> Vec<(String, BBox2Nm)> {
    let mut sections = Vec::new();
    for (id, b) in board_bounds {
        if b.min.z.0 <= z_clip_nm && z_clip_nm <= b.max.z.0 {
            sections.push((
                id.clone(),
                BBox2Nm {
                    min: Point2Nm::new(b.min.x.0, b.min.y.0),
                    max: Point2Nm::new(b.max.x.0, b.max.y.0),
                },
            ));
        }
    }
    sections
}

/// Traces end-to-end electrical net continuity across mated board-to-board connectors (`FEAT-061`).
pub fn trace_multiboard_connector_nets(
    pin_mates: &[ConnectorMatePinPair],
) -> Result<BTreeMap<String, Vec<String>>, HarnessError> {
    let mut chains: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for pair in pin_mates {
        if pair.from_net.is_empty() || pair.to_net.is_empty() {
            return Err(HarnessError::UnroutedConnectorPin(pair.pin_number));
        }
        let key = format!("{}:{}", pair.from_board, pair.from_net);
        chains.entry(key).or_default().push(format!(
            "{}:{}@pin{}",
            pair.to_board, pair.to_net, pair.pin_number
        ));
    }
    Ok(chains)
}
