// Module: engine/crates/signal-fpga/src/lib.rs — FPGA Studio: Bank I/O, SDC/XDC CDC, FSM Codegen & Fixed-Point DSP
// Invariants: Rejects conflicting bank V_CCO/SSN, unsynchronized CDCs & deadlocked FSMs; verifies FIR SQNR (FEAT-051..060).
// Spec: architecture/17-extended-feature-specifications-ui-and-rust.md
#![forbid(unsafe_code)]

use thiserror::Error;

pub mod fsm_and_dsp;
pub mod io_and_sdc;

pub use fsm_and_dsp::{
    FixedPointFirResult, FsmTransition, plan_fpga_io_and_dsp, synthesize_fixed_point_fir_qmn,
    verify_and_emit_fsm_systemverilog,
};
pub use io_and_sdc::{
    CdcCrossingArc, FpgaIoPlanReport, FpgaPinAssignment, plan_fpga_bank_io_and_escape,
    verify_sdc_cdc_constraints,
};

#[derive(Debug, Error, PartialEq, Eq)]
pub enum FpgaError {
    #[error("invalid FSM topology: {0}")]
    InvalidFsmGraph(String),
    #[error("invalid fixed-point FIR tap count or fractional bit width ({0})")]
    InvalidDspParameters(usize),
}

#[cfg(test)]
mod tests {
    use super::*;
    use signal_proto::v1::PlanFpgaIoAndDspRequest;

    #[test]
    fn verifies_bank_vcco_cdc_fsm_deadlock_detection_and_qmn_fir_sqnr() {
        let bad_pins = [
            FpgaPinAssignment {
                signal_name: "p0".into(),
                bank_id: 65,
                required_vcco_mv: 1_800,
                is_lvds_diff: false,
                drive_ma: 12,
                bga_ball_order: 1,
                pcb_target_order: 2,
            },
            FpgaPinAssignment {
                signal_name: "p1".into(),
                bank_id: 65,
                required_vcco_mv: 3_300, // conflicts with 1.8V bank!
                is_lvds_diff: false,
                drive_ma: 12,
                bga_ball_order: 2,
                pcb_target_order: 1,
            },
        ];
        let io_bad = plan_fpga_bank_io_and_escape(&bad_pins, 100);
        assert!(!io_bad.vcco_compatible);
        assert_eq!(io_bad.pcb_escape_crossings, 1);

        let sv = verify_and_emit_fsm_systemverilog(
            "pcie_ltssm",
            &["DETECT".into(), "POLLING".into(), "L0".into()],
            "DETECT",
            &["L0".into()],
            &[
                FsmTransition {
                    from_state: "DETECT".into(),
                    guard_expr: "rx_present".into(),
                    to_state: "POLLING".into(),
                },
                FsmTransition {
                    from_state: "POLLING".into(),
                    guard_expr: "ts2_aligned".into(),
                    to_state: "L0".into(),
                },
            ],
        )
        .expect("valid fsm");
        assert!(sv.contains("always_ff"));

        let resp = plan_fpga_io_and_dsp(&PlanFpgaIoAndDspRequest {
            device_part: "XCVU19P-2FSVA3824E".into(),
            io_pin_count: 32,
            fir_tap_count: 31,
            fixed_point_frac_bits: 15,
        })
        .expect("fpga plan");
        assert!(resp.bank_vcco_and_sdc_clean);
        assert_eq!(resp.pcb_escape_crossings, 0);
        assert!(resp.quantized_sqnr_db > 70.0);
        assert_eq!(resp.generated_rtl_digest.len(), 64);
    }
}
