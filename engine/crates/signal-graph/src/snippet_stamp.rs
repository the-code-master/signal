// Module: engine/crates/signal-graph/src/snippet_stamp.rs — Reusable Golden Snippet Stamper (FEAT-079)
// Invariants: Applies deterministic refdes base offset, coordinate anchor translation, and local net scoping while preserving global rails.
// Spec: architecture/17-extended-feature-specifications-ui-and-rust.md, architecture/18-universal-component-and-datasheet-repository.md

use crate::GraphError;
use signal_geom::Point2Nm;
use signal_proto::v1::{StampGoldenSnippetRequest, StampGoldenSnippetResponse};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SnippetComponentTemplate {
    pub refdes_prefix: String,
    pub local_index: u32,
    pub upid: String,
    pub relative_pos_nm: Point2Nm,
    pub pin_nets: Vec<(String, String)>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SnippetTraceTemplate {
    pub local_net_name: String,
    pub start_rel_nm: Point2Nm,
    pub end_rel_nm: Point2Nm,
    pub width_nm: i64,
    pub layer_index: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GoldenSnippetTemplate {
    pub snippet_id: String,
    pub components: Vec<SnippetComponentTemplate>,
    pub pre_routed_traces: Vec<SnippetTraceTemplate>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StampedComponentInstance {
    pub stamped_refdes: String,
    pub upid: String,
    pub world_pos_nm: Point2Nm,
    pub resolved_pin_nets: Vec<(String, String)>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StampedPreRoutedTrace {
    pub resolved_net_name: String,
    pub start_world_nm: Point2Nm,
    pub end_world_nm: Point2Nm,
    pub width_nm: i64,
    pub layer_index: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StampedSnippetBundle {
    pub response: StampGoldenSnippetResponse,
    pub components: Vec<StampedComponentInstance>,
    pub traces: Vec<StampedPreRoutedTrace>,
}

fn is_global_power_or_ground(net: &str) -> bool {
    matches!(
        net.trim().to_ascii_uppercase().as_str(),
        "0" | "GND" | "VSS" | "AGND" | "DGND" | "PGND" | "VCC" | "VDD" | "VBUS" | "3V3" | "5V"
    )
}

fn remap_snippet_net(net: &str, refdes_base_offset: u32) -> String {
    if is_global_power_or_ground(net) {
        net.trim().to_string()
    } else {
        format!("SNIP_{refdes_base_offset}_{}", net.trim())
    }
}

/// Stamps a verified Golden Snippet (hierarchical schematic subcircuit + pre-routed trace segments)
/// onto a target graph branch with deterministic refdes offsetting (`R1` + `200` -> `R201`),
/// exact `i128`-safe coordinate translation, and isolated local net names (`FEAT-079`).
pub fn stamp_golden_snippet(
    template: &GoldenSnippetTemplate,
    req: &StampGoldenSnippetRequest,
    current_revision: u64,
) -> Result<StampedSnippetBundle, GraphError> {
    if req.snippet_id.trim().is_empty() || req.snippet_id != template.snippet_id {
        return Err(GraphError::InvalidCommandOrSlice(format!(
            "snippet_id mismatch (`{}` vs `{}`)",
            req.snippet_id, template.snippet_id
        )));
    }
    if req.target_branch_id.trim().is_empty() {
        return Err(GraphError::UnknownBranch(req.target_branch_id.clone()));
    }

    let anchor = req
        .anchor_nm
        .as_ref()
        .map_or(Point2Nm::new(0, 0), |p| Point2Nm::new(p.x_nm, p.y_nm));

    let mut stamped_components = Vec::with_capacity(template.components.len());
    for comp in &template.components {
        let num = comp.local_index.saturating_add(req.refdes_base_offset);
        let stamped_refdes = format!("{}{num}", comp.refdes_prefix.trim());
        let world_pos_nm = Point2Nm::new(
            anchor.x.0.saturating_add(comp.relative_pos_nm.x.0),
            anchor.y.0.saturating_add(comp.relative_pos_nm.y.0),
        );
        let resolved_pin_nets = comp
            .pin_nets
            .iter()
            .map(|(pin, net)| (pin.clone(), remap_snippet_net(net, req.refdes_base_offset)))
            .collect();
        stamped_components.push(StampedComponentInstance {
            stamped_refdes,
            upid: comp.upid.clone(),
            world_pos_nm,
            resolved_pin_nets,
        });
    }

    let mut stamped_traces = Vec::with_capacity(template.pre_routed_traces.len());
    for tr in &template.pre_routed_traces {
        stamped_traces.push(StampedPreRoutedTrace {
            resolved_net_name: remap_snippet_net(&tr.local_net_name, req.refdes_base_offset),
            start_world_nm: Point2Nm::new(
                anchor.x.0.saturating_add(tr.start_rel_nm.x.0),
                anchor.y.0.saturating_add(tr.start_rel_nm.y.0),
            ),
            end_world_nm: Point2Nm::new(
                anchor.x.0.saturating_add(tr.end_rel_nm.x.0),
                anchor.y.0.saturating_add(tr.end_rel_nm.y.0),
            ),
            width_nm: tr.width_nm,
            layer_index: tr.layer_index,
        });
    }

    let entity_count = (stamped_components.len() + stamped_traces.len()) as u32;
    let new_revision = current_revision.saturating_add(1);

    Ok(StampedSnippetBundle {
        response: StampGoldenSnippetResponse {
            stamped_entity_count: entity_count,
            new_revision,
        },
        components: stamped_components,
        traces: stamped_traces,
    })
}
