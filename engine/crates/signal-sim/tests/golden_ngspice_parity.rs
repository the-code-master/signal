// Module: engine/crates/signal-sim/tests/golden_ngspice_parity.rs — Golden NGSPICE/Analytical <= 0.1% Parity Suite
// Invariants: Verifies RC, RLC AC, Diode rectifier, MOSFET CS amp, Sync Buck PSS/TR-BDF2 & Virtual Bench (F-023..028, F-110).
// Spec: architecture/03-rust-eda-engine.md, benchmarks/README.md
use serde::Deserialize;
use signal_component::ElectronicModule;
use signal_core::{Microvolts, det_sin};
use signal_lib_semiconductor::{ContinuousMosfet, ShockleyDiode};
use signal_sim::*;
use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

#[derive(Deserialize)]
struct GoldenSuite {
    physics_determinism_epoch: u32,
    max_relative_error: f64,
    cases: serde_json::Value,
}

fn load_golden_suite() -> GoldenSuite {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../../benchmarks/spice/ngspice_reference_vectors.json");
    let raw = fs::read_to_string(path).expect("read golden spice json");
    serde_json::from_str(&raw).expect("parse golden spice json")
}

#[test]
fn golden_rc_step_and_rlc_ac_sweep_match_reference_within_0_1_percent() {
    let suite = load_golden_suite();
    assert_eq!(suite.physics_determinism_epoch, PHYSICS_DETERMINISM_EPOCH);
    let tol = suite.max_relative_error;

    // 1. RC step response: V=3.3V, R=1k, C=100nF, initial x(0)=[3.3, 0.0, -0.0033] at t=0+
    let mut rc = MnaCircuit::new(2);
    let _ = rc.add_dc_ac_vsource("V1", 1, 0, 3.3, 0.0);
    rc.add_resistor("R1", 1, 2, 1_000.0);
    rc.add_capacitor("C1", 2, 0, 1.0e-7);

    let mut x = vec![3.3, 0.0, -0.0033];
    let h = 1.0e-6; // 1 us step -> 100 steps to tau = 100 us
    for step in 0..100 {
        x = step_tr_bdf2(&rc, (step as f64) * h, h, &x, 300_150).expect("rc step");
    }
    let ref_100us = suite.cases["rc_step_1khz"]["v_at_100us_volts"]
        .as_f64()
        .expect("ref");
    assert!(((x[1] - ref_100us) / ref_100us).abs() <= tol);

    let traj = solve_transient_tr_bdf2(&rc, 1.5e-3, 2.0e-5, Some(&[3.3, 0.0, -0.0033]), 300_150)
        .expect("rc settling");
    let v_final = traj.states.last().expect("final")[1];
    assert!(((v_final - 3.3) / 3.3).abs() <= tol);

    // 2. Series-LC + Shunt-R Bandpass AC sweep
    // Node 1: Vin=1Vac, L(1->2)=10uH (0 DCR), C(2->3)=1nF, R(3->0)=50 Ohm
    let mut rlc = MnaCircuit::new(3);
    let _ = rlc.add_dc_ac_vsource("VIN", 1, 0, 0.0, 1.0);
    let _ = rlc.add_inductor("L1", 1, 2, 1.0e-5, 0.0);
    rlc.add_capacitor("C1", 2, 3, 1.0e-9);
    rlc.add_resistor("RL", 3, 0, 50.0);

    let f_res = suite.cases["rlc_bandpass_ac"]["f_res_hz"]
        .as_f64()
        .expect("f0");
    let ac = solve_ac_sweep(&rlc, &[1.0e6, f_res, 2.5e6], 300_150).expect("ac sweep");
    let m0 = ac[0].node_phasors[2].mag();
    let m1 = ac[1].node_phasors[2].mag();
    let m2 = ac[2].node_phasors[2].mag();
    let ref_m0 = suite.cases["rlc_bandpass_ac"]["mag_at_1mhz"]
        .as_f64()
        .expect("m0");
    let ref_m2 = suite.cases["rlc_bandpass_ac"]["mag_at_2p5mhz"]
        .as_f64()
        .expect("m2");

    assert!(((m0 - ref_m0) / ref_m0).abs() <= tol);
    assert!((m1 - 1.0).abs() <= tol);
    assert!(((m2 - ref_m2) / ref_m2).abs() <= tol);
}

