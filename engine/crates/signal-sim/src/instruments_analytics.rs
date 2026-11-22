// Module: engine/crates/signal-sim/src/instruments_analytics.rs — Unified Virtual Bench & Statistical Analytics Facade
// Invariants: Re-exports FEAT-003, FEAT-004, FEAT-081..084, FEAT-086, FEAT-090 & F-069 waveform decimator.
// Spec: architecture/03-rust-eda-engine.md, architecture/17-extended-feature-specifications-ui-and-rust.md
pub use crate::analytics::{
    decompose_jitter_pam4_ber, det_erfc_positive, halton_radical_inverse,
    run_regression_assertions, run_yield_analytics, synthesize_stimulus,
};
pub use crate::instruments::{
    EnvelopeBucket, FftSpectrumMetrics, analyze_fft_spectrum, compute_branch_telemetry,
    decimate_minmax_envelope, eval_wave_expr, interactive_slider_step, s2p_to_z_matrix,
};
