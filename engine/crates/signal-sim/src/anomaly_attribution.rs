// Module: engine/crates/signal-sim/src/anomaly_attribution.rs — Ringing, Overshoot & Droop Root-Cause Ranker (FEAT-106)
// Invariants: Extracts damping ratio zeta & resonant f_0 from waveform extrema and attributes culprit L_loop*C_oss / PDN ESR.
// Spec: architecture/03-rust-eda-engine.md, architecture/17-extended-feature-specifications-ui-and-rust.md

use signal_core::det_ln;
use signal_proto::v1::{AttributeWaveformAnomalyRequest, AttributeWaveformAnomalyResponse};

#[derive(Debug, Clone, PartialEq)]
pub struct ParasiticLoopCandidate {
    pub entity_id: String,
    pub loop_inductance_nh: f64,
    pub parasitic_cap_pf: f64,
    pub pdn_esr_mohm: f64,
}

/// Analyzes transient waveform samples `(time_ps, voltage_uv)` inside `[window_start_ps, window_end_ps]` (`FEAT-106`),
/// computing underdamped ringing $f_0$, damping ratio $\zeta$, or IR/di/dt droop and ranking physical fixes.
pub fn attribute_waveform_anomaly(
    req: &AttributeWaveformAnomalyRequest,
    samples_ps_uv: &[(i64, i64)],
    candidates: &[ParasiticLoopCandidate],
) -> AttributeWaveformAnomalyResponse {
    let win: Vec<(f64, f64)> = samples_ps_uv
        .iter()
        .filter(|&&(t, _)| t >= req.window_start_ps && t <= req.window_end_ps)
        .map(|&(t, v)| ((t as f64) * 1.0e-12, (v as f64) * 1.0e-6))
        .collect();

    if win.len() < 4 || candidates.is_empty() {
        return AttributeWaveformAnomalyResponse {
            dominant_root_cause: "INSUFFICIENT_WAVEFORM_WINDOW".to_string(),
            culprit_entity_id: candidates
                .first()
                .map_or_else(String::new, |c| c.entity_id.clone()),
            ranked_mitigations: Vec::new(),
        };
    }

    let v_steady = win.last().map_or(0.0, |&(_, v)| v);
    let mut peaks: Vec<(f64, f64)> = Vec::new();
    for i in 1..(win.len() - 1) {
        let (t0, v0) = (win[i - 1].1 - v_steady, win[i].1 - v_steady);
        let v2 = win[i + 1].1 - v_steady;
        if v0 > 0.0 && v0 >= t0 && v0 > v2 {
            peaks.push((win[i].0, v0));
        }
    }

    // Underdamped LC parasitic ringing when >= 2 positive overshoot peaks exist
    if peaks.len() >= 2 {
        let t_ring_s = (peaks[1].0 - peaks[0].0).max(1.0e-12);
        let ratio = (peaks[0].1 / peaks[1].1.max(1.0e-9)).max(1.000_1);
        let delta = det_ln(ratio);
        let zeta =
            delta / (4.0 * std::f64::consts::PI * std::f64::consts::PI + delta * delta).sqrt();
        let f0_hz = 1.0 / (t_ring_s * (1.0 - zeta * zeta).max(0.05).sqrt());

        let best = candidates
            .iter()
            .min_by(|a, b| {
                let fa = 1.0
                    / (2.0
                        * std::f64::consts::PI
                        * ((a.loop_inductance_nh * 1e-9) * (a.parasitic_cap_pf * 1e-12))
                            .max(1e-30)
                            .sqrt());
                let fb = 1.0
                    / (2.0
                        * std::f64::consts::PI
                        * ((b.loop_inductance_nh * 1e-9) * (b.parasitic_cap_pf * 1e-12))
                            .max(1e-30)
                            .sqrt());
                let ea = ((fa - f0_hz) / f0_hz).abs();
                let eb = ((fb - f0_hz) / f0_hz).abs();
                ea.total_cmp(&eb)
            })
            .unwrap_or(&candidates[0]);

        let z_char_ohm =
            ((best.loop_inductance_nh * 1e-9) / (best.parasitic_cap_pf * 1e-12).max(1e-18)).sqrt();
        let c_snub_pf = (best.parasitic_cap_pf * 3.0).round();
        return AttributeWaveformAnomalyResponse {
            dominant_root_cause: format!(
                "UNDERDAMPED_LC_PARASITIC_RINGING(f0={:.2}MHz, zeta={zeta:.3})",
                f0_hz * 1.0e-6
            ),
            culprit_entity_id: best.entity_id.clone(),
            ranked_mitigations: vec![
                format!(
                    "Add RC snubber R_snub={z_char_ohm:.1} Ohm + C_snub={c_snub_pf:.0} pF across `{}`",
                    best.entity_id
                ),
                format!(
                    "Tighten hot-loop return plane under `{}` to cut L_loop ({:.1} nH) by >50%",
                    best.entity_id, best.loop_inductance_nh
                ),
                "Increase high-side gate series resistor R_g by 2.2..4.7 Ohm to slew-limit dI/dt"
                    .to_string(),
            ],
        };
    }

    // Supply droop / PDN ESR + ESL transient sag
    let culprit = candidates
        .iter()
        .max_by(|a, b| a.pdn_esr_mohm.total_cmp(&b.pdn_esr_mohm))
        .unwrap_or(&candidates[0]);
    AttributeWaveformAnomalyResponse {
        dominant_root_cause: "PDN_TRANSIENT_IMPEDANCE_DROOP".to_string(),
        culprit_entity_id: culprit.entity_id.clone(),
        ranked_mitigations: vec![
            format!(
                "Parallel low-ESL 0402 X7R MLCCs adjacent to `{}` to lower PDN ESR ({:.1} mOhm)",
                culprit.entity_id, culprit.pdn_esr_mohm
            ),
            "Widen power/ground copper pour and stitch return vias within 1.5 mm".to_string(),
        ],
    }
}
