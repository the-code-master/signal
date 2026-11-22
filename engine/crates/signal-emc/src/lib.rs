// Module: engine/crates/signal-emc/src/lib.rs — CISPR/FCC EMC, EMI Filter, ESD/Surge, Cavity, FMEDA, FTA, WCCA & Rad-Hard
// Invariants: #![forbid(unsafe_code)]; deterministic EMC pre-compliance & ISO 26262 FMEDA / minimal cut-set solvers.
// Spec: architecture/03-rust-eda-engine.md, architecture/17-extended-feature-specifications-ui-and-rust.md
#![forbid(unsafe_code)]

use thiserror::Error;

pub mod emi_esd_cavity;
pub mod safety_wcca_rad;

pub use emi_esd_cavity::{
    compute_enclosure_cavity_and_slot_se, evaluate_cispr32_precompliance,
    evaluate_mlcc_piezo_acoustic_risk_dba, verify_emi_filter_middlebrook_stability,
    verify_iec_esd_and_surge_tvs_clamping,
};
pub use safety_wcca_rad::{
    FaultTreeNode, FmedaPartFailureEntry, compute_iso26262_fmeda, compute_wcca_eva_and_rss,
    evaluate_heavy_ion_let_seu, run_automated_fmeda_rpc, run_emc_precompliance_and_safety_rpc,
    solve_fault_tree_minimal_cut_sets, verify_ecss_component_derating,
};

#[derive(Debug, Error, PartialEq, Eq)]
pub enum EmcError {
    #[error("missing fencing authority on automated FMEDA run")]
    MissingFencing,
}

#[cfg(test)]
mod tests {
    use super::*;
    use signal_proto::v1::RunEmcPrecomplianceAndSafetyRequest;

    #[test]
    fn cispr32_middlebrook_esd_cavity_piezo_fmeda_fta_wcca_and_radhard_verified() {
        let resp = run_emc_precompliance_and_safety_rpc(&RunEmcPrecomplianceAndSafetyRequest {
            branch_id: "main".into(),
            limit_standard: "CISPR_32_CLASS_B".into(),
            safety_asil_level: "ASIL_D".into(),
            mission_years: 15,
        });
        assert!(
            resp.cispr_radiated_conducted_pass && resp.spfm_pct >= 99.0 && resp.lfm_pct >= 90.0
        );
        assert!(resp.pmhf_fit < 10.0 && resp.end_of_life_survival_prob > 0.90);

        let (mb_ok, mb_db) =
            verify_emi_filter_middlebrook_stability(10.0e-6, 4.7e-6, 2.2, 24.0, 20.0);
        assert!(mb_ok && mb_db >= 6.0);

        let (esd_ok, v_clamp) = verify_iec_esd_and_surge_tvs_clamping(8.0, 5.5, 0.25, 20.0);
        assert!(esd_ok && v_clamp < 18.0);

        let (f_cav, se_db) = compute_enclosure_cavity_and_slot_se(0.20, 0.05, 0.15, 0.015, 1.0e9);
        assert!(f_cav > 1.0e9 && se_db > 15.0);

        let (audible, dba) = evaluate_mlcc_piezo_acoustic_risk_dba(2_500.0, 1.8, true);
        assert!(audible && dba > 32.0);

        // Fault tree: Top = A OR (A AND B) OR (B AND C) => minimal cut sets are {A} and {B, C}!
        let tree = FaultTreeNode::OrGate(vec![
            FaultTreeNode::BasicEvent("A".into()),
            FaultTreeNode::AndGate(vec![
                FaultTreeNode::BasicEvent("A".into()),
                FaultTreeNode::BasicEvent("B".into()),
            ]),
            FaultTreeNode::AndGate(vec![
                FaultTreeNode::BasicEvent("B".into()),
                FaultTreeNode::BasicEvent("C".into()),
            ]),
        ]);
        let (cuts, p_top) = solve_fault_tree_minimal_cut_sets(
            &tree,
            &[("A".into(), 1e-4), ("B".into(), 1e-3), ("C".into(), 1e-3)],
        );
        assert_eq!(cuts.len(), 2);
        assert!((p_top - 1.01e-4).abs() < 1e-6);

        let (_, eva, rss) = compute_wcca_eva_and_rss(3.3, &[(1.0, 0.03), (1.0, 0.04)]);
        assert!((eva - 0.07).abs() < 1e-9 && (rss - 0.05).abs() < 1e-9);

        let (upset, q_fc, _) = evaluate_heavy_ion_let_seu(35.0, 1.5, 120.0, 60.0);
        assert!(upset && q_fc > 500.0);
        assert!(verify_ecss_component_derating(0.60, 0.50, 85.0, 125.0));
    }
}
