// Module: engine/crates/signal-component/src/soa_thermal.rs — Electro-Thermal Self-Heating, SOA Watchdog & Fault Injection
// Invariants: Computes Tj = Tamb + P * R_theta_jc; emits SoaFailureData (FEAT-113); supports FMEDA FaultInjectionMode.
// Spec: architecture/03-rust-eda-engine.md, architecture/17-extended-feature-specifications-ui-and-rust.md
use crate::ConservativeStamp;
use signal_core::{Millikelvin, Nanometers};

/// FMEDA / Interactive Fault Injection Modes (`FEAT-003`, `signal-component`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FaultInjectionMode {
    None,
    Open,
    Short,
    ParametricDrift { multiplier: f64 },
    StuckAt { clamped_voltage_v: f64 },
}

/// Safe Operating Area (SOA) & electro-thermal limits for a component instance (`FEAT-113`).
#[derive(Debug, Clone, PartialEq)]
pub struct SoaSpec {
    pub max_abs_voltage_uv: i64,
    pub max_abs_current_na: i64,
    pub max_power_uw: i64,
    pub max_junction_temp_mk: i64,
    pub r_theta_jc_k_per_w: f64,
    pub smoke_anchor_nm: (Nanometers, Nanometers),
    pub recommended_upgrade_mpn: String,
}

impl Default for SoaSpec {
    fn default() -> Self {
        Self {
            max_abs_voltage_uv: 50_000_000,    // 50 V
            max_abs_current_na: 5_000_000_000, // 5 A
            max_power_uw: 250_000,             // 0.25 W
            max_junction_temp_mk: 423_150,     // 150 C
            r_theta_jc_k_per_w: 120.0,
            smoke_anchor_nm: (Nanometers(0), Nanometers(0)),
            recommended_upgrade_mpn: "UPGRADE_RATED_PART".to_string(),
        }
    }
}

/// Emitted by autonomous SOA watchdog when V, I, P, or junction temperature breaches limits (`FEAT-113`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SoaFailureData {
    pub entity_id: String,
    pub failure_mode: String,
    pub peak_voltage_uv: i64,
    pub peak_current_na: i64,
    pub peak_power_uw: i64,
    pub junction_temp_mkelvin: i64,
    pub smoke_anchor_nm: (Nanometers, Nanometers),
    pub recommended_upgrade_mpn: String,
}

/// Computes dynamic steady-state / lumped junction temperature $T_j = T_{\text{amb}} + |V \cdot I| \cdot R_{\theta JC}$.
pub fn compute_junction_temp_mk(
    voltage_volts: f64,
    current_amps: f64,
    ambient_temp_mk: i64,
    r_theta_jc_k_per_w: f64,
) -> Millikelvin {
    let power_w = (voltage_volts * current_amps).abs();
    let rise_mk = (power_w * r_theta_jc_k_per_w * 1_000.0).round() as i64;
    Millikelvin(ambient_temp_mk.saturating_add(rise_mk))
}

/// Evaluates V / I / P / $T_j$ against `SoaSpec`, returning `Some(SoaFailureData)` on violation (`FEAT-113`).
pub fn evaluate_soa_watchdog(
    entity_id: &str,
    voltage_volts: f64,
    current_amps: f64,
    ambient_temp_mk: i64,
    spec: &SoaSpec,
) -> Option<SoaFailureData> {
    let v_uv = (voltage_volts.abs() * 1.0e6).round() as i64;
    let i_na = (current_amps.abs() * 1.0e9).round() as i64;
    let p_uw = ((voltage_volts * current_amps).abs() * 1.0e6).round() as i64;
    let tj_mk = compute_junction_temp_mk(
        voltage_volts,
        current_amps,
        ambient_temp_mk,
        spec.r_theta_jc_k_per_w,
    )
    .0;

    let mode = if v_uv > spec.max_abs_voltage_uv {
        Some("DIELECTRIC_OVERVOLTAGE_BREAKDOWN")
    } else if i_na > spec.max_abs_current_na {
        Some("BONDWIRE_OVERCURRENT_FUSE")
    } else if p_uw > spec.max_power_uw {
        Some("THERMAL_OVERPOWER_SMOKE")
    } else if tj_mk > spec.max_junction_temp_mk {
        Some("JUNCTION_OVERTEMP_RUNAWAY")
    } else {
        None
    }?;

    Some(SoaFailureData {
        entity_id: entity_id.to_string(),
        failure_mode: mode.to_string(),
        peak_voltage_uv: v_uv,
        peak_current_na: i_na,
        peak_power_uw: p_uw,
        junction_temp_mkelvin: tj_mk,
        smoke_anchor_nm: spec.smoke_anchor_nm,
        recommended_upgrade_mpn: spec.recommended_upgrade_mpn.clone(),
    })
}

/// Applies FMEDA `FaultInjectionMode` to a 2-terminal `ConservativeStamp`.
pub fn apply_fault_injection(
    base: ConservativeStamp,
    branch_voltage_v: f64,
    fault: FaultInjectionMode,
) -> ConservativeStamp {
    match fault {
        FaultInjectionMode::None => base,
        FaultInjectionMode::Open => ConservativeStamp {
            current_amps: 1.0e-12 * branch_voltage_v,
            charge_coulombs: 0.0,
            flux_webers: 0.0,
            conductance_siemens: 1.0e-12,
        },
        FaultInjectionMode::Short => {
            let g_short = 1.0e3; // 1 mOhm hard short
            ConservativeStamp {
                current_amps: g_short * branch_voltage_v,
                charge_coulombs: 0.0,
                flux_webers: 0.0,
                conductance_siemens: g_short,
            }
        }
        FaultInjectionMode::ParametricDrift { multiplier } => ConservativeStamp {
            current_amps: base.current_amps * multiplier,
            charge_coulombs: base.charge_coulombs * multiplier,
            flux_webers: base.flux_webers * multiplier,
            conductance_siemens: base.conductance_siemens * multiplier,
        },
        FaultInjectionMode::StuckAt { clamped_voltage_v } => {
            let g_clamp = 100.0;
            ConservativeStamp {
                current_amps: g_clamp * (branch_voltage_v - clamped_voltage_v),
                charge_coulombs: base.charge_coulombs,
                flux_webers: base.flux_webers,
                conductance_siemens: g_clamp,
            }
        }
    }
}
