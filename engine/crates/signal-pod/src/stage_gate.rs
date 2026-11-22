// Module: engine/crates/signal-pod/src/stage_gate.rs — Deterministic Hardware Lifecycle Stage-Gate Evaluator (FEAT-098)
// Invariants: Enforces progressive EVT -> DVT -> PVT -> TAPEOUT engineering & cryptographic readiness gates.
// Spec: architecture/17-extended-feature-specifications-ui-and-rust.md, architecture/21-collaboration-space-project-status-and-automated-reviews.md
use signal_proto::v1::{
    EvaluateStageGateChecklistRequest, EvaluateStageGateChecklistResponse, HardwareLifecycleStage,
    StageGateCriterionResult,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StageGateEvidenceInput {
    pub erc_critical_count: u32,
    pub bom_mpn_coverage_pct: f64,
    pub spice_smoke_passed: bool,
    pub drc_lvs_violations: u32,
    pub min_soa_thermal_margin_k: f64,
    pub emc_precompliance_passed: bool,
    pub monte_carlo_cpk: f64,
    pub authorized_provenance_pct: f64,
    pub gold_attested_parts_pct: f64,
    pub super_commit_merkle_signed: bool,
    pub dual_factor_part11_signed: bool,
}

fn criterion(id: &str, label: &str, passed: bool, evidence: String) -> StageGateCriterionResult {
    StageGateCriterionResult {
        criterion_id: id.to_string(),
        label: label.to_string(),
        passed,
        evidence_summary: evidence,
    }
}

/// Evaluates deterministic hardware lifecycle gates (`EVT`, `DVT`, `PVT`, `TAPEOUT`) (`FEAT-098`).
pub fn evaluate_stage_gate_checklist(
    req: &EvaluateStageGateChecklistRequest,
    ev: &StageGateEvidenceInput,
) -> EvaluateStageGateChecklistResponse {
    let stage = HardwareLifecycleStage::try_from(req.stage).unwrap_or(HardwareLifecycleStage::Evt);
    let mut criteria = vec![
        criterion(
            "EVT-01",
            "Schematic ERC Zero Critical Errors",
            ev.erc_critical_count == 0,
            format!("erc_critical_count={}", ev.erc_critical_count),
        ),
        criterion(
            "EVT-02",
            "Orderable MPN BOM Coverage >= 98%",
            ev.bom_mpn_coverage_pct >= 98.0,
            format!("bom_mpn_coverage_pct={:.1}%", ev.bom_mpn_coverage_pct),
        ),
        criterion(
            "EVT-03",
            "SPICE Operating-Point & Smoke Verification",
            ev.spice_smoke_passed,
            format!("spice_smoke_passed={}", ev.spice_smoke_passed),
        ),
    ];

    if matches!(
        stage,
        HardwareLifecycleStage::Dvt | HardwareLifecycleStage::Pvt | HardwareLifecycleStage::Tapeout
    ) {
        criteria.push(criterion(
            "DVT-01",
            "PCB/IC Layout DRC & LVS Clean (0 Violations)",
            ev.drc_lvs_violations == 0,
            format!("drc_lvs_violations={}", ev.drc_lvs_violations),
        ));
        criteria.push(criterion(
            "DVT-02",
            "SOA Junction Thermal Margin >= 10 K",
            ev.min_soa_thermal_margin_k >= 10.0,
            format!(
                "min_soa_thermal_margin_k={:.1} K",
                ev.min_soa_thermal_margin_k
            ),
        ));
        criteria.push(criterion(
            "DVT-03",
            "Power Integrity & EMC Pre-Compliance",
            ev.emc_precompliance_passed,
            format!("emc_precompliance_passed={}", ev.emc_precompliance_passed),
        ));
    }

    if matches!(
        stage,
        HardwareLifecycleStage::Pvt | HardwareLifecycleStage::Tapeout
    ) {
        criteria.push(criterion(
            "PVT-01",
            "Monte Carlo Manufacturing Yield Cpk >= 1.33",
            ev.monte_carlo_cpk >= 1.33,
            format!("cpk={:.2}", ev.monte_carlo_cpk),
        ));
        criteria.push(criterion(
            "PVT-02",
            "Authorized Franchise Distributor Provenance 100%",
            ev.authorized_provenance_pct >= 100.0 - 1.0e-6,
            format!("authorized_provenance={:.1}%", ev.authorized_provenance_pct),
        ));
    }

    if stage == HardwareLifecycleStage::Tapeout {
        criteria.push(criterion(
            "TO-01",
            "100% GOLD Attested Universal Parts & Padstacks",
            ev.gold_attested_parts_pct >= 100.0 - 1.0e-6,
            format!("gold_parts={:.1}%", ev.gold_attested_parts_pct),
        ));
        criteria.push(criterion(
            "TO-02",
            "Signed Super-Commit Merkle Lock & Dual-Factor Attestation",
            ev.super_commit_merkle_signed && ev.dual_factor_part11_signed,
            format!(
                "super_commit={} part11={}",
                ev.super_commit_merkle_signed, ev.dual_factor_part11_signed
            ),
        ));
    }

    let gate_ready = !criteria.is_empty() && criteria.iter().all(|c| c.passed);
    EvaluateStageGateChecklistResponse {
        gate_ready,
        criteria,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stage_gate_evaluator_enforces_progressive_evt_dvt_pvt_tapeout_criteria() {
        let mut ev = StageGateEvidenceInput {
            erc_critical_count: 0,
            bom_mpn_coverage_pct: 100.0,
            spice_smoke_passed: true,
            drc_lvs_violations: 0,
            min_soa_thermal_margin_k: 18.5,
            emc_precompliance_passed: true,
            monte_carlo_cpk: 1.10, // Below PVT 1.33!
            authorized_provenance_pct: 100.0,
            gold_attested_parts_pct: 100.0,
            super_commit_merkle_signed: true,
            dual_factor_part11_signed: true,
        };
        let dvt = evaluate_stage_gate_checklist(
            &EvaluateStageGateChecklistRequest {
                project_id: "p1".into(),
                super_commit_id: "sc1".into(),
                stage: HardwareLifecycleStage::Dvt as i32,
            },
            &ev,
        );
        assert!(dvt.gate_ready);

        let pvt_fail = evaluate_stage_gate_checklist(
            &EvaluateStageGateChecklistRequest {
                project_id: "p1".into(),
                super_commit_id: "sc1".into(),
                stage: HardwareLifecycleStage::Pvt as i32,
            },
            &ev,
        );
        assert!(!pvt_fail.gate_ready);

        ev.monte_carlo_cpk = 1.67;
        let to_ok = evaluate_stage_gate_checklist(
            &EvaluateStageGateChecklistRequest {
                project_id: "p1".into(),
                super_commit_id: "sc1".into(),
                stage: HardwareLifecycleStage::Tapeout as i32,
            },
            &ev,
        );
        assert!(to_ok.gate_ready);
        assert_eq!(to_ok.criteria.len(), 10);
    }
}
