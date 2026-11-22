// Module: engine/crates/signal-formal/src/lib.rs — SAT/IC3 Formal Prover, 200-Pt Red-Team & Constraint AST (I-03)
// Invariants: #![forbid(unsafe_code)]; signs ProofBundle with HybridSignature bound to base_graph_merkle_root (F-133).
// Spec: architecture/03-rust-eda-engine.md, architecture/05-ai-agent-and-datasheet-rag.md, architecture/16-*
#![forbid(unsafe_code)]

use signal_core::{HybridSignature, SecretBox};
use signal_proto::v1::{HybridPqcSignature, ProofBundle};
use thiserror::Error;

pub mod constraint_ast;
pub mod redteam_deck;
pub mod sat_ic3;

pub use constraint_ast::{ConstraintRuleAst, compile_nl_constraint_rule};
pub use redteam_deck::{
    DesignRedTeamSnapshot, REDTEAM_TOTAL_CHECK_COUNT, run_principal_redteam_audit,
};
pub use sat_ic3::{
    CnfSolver, FiniteTransitionSystem, SatOutcome, prove_miter_equivalence,
    verify_fsm_cdc_and_safety_ic3,
};

#[derive(Debug, Error, PartialEq, Eq)]
pub enum FormalError {
    #[error("invalid formal constraint rule: {0}")]
    InvalidConstraintRule(String),
}

/// Constructs a cryptographically signed `ProofBundle` bound to `base_graph_merkle_root` & `physics_determinism_epoch` (`F-133`).
pub fn seal_verified_proof_bundle(
    proof_bundle_id: &str,
    base_graph_merkle_root: &str,
    physics_determinism_epoch: u32,
    spice_assertions_passed: bool,
    formal_properties_unsat: bool,
    drc_erc_clean: bool,
    ed25519_seed: &SecretBox<[u8; 32]>,
) -> ProofBundle {
    let ml_dsa_seed = SecretBox::new(*ed25519_seed.expose_secret());
    let transcript = format!(
        "PROOF|{proof_bundle_id}|{base_graph_merkle_root}|{physics_determinism_epoch}|{spice_assertions_passed}|{formal_properties_unsat}|{drc_erc_clean}"
    );
    let sig =
        HybridSignature::sign_deterministic(ed25519_seed, &ml_dsa_seed, transcript.as_bytes());
    ProofBundle {
        proof_bundle_id: proof_bundle_id.to_string(),
        base_graph_merkle_root: base_graph_merkle_root.to_string(),
        physics_determinism_epoch,
        spice_assertions_passed,
        formal_properties_unsat,
        drc_erc_clean,
        verifier_signature: Some(HybridPqcSignature {
            ed25519_sig: sig.ed25519_sig.to_vec(),
            ml_dsa_65_sig: sig.ml_dsa_65_sig.to_vec(),
            key_id: "spiffe://signal.design/verifier/signal-formal".to_string(),
            signed_at_utc_ms: 1_763_770_000_000,
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use signal_proto::v1::CompileNlConstraintRuleRequest;
    use std::collections::{BTreeMap, BTreeSet};

    #[test]
    fn proves_demorgan_miter_equivalence_and_catches_buggy_gate_netlist() {
        let mut s = CnfSolver::new();
        let a = s.fresh_var();
        let b = s.fresh_var();
        // RTL: !(a & b)
        let ab = s.encode_and(a, b);
        let rtl_out = s.encode_not(ab);
        // Gate: (!a) | (!b)
        let na = s.encode_not(a);
        let nb = s.encode_not(b);
        let gate_out = s.encode_or(na, nb);

        let (eq, cex) = prove_miter_equivalence(s.clone(), &[(rtl_out, gate_out)]);
        assert!(eq && cex.is_none());

        // Buggy gate: (!a) & (!b) != !(a & b)
        let buggy_out = s.encode_and(na, nb);
        let (eq_buggy, cex_buggy) = prove_miter_equivalence(s, &[(rtl_out, buggy_out)]);
        assert!(!eq_buggy && cex_buggy.is_some());
    }

    #[test]
    fn ic3_proves_fsm_deadlock_freedom_gray_cdc_and_power_rail_order() {
        let sys_ok = FiniteTransitionSystem {
            initial_states: BTreeSet::from([0]),
            transitions: BTreeMap::from([(0, vec![1]), (1, vec![2]), (2, vec![0])]),
            bad_states: BTreeSet::from([99]),
            cdc_crossing_codes: vec![0b00, 0b01, 0b11, 0b10],
            cdc_has_two_ff_sync: true,
            power_rail_up_order: vec!["VDD_CORE_0V8".into(), "VDD_IO_3V3".into()],
        };
        let rep = verify_fsm_cdc_and_safety_ic3("prf-1", true, &sys_ok, 16);
        assert!(rep.logically_equivalent && rep.fsm_deadlock_free && rep.cdc_rdc_safe);

        let mut sys_bad = sys_ok;
        sys_bad.transitions.insert(2, Vec::new()); // Deadlock at state 2!
        sys_bad.cdc_crossing_codes = vec![0b00, 0b11]; // 2-bit jump!
        let rep_bad = verify_fsm_cdc_and_safety_ic3("prf-2", true, &sys_bad, 16);
        assert!(!rep_bad.fsm_deadlock_free);
        assert!(!rep_bad.cdc_rdc_safe);
    }

    #[test]
    fn evaluates_200_point_redteam_deck_and_compiles_drc_sdc_upf_sva_rules() {
        let mut snap = DesignRedTeamSnapshot::default();
        snap.ic_missing_local_decoupling.push("U4".into());
        snap.unsynchronized_cdc_signals.push("spi_rx_rdy".into());
        let audit = run_principal_redteam_audit(&snap);
        assert_eq!(audit.checks_evaluated, 200);
        assert_eq!(audit.findings.len(), 2);

        let (_, sdc) = compile_nl_constraint_rule(&CompileNlConstraintRuleRequest {
            natural_language_prompt: "Constrain clock clk_pcie at 250MHz with 15ps uncertainty"
                .into(),
            target_domain: "SDC".into(),
        })
        .expect("sdc");
        assert!(sdc.deterministic_ast_valid && sdc.compiled_rule_dsl.contains("4000"));

        let seed = SecretBox::new([0x33_u8; 32]);
        let bundle = seal_verified_proof_bundle(
            "pb-1",
            "aa".repeat(32).as_str(),
            1,
            true,
            true,
            true,
            &seed,
        );
        assert!(bundle.formal_properties_unsat && bundle.verifier_signature.is_some());
    }
}
