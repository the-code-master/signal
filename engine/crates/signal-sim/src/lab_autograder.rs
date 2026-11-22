// Module: engine/crates/signal-sim/src/lab_autograder.rs — Headless Seed-Randomized Circuit Lab Auto-Grader & LTI 1.3 Signer (FEAT-112)
// Invariants: Derives anti-cheat student targets deterministically from randomized_rubric_seed; signs LTI 1.3 receipt with HybridPqcSignature.
// Spec: architecture/14-universal-scale-accessibility-and-expandability.md, architecture/17-extended-feature-specifications-ui-and-rust.md
use signal_core::{HybridSignature, SecretBox, hmac_sha512, sha512_256_hex};
use signal_proto::v1::{GradeAcademyLabRequest, GradeAcademyLabResponse, HybridPqcSignature};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StudentCircuitSubmission {
    pub r_top_ohm: f64,
    pub r_bot_ohm: f64,
    pub filter_cap_f: f64,
}

/// Derives the student-specific target output ratio $V_{\text{out}}/V_{\text{in}} \in [0.20, 0.80]$ from `randomized_rubric_seed` (`FEAT-112`).
pub fn derive_seeded_target_divider_ratio(seed: u64, student_pseudo_id: &str) -> f64 {
    let mac = hmac_sha512(&seed.to_le_bytes(), student_pseudo_id.as_bytes());
    let u = (u32::from_be_bytes([mac[0], mac[1], mac[2], mac[3]]) as f64) / (u32::MAX as f64);
    0.20 + 0.60 * u
}

/// Evaluates a student circuit across nominal + 4 injected fault/corner rubrics and emits a signed LTI 1.3 receipt (`FEAT-112`).
pub fn grade_academy_circuit_lab(
    req: &GradeAcademyLabRequest,
    submission: StudentCircuitSubmission,
    grader_ed25519_seed: &SecretBox<[u8; 32]>,
    grader_mldsa_seed: &SecretBox<[u8; 32]>,
    graded_at_utc_ms: i64,
) -> GradeAcademyLabResponse {
    let target_ratio =
        derive_seeded_target_divider_ratio(req.randomized_rubric_seed, &req.student_pseudo_id);
    let r_top = submission.r_top_ohm.max(1.0);
    let r_bot = submission.r_bot_ohm.max(1.0);
    let total_r = r_top + r_bot;

    let mut passed = 0_u32;
    let total_rubrics = 5_u32;

    // Rubric 1: Nominal unloaded DC transfer ratio within 2.5%
    let unloaded_ratio = r_bot / total_r;
    if (unloaded_ratio - target_ratio).abs() <= 0.025 {
        passed += 1;
    }

    // Rubric 2: Quiescent current budget (1 kOhm <= R_top + R_bot <= 250 kOhm)
    if (1_000.0..=250_000.0).contains(&total_r) {
        passed += 1;
    }

    // Rubric 3: Injected 1 MOhm oscilloscope/ADC load droop < 3.5%
    let r_bot_loaded = (r_bot * 1_000_000.0) / (r_bot + 1_000_000.0);
    let loaded_ratio = r_bot_loaded / (r_top + r_bot_loaded);
    if (loaded_ratio - target_ratio).abs() <= 0.035 {
        passed += 1;
    }

    // Rubric 4: Injected +-5% resistor lot skew worst-case stay within +-6% of target
    let skew_ratio = (r_bot * 0.95) / (r_top * 1.05 + r_bot * 0.95);
    if (skew_ratio - target_ratio).abs() <= 0.060 {
        passed += 1;
    }

    // Rubric 5: Anti-aliasing RC pole f_c = 1 / (2*pi*R_th*C) between 100 Hz and 250 kHz
    let r_th = (r_top * r_bot) / total_r;
    let fc_hz = 1.0 / (std::f64::consts::TAU * r_th * submission.filter_cap_f.max(1e-15));
    if (100.0..=250_000.0).contains(&fc_hz) {
        passed += 1;
    }

    let score_pct = (f64::from(passed) / f64::from(total_rubrics)) * 100.0;
    let receipt_payload = format!(
        "lti13-ags:v1|lab={}|student={}|branch={}|seed={}|score={score_pct:.2}",
        req.lab_assignment_id,
        req.student_pseudo_id,
        req.submission_branch_id,
        req.randomized_rubric_seed
    );
    let receipt_id = format!(
        "lti13_rcpt_{}",
        &sha512_256_hex(receipt_payload.as_bytes())[..16]
    );
    let sig = HybridSignature::sign_deterministic(
        grader_ed25519_seed,
        grader_mldsa_seed,
        receipt_payload.as_bytes(),
    );

    GradeAcademyLabResponse {
        score_pct,
        passed_fault_rubrics: passed,
        total_fault_rubrics: total_rubrics,
        lti13_grade_passback_receipt: receipt_id,
        grader_signature: Some(HybridPqcSignature {
            ed25519_sig: sig.ed25519_sig.to_vec(),
            ml_dsa_65_sig: sig.ml_dsa_65_sig,
            key_id: "signal-academy-lti13-grader-v1".into(),
            signed_at_utc_ms: graded_at_utc_ms,
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grades_seeded_student_divider_across_fault_rubrics_and_signs_lti13_receipt() {
        let req = GradeAcademyLabRequest {
            lab_assignment_id: "EE101-LAB-02".into(),
            student_pseudo_id: "stu_98a1f".into(),
            submission_branch_id: "sub-01".into(),
            randomized_rubric_seed: 9_007_199_254_740_995,
        };
        let target =
            derive_seeded_target_divider_ratio(req.randomized_rubric_seed, &req.student_pseudo_id);
        let r_total = 20_000.0;
        let r_bot = r_total * target;
        let r_top = r_total - r_bot;

        let resp = grade_academy_circuit_lab(
            &req,
            StudentCircuitSubmission {
                r_top_ohm: r_top,
                r_bot_ohm: r_bot,
                filter_cap_f: 10e-9,
            },
            &SecretBox::new([0x21; 32]),
            &SecretBox::new([0x22; 32]),
            1_795_326_000_000,
        );
        assert_eq!(resp.passed_fault_rubrics, 5);
        assert!((resp.score_pct - 100.0).abs() < 1e-9);
        assert!(resp.lti13_grade_passback_receipt.starts_with("lti13_rcpt_"));
        assert!(resp.grader_signature.is_some());
    }
}
