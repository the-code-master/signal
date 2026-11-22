// Module: engine/crates/signal-security-hw/src/lib.rs — Hardware Security DPA/TVLA, Fault-Injection & GLIFT Verifier
// Invariants: Detects side-channel |t| > 4.5 leakage, Hamming-<=2 FSM glitch states & unmasked key I/O paths (FEAT-050).
// Spec: architecture/17-extended-feature-specifications-ui-and-rust.md, architecture/15-application-security-and-project-isolation.md
#![forbid(unsafe_code)]

use thiserror::Error;

pub mod glift_trojan;
pub mod tvla_cpa_glitch;

pub use glift_trojan::{
    GliftGateNode, GliftGateOp, HwSecurityCircuitModel, audit_hardware_security_tvla,
    verify_glift_secret_isolation,
};
pub use tvla_cpa_glitch::{
    TVLA_WELCH_T_THRESHOLD, compute_cpa_key_correlation, compute_welch_tvla_t_stat,
    count_glitch_vulnerable_fsm_states, simulate_cmos_hamming_power_traces,
};

#[derive(Debug, Error, PartialEq, Eq)]
pub enum HwSecurityError {
    #[error("TVLA requires at least 4 power traces per group (got {0})")]
    InsufficientPowerTraces(usize),
}

#[cfg(test)]
mod tests {
    use super::*;
    use signal_proto::v1::AuditHardwareSecurityTvlaRequest;

    #[test]
    fn flags_unmasked_sbox_tvla_and_trojan_mux_while_passing_masked_design() {
        let req = AuditHardwareSecurityTvlaRequest {
            branch_id: "aes-core".into(),
            power_trace_count: 256,
            run_gate_level_ift: true,
        };
        let vuln = audit_hardware_security_tvla(
            &req,
            &HwSecurityCircuitModel {
                boolean_masked_datapath: false,
                gates: vec![GliftGateNode {
                    out_net: "debug_gpio_out".into(),
                    op: GliftGateOp::And("aes_key_bit0".into(), "trojan_trigger".into()),
                }],
                secret_nets: vec!["aes_key_bit0".into()],
                fresh_masks: vec![],
                primary_outputs: vec!["debug_gpio_out".into()],
                fsm_non_priv: vec![0b0000],
                fsm_priv: vec![0b0001], // HD = 1 -> vulnerable!
            },
        )
        .expect("unmasked audit");
        assert!(vuln.tvla_leakage_detected);
        assert!(!vuln.key_to_unmasked_io_path_free);
        assert_eq!(vuln.glitch_vulnerable_fsm_states, 1);

        let safe = audit_hardware_security_tvla(
            &req,
            &HwSecurityCircuitModel {
                boolean_masked_datapath: true,
                gates: vec![GliftGateNode {
                    out_net: "ciphertext_bus".into(),
                    op: GliftGateOp::FreshMaskXor {
                        data_net: "aes_key_bit0".into(),
                        fresh_mask_net: "trng_mask_0".into(),
                    },
                }],
                secret_nets: vec!["aes_key_bit0".into()],
                fresh_masks: vec!["trng_mask_0".into()],
                primary_outputs: vec!["ciphertext_bus".into()],
                fsm_non_priv: vec![0x5A3C],
                fsm_priv: vec![0xA5C3], // HD = 16 >= 4!
            },
        )
        .expect("masked audit");
        assert!(!safe.tvla_leakage_detected);
        assert!(safe.key_to_unmasked_io_path_free);
        assert_eq!(safe.glitch_vulnerable_fsm_states, 0);
    }
}