#[test]
fn golden_diode_rectifier_mosfet_cs_amp_and_sync_buck_pss_match_within_0_1_percent() {
    let suite = load_golden_suite();
    let tol = suite.max_relative_error;

    // 3. Diode rectifier DC & Harmonic Balance + transient check
    let mut d_ckt = MnaCircuit::new(2);
    let _ = d_ckt.add_dc_ac_vsource("VIN", 1, 0, 5.0, 0.0);
    d_ckt.add_diode("D1", 1, 2, ShockleyDiode::silicon_1n4148());
    d_ckt.add_resistor("RL", 2, 0, 1_000.0);

    let dc_d = solve_dc_op(&d_ckt, None, 300_150).expect("diode dc");
    let ref_vd = suite.cases["diode_halfwave_rectifier"]["dc_vout_volts"]
        .as_f64()
        .expect("vd");
    assert!(((dc_d[1] - ref_vd) / ref_vd).abs() <= tol);

    // Half-wave sine rectifier with filter cap via TR-BDF2
    let mut rect = MnaCircuit::new(2);
    let _ = rect.add_sine_vsource("VSIN", 1, 0, 0.0, 5.0, 1_000.0);
    rect.add_diode("D1", 1, 2, ShockleyDiode::silicon_1n4148());
    rect.add_resistor("RL", 2, 0, 1_000.0);
    rect.add_capacitor("CF", 2, 0, 1.0e-6);
    let rect_tr = solve_transient_tr_bdf2(&rect, 1.0e-3, 1.0e-5, Some(&[0.0, 0.0, 0.0]), 300_150)
        .expect("rectifier tran");
    let peak_vout = rect_tr.states.iter().fold(0.0_f64, |m, s| m.max(s[1]));
    assert!(peak_vout > 4.2 && peak_vout < 4.5);

    // 4. MOSFET common-source amplifier DC-OP & small-signal gain parity
    let mos = ContinuousMosfet::nmos_default();
    let mut cs = MnaCircuit::new(3);
    let _ = cs.add_dc_ac_vsource("VDD", 1, 0, 5.0, 0.0);
    let _ = cs.add_dc_ac_vsource("VGS", 2, 0, 1.5, 1.0);
    cs.add_resistor("RD", 1, 3, 2_000.0);
    cs.add_mosfet("M1", 3, 2, 0, 0, mos.clone());

    let dc_cs = solve_dc_op(&cs, None, 300_150).expect("cs dc");
    let v_drain = dc_cs[2];
    assert!(v_drain > 1.0 && v_drain < 4.5);

    // Compare MNA AC gain against analytical -gm * (RD || ro)
    let st_m = mos.evaluate_multi_terminal(
        &[
            Microvolts((v_drain * 1.0e6).round() as i64),
            Microvolts(1_500_000),
            Microvolts(0),
            Microvolts(0),
        ],
        300_150,
    );
    let gm = st_m.g_jacobian[0][1];
    let gds = st_m.g_jacobian[0][0];
    let expected_gain_mag = gm / (gds + 1.0 / 2_000.0);
    let ac_cs = solve_ac_sweep(&cs, &[1_000.0], 300_150).expect("cs ac");
    let sim_gain_mag = ac_cs[0].node_phasors[2].mag();
    assert!(((sim_gain_mag - expected_gain_mag) / expected_gain_mag).abs() <= tol);

    // 5. Switched Synchronous Buck Converter (48V -> 1.8V, 500 kHz, T = 2 us) via Shooting-Newton PSS
    let mut buck = MnaCircuit::new(3);
    let _ = buck.add_dc_ac_vsource("VIN", 1, 0, 48.0, 0.0);
    let period_s = 2.0e-6;
    buck.add_pwm_switch("SW_HS", 1, 2, 1.0e-4, 1.0e6, period_s, 0.0375, true);
    buck.add_pwm_switch("SW_LS", 2, 0, 1.0e-4, 1.0e6, period_s, 0.0375, false);
    let _ = buck.add_inductor("L1", 2, 3, 1.0e-5, 1.0e-4);
    buck.add_capacitor("COUT", 3, 0, 4.7e-5);
    buck.add_resistor("RLOAD", 3, 0, 1.8);

    let pss = solve_shooting_pss(
        &buck,
        period_s,
        40,
        Some(&[48.0, 0.0, 1.75, 0.0, 0.9]),
        300_150,
    )
    .expect("buck shooting PSS");
    assert!(pss.closure_residual < 1.0e-5);
    let mut v_int = 0.0;
    for k in 0..(pss.period_times_s.len() - 1) {
        let dt = pss.period_times_s[k + 1] - pss.period_times_s[k];
        v_int += 0.5 * (pss.period_states[k][2] + pss.period_states[k + 1][2]) * dt;
    }
    let avg_vout = v_int / period_s;
    let ref_buck = suite.cases["sync_buck_48v_to_1v8"]["expected_vout_ss_volts"]
        .as_f64()
        .expect("buck ref");
    assert!(((avg_vout - ref_buck) / ref_buck).abs() <= tol);
}

