// Module: engine/crates/signal-sim/tests/phase3_anomaly_attribution.rs — FEAT-106 Waveform Anomaly Attribution Tests
// Invariants: Verifies resonant LC ringing frequency/zeta extraction and culprit loop inductance ranking.
// Spec: architecture/03-rust-eda-engine.md, architecture/17-extended-feature-specifications-ui-and-rust.md

use signal_core::{det_cos, det_exp};
use signal_proto::v1::AttributeWaveformAnomalyRequest;
use signal_sim::{ParasiticLoopCandidate, attribute_waveform_anomaly};

#[test]
fn attributes_100mhz_underdamped_switch_node_ringing_to_exact_lc_culprit() {
    // Synthesize 100 MHz damped ringing around 12 V (L = 2.533 nH, C = 1000 pF => f0 ~= 100 MHz)
    let f0 = 100.0e6_f64;
    let mut samples = Vec::new();
    for k in 0..=200 {
        let t_ps = k * 250; // 0..50 ns in 250 ps steps
        let t_s = (t_ps as f64) * 1.0e-12;
        let env = 3.5 * det_exp(-3.5e7 * t_s);
        let v = 12.0 + env * det_cos(2.0 * std::f64::consts::PI * f0 * t_s);
        samples.push((t_ps, (v * 1_000_000.0).round() as i64));
    }

    let cands = vec![
        ParasiticLoopCandidate {
            entity_id: "C_BULK_ELECTROLYTIC".into(),
            loop_inductance_nh: 25.0,
            parasitic_cap_pf: 100_000.0, // ~3.18 MHz
            pdn_esr_mohm: 40.0,
        },
        ParasiticLoopCandidate {
            entity_id: "Q1_SW_HOT_LOOP".into(),
            loop_inductance_nh: 2.53,
            parasitic_cap_pf: 1_000.0, // ~100.06 MHz!
            pdn_esr_mohm: 8.0,
        },
    ];

    let resp = attribute_waveform_anomaly(
        &AttributeWaveformAnomalyRequest {
            simulation_run_id: "sim-ring-1".into(),
            target_net_name: "SW_NODE".into(),
            window_start_ps: 0,
            window_end_ps: 50_000,
        },
        &samples,
        &cands,
    );
    assert_eq!(resp.culprit_entity_id, "Q1_SW_HOT_LOOP");
    assert!(
        resp.dominant_root_cause
            .contains("UNDERDAMPED_LC_PARASITIC_RINGING")
    );
    assert!(resp.ranked_mitigations.len() >= 2);
}
