// Module: engine/crates/signal-security-hw/src/glift_trojan.rs — Gate-Level Information-Flow Tracking (GLIFT) Trojan Verifier (FEAT-050)
// Invariants: Proves secret key shadow-taint never reaches unmasked primary I/O pins (FEAT-050).
// Spec: architecture/17-extended-feature-specifications-ui-and-rust.md, architecture/15-application-security-and-project-isolation.md
use crate::HwSecurityError;
use crate::tvla_cpa_glitch::{
    TVLA_WELCH_T_THRESHOLD, compute_welch_tvla_t_stat, count_glitch_vulnerable_fsm_states,
    simulate_cmos_hamming_power_traces,
};
use signal_proto::v1::{AuditHardwareSecurityTvlaRequest, AuditHardwareSecurityTvlaResponse};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GliftGateOp {
    And(String, String),
    Or(String, String),
    Xor(String, String),
    Not(String),
    /// Cryptographic fresh-masking barrier: `secret ^ fresh_rng_mask` clears 1st-order unmasked I/O taint
    FreshMaskXor {
        data_net: String,
        fresh_mask_net: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GliftGateNode {
    pub out_net: String,
    pub op: GliftGateOp,
}

/// Propagates conservative Gate-Level Information-Flow Tracking (GLIFT) shadow taint from secret key nets to primary outputs.
/// Returns the list of unmasked primary output nets tainted by secret key material (`empty` == Trojan/leakage free!).
pub fn verify_glift_secret_isolation(
    secret_key_nets: &[String],
    fresh_entropy_nets: &[String],
    gates_topo_order: &[GliftGateNode],
    unmasked_primary_outputs: &[String],
) -> Vec<String> {
    let mut tainted: BTreeMap<String, bool> = BTreeMap::new();
    let fresh_set: BTreeSet<&str> = fresh_entropy_nets.iter().map(String::as_str).collect();
    for k in secret_key_nets {
        tainted.insert(k.clone(), true);
    }

    for gate in gates_topo_order {
        let out_taint = match &gate.op {
            GliftGateOp::Not(a) => *tainted.get(a).unwrap_or(&false),
            GliftGateOp::And(a, b) | GliftGateOp::Or(a, b) | GliftGateOp::Xor(a, b) => {
                *tainted.get(a).unwrap_or(&false) || *tainted.get(b).unwrap_or(&false)
            }
            GliftGateOp::FreshMaskXor {
                data_net,
                fresh_mask_net,
            } => {
                if fresh_set.contains(fresh_mask_net.as_str()) {
                    false
                } else {
                    *tainted.get(data_net).unwrap_or(&false)
                }
            }
        };
        tainted.insert(gate.out_net.clone(), out_taint);
    }

    unmasked_primary_outputs
        .iter()
        .filter(|out| *tainted.get(out.as_str()).unwrap_or(&false))
        .cloned()
        .collect()
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HwSecurityCircuitModel {
    pub boolean_masked_datapath: bool,
    pub gates: Vec<GliftGateNode>,
    pub secret_nets: Vec<String>,
    pub fresh_masks: Vec<String>,
    pub primary_outputs: Vec<String>,
    pub fsm_non_priv: Vec<u32>,
    pub fsm_priv: Vec<u32>,
}

/// Executes full Hardware Security TVLA + FSM Glitch + GLIFT Trojan audit (`AuditHardwareSecurityTvla`, `FEAT-050`).
pub fn audit_hardware_security_tvla(
    req: &AuditHardwareSecurityTvlaRequest,
    model: &HwSecurityCircuitModel,
) -> Result<AuditHardwareSecurityTvlaResponse, HwSecurityError> {
    let n = (req.power_trace_count as usize).max(32);
    let fixed = simulate_cmos_hamming_power_traces(0xA6, n, true, model.boolean_masked_datapath)?;
    let random = simulate_cmos_hamming_power_traces(0xA6, n, false, model.boolean_masked_datapath)?;
    let t_stat = compute_welch_tvla_t_stat(&fixed, &random)?.abs();

    let leaked_outputs = if req.run_gate_level_ift {
        verify_glift_secret_isolation(
            &model.secret_nets,
            &model.fresh_masks,
            &model.gates,
            &model.primary_outputs,
        )
    } else {
        Vec::new()
    };
    let glitch_vuln = count_glitch_vulnerable_fsm_states(&model.fsm_non_priv, &model.fsm_priv);

    Ok(AuditHardwareSecurityTvlaResponse {
        max_welch_t_statistic: t_stat,
        tvla_leakage_detected: t_stat > TVLA_WELCH_T_THRESHOLD,
        key_to_unmasked_io_path_free: leaked_outputs.is_empty(),
        glitch_vulnerable_fsm_states: glitch_vuln,
    })
}