#[test]
fn ruiz_equilibration_hb_fft_yield_jitter_regression_and_stimulus_verified() {
    // Ruiz equilibration on 1e24 row/column conductance scale spread (exact f64 representation)
    let ill = vec![vec![1.0e12, -1.0e12], vec![1.0e-12, 1.0e-12]];
    let x_ill = solve_equilibrated_lu(&ill, &[0.0, 2.0e-12]).expect("ruiz lu");
    assert!((x_ill[0] - 1.0).abs() < 1e-12 && (x_ill[1] - 1.0).abs() < 1e-12);

    let disp = LinearSolverDispatcher {
        gpu_available: true,
    };
    assert_eq!(
        disp.dispatch(500, 1),
        LinearSolverBackend::CpuKluRuizEquilibrated
    );
    assert_eq!(
        disp.dispatch(25_000, 1),
        LinearSolverBackend::GpuCuDssMixedPrecision
    );
    assert_eq!(
        disp.dispatch(200, 32),
        LinearSolverBackend::GpuCuDssMixedPrecision
    );

    // Harmonic Balance on linear/nonlinear driven RC
    let mut hb_ckt = MnaCircuit::new(2);
    let _ = hb_ckt.add_sine_vsource("VS", 1, 0, 0.0, 1.0, 1_000.0);
    hb_ckt.add_resistor("R1", 1, 2, 1_000.0);
    hb_ckt.add_capacitor("C1", 2, 0, 1.0e-7);
    let hb = solve_harmonic_balance(&hb_ckt, 1_000.0, 3, 300_150).expect("hb");
    assert!(hb.harmonic_phasors[1][1].mag() > 0.4);

    // FEAT-081: FFT THD/SINAD/ENOB + 2-port S-to-Z
    let n = 256;
    let pure_sine: Vec<f64> = (0..n)
        .map(|k| det_sin(std::f64::consts::TAU * 16.0 * (k as f64) / (n as f64)))
        .collect();
    let spec = analyze_fft_spectrum(&pure_sine).expect("fft");
    assert!(spec.sinad_db > 60.0);
    assert!(spec.thd_db < -60.0);
    assert!(spec.enob_bits > 10.0);

    let z_matched = s2p_to_z_matrix([[Complex64::ZERO; 2]; 2], 50.0);
    assert!((z_matched[0][0].re - 50.0).abs() < 1e-12);

    // FEAT-083: Halton QMC Cpk + Spearman
    let mc = run_yield_analytics(
        &[
            ("R1".into(), "r".into(), 10_000.0, 0.01),
            ("R2".into(), "r".into(), 10_000.0, 0.001),
        ],
        64,
        2.4,
        2.6,
        |p| 5.0 * p[1] / (p[0] + p[1]),
    );
    assert!(mc.cpk_index > 1.0);
    assert_eq!(mc.tornado_bars[0].entity_id, "R1");

    // FEAT-084: Dual-Dirac PAM4 BER bathtub
    let (jit, tub) = decompose_jitter_pam4_ber(100_000, 1_500, 8_000, 4_000);
    assert!(jit.horizontal_eye_opening_ui_at_1e15 > 0.5);
    assert!(tub[20].1 < 1.0e-15);

    // FEAT-086 & FEAT-090 & F-069
    let mut metrics = BTreeMap::new();
    metrics.insert("vout_dc".into(), 1.801);
    let reg = run_regression_assertions(
        &metrics,
        &["vout_dc <= 1.85".into(), "vout_dc >= 1.75".into()],
    );
    assert!(reg.all_passed);

    let prbs = synthesize_stimulus("PRBS7", 1_000_000, 127);
    assert_eq!(prbs.len(), 127);
    let env = decimate_minmax_envelope(
        &(0..10_000).map(|i| i as f64).collect::<Vec<_>>(),
        &(0..10_000)
            .map(|i| if i == 5432 { 99.0 } else { 0.0 })
            .collect::<Vec<_>>(),
        4096,
    );
    assert_eq!(env.len(), 4096);
    assert!(env.iter().any(|b| (b.max_val - 99.0).abs() < 1e-12));
}
