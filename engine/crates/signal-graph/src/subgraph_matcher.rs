// Module: engine/crates/signal-graph/src/subgraph_matcher.rs — Subgraph Isomorphism & Symbolic Stage Explainer (FEAT-101)
// Invariants: Recognizes canonical analog/power topologies via structural pin-role incidence & emits symbolic H(s) strings.
// Spec: architecture/05-ai-agent-and-datasheet-rag.md, architecture/17-extended-feature-specifications-ui-and-rust.md

use signal_geom::{BBox2Nm, Point2Nm};
use signal_proto::v1::{BoundingBox2dNm, CircuitStageExplanation, ExplainCircuitStagesResponse};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StagePrimitiveKind {
    Resistor,
    Capacitor,
    Inductor,
    OpAmp,
    MosfetSwitch,
    BjtOrMosPair,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StageComponentNode {
    pub refdes: String,
    pub kind: StagePrimitiveKind,
    pub value_si: f64,
    pub nets: Vec<String>,
    pub pos_nm: Point2Nm,
}

fn is_supply_or_gnd(net: &str) -> bool {
    matches!(
        net.trim().to_ascii_uppercase().as_str(),
        "0" | "GND"
            | "VSS"
            | "AGND"
            | "DGND"
            | "PGND"
            | "VCC"
            | "VDD"
            | "VIN"
            | "VEE"
            | "3V3"
            | "5V"
    )
}

fn is_gnd(net: &str) -> bool {
    matches!(
        net.trim().to_ascii_uppercase().as_str(),
        "0" | "GND" | "VSS" | "AGND" | "DGND" | "PGND"
    )
}

fn stage_bbox_proto(nodes: &[&StageComponentNode]) -> BoundingBox2dNm {
    let first = nodes.first().map_or(Point2Nm::ORIGIN, |n| n.pos_nm);
    let mut bb = BBox2Nm::from_point(first);
    for n in nodes.iter().skip(1) {
        bb = bb.union(&BBox2Nm::from_point(n.pos_nm));
    }
    BoundingBox2dNm {
        min_x_nm: bb.min.x.0 - 1_000_000,
        min_y_nm: bb.min.y.0 - 1_000_000,
        max_x_nm: bb.max.x.0 + 1_000_000,
        max_y_nm: bb.max.y.0 + 1_000_000,
    }
}

/// Recognizes functional circuit stages via structural net-incidence subgraph matching (`FEAT-101`):
/// Buck LC power stage, Op-Amp feedback amplifier, RC low-pass filter, and Resistive Voltage Divider.
pub fn explain_circuit_stages(components: &[StageComponentNode]) -> ExplainCircuitStagesResponse {
    let mut by_net: BTreeMap<String, Vec<usize>> = BTreeMap::new();
    for (idx, c) in components.iter().enumerate() {
        for net in &c.nets {
            by_net.entry(net.clone()).or_default().push(idx);
        }
    }

    let mut used = vec![false; components.len()];
    let mut stages = Vec::new();
    let mut next_id = 1_u32;

    // 1. Buck Converter Stage: MosfetSwitch + Inductor sharing SW node + Capacitor to GND on VOUT
    for (sw_idx, sw) in components.iter().enumerate() {
        if sw.kind != StagePrimitiveKind::MosfetSwitch || used[sw_idx] {
            continue;
        }
        for sw_net in sw.nets.iter().filter(|n| !is_supply_or_gnd(n)) {
            let Some(peers) = by_net.get(sw_net) else {
                continue;
            };
            let Some(&l_idx) = peers
                .iter()
                .find(|&&i| !used[i] && components[i].kind == StagePrimitiveKind::Inductor)
            else {
                continue;
            };
            let ind = &components[l_idx];
            if let Some(vout_net) = ind
                .nets
                .iter()
                .find(|&n| n != sw_net && !is_supply_or_gnd(n))
                && let Some(out_peers) = by_net.get(vout_net)
                && let Some(&c_idx) = out_peers.iter().find(|&&i| {
                    !used[i]
                        && components[i].kind == StagePrimitiveKind::Capacitor
                        && components[i].nets.iter().any(|n| is_gnd(n))
                })
            {
                used[sw_idx] = true;
                used[l_idx] = true;
                used[c_idx] = true;
                let f0_khz = (1.0
                    / (2.0
                        * std::f64::consts::PI
                        * (ind.value_si * components[c_idx].value_si)
                            .max(1e-24)
                            .sqrt()))
                    / 1_000.0;
                stages.push(CircuitStageExplanation {
                    stage_id: format!("stage-{next_id}"),
                    topology_name: "Synchronous Buck Power Stage".to_string(),
                    symbolic_transfer_fn: format!(
                        "H(s) = D / (1 + s*(L/R_load) + s^2*L*C), f_0 = {f0_khz:.2} kHz"
                    ),
                    bounds_nm: Some(stage_bbox_proto(&[sw, ind, &components[c_idx]])),
                });
                next_id += 1;
                break;
            }
        }
    }

    // 2. Inverting / Feedback Op-Amp Stage: OpAmp + 2 Resistors sharing inverting input node
    for (op_idx, op) in components.iter().enumerate() {
        if op.kind != StagePrimitiveKind::OpAmp || used[op_idx] {
            continue;
        }
        let mut r_attached = Vec::new();
        for net in op.nets.iter().filter(|n| !is_supply_or_gnd(n)) {
            if let Some(peers) = by_net.get(net) {
                for &pi in peers {
                    if !used[pi]
                        && components[pi].kind == StagePrimitiveKind::Resistor
                        && !r_attached.contains(&pi)
                    {
                        r_attached.push(pi);
                    }
                }
            }
        }
        if r_attached.len() >= 2 {
            let r1 = &components[r_attached[0]];
            let r2 = &components[r_attached[1]];
            used[op_idx] = true;
            used[r_attached[0]] = true;
            used[r_attached[1]] = true;
            let gain = -r2.value_si.max(r1.value_si) / r1.value_si.min(r2.value_si).max(1e-9);
            stages.push(CircuitStageExplanation {
                stage_id: format!("stage-{next_id}"),
                topology_name: "Feedback Operational Amplifier".to_string(),
                symbolic_transfer_fn: format!("H(s) = -R_f / R_in = {gain:.3} V/V"),
                bounds_nm: Some(stage_bbox_proto(&[op, r1, r2])),
            });
            next_id += 1;
        }
    }

    // 3. RC Low-Pass Filter & 4. Resistive Voltage Divider on internal 2-element midpoint nets
    for (mid_net, members) in &by_net {
        if is_supply_or_gnd(mid_net) || members.len() != 2 {
            continue;
        }
        let (a_idx, b_idx) = (members[0], members[1]);
        if used[a_idx] || used[b_idx] {
            continue;
        }
        let (ca, cb) = (&components[a_idx], &components[b_idx]);
        match (ca.kind, cb.kind) {
            (StagePrimitiveKind::Resistor, StagePrimitiveKind::Capacitor)
            | (StagePrimitiveKind::Capacitor, StagePrimitiveKind::Resistor) => {
                let (r_node, c_node) = if ca.kind == StagePrimitiveKind::Resistor {
                    (ca, cb)
                } else {
                    (cb, ca)
                };
                if c_node.nets.iter().any(|n| is_gnd(n)) {
                    used[a_idx] = true;
                    used[b_idx] = true;
                    let fc_hz = 1.0
                        / (2.0
                            * std::f64::consts::PI
                            * (r_node.value_si * c_node.value_si).max(1e-24));
                    stages.push(CircuitStageExplanation {
                        stage_id: format!("stage-{next_id}"),
                        topology_name: "First-Order RC Low-Pass Filter".to_string(),
                        symbolic_transfer_fn: format!(
                            "H(s) = 1 / (1 + s*R*C), f_c = {fc_hz:.2} Hz"
                        ),
                        bounds_nm: Some(stage_bbox_proto(&[r_node, c_node])),
                    });
                    next_id += 1;
                }
            }
            (StagePrimitiveKind::Resistor, StagePrimitiveKind::Resistor) => {
                let (r_top, r_bot) = if cb.nets.iter().any(|n| is_gnd(n)) {
                    (ca, cb)
                } else {
                    (cb, ca)
                };
                if r_bot.nets.iter().any(|n| is_gnd(n)) {
                    used[a_idx] = true;
                    used[b_idx] = true;
                    let ratio = r_bot.value_si / (r_top.value_si + r_bot.value_si).max(1e-12);
                    stages.push(CircuitStageExplanation {
                        stage_id: format!("stage-{next_id}"),
                        topology_name: "Resistive Voltage Divider".to_string(),
                        symbolic_transfer_fn: format!(
                            "H(s) = R_bot / (R_top + R_bot) = {ratio:.4}"
                        ),
                        bounds_nm: Some(stage_bbox_proto(&[r_top, r_bot])),
                    });
                    next_id += 1;
                }
            }
            _ => {}
        }
    }

    ExplainCircuitStagesResponse { stages }
}
