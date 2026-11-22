// Module: engine/crates/signal-core/src/eng_calculators.rs — Divider, LED Current-Limiting & IPC-2152 PCB Trace Calculators
// Invariants: Re-exports E-series/EIA-96 resistor decoders; computes exact IPC-2152 trace current & temp-rise (`FEAT-114`).
// Spec: architecture/01-product-vision-and-features.md, architecture/17-extended-feature-specifications-ui-and-rust.md
use crate::CoreError;
use crate::core_math::{det_exp, det_ln};
pub use crate::resistor_codes::{
    DecodedResistor, decode_eia96_smd, decode_resistor_color_bands, nearest_e_series_mohm,
};
use crate::units::Milliohms;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VoltageDividerResult {
    pub vout_volts: f64,
    pub divider_current_amps: f64,
    pub r_top_mohm: Milliohms,
    pub r_bot_mohm: Milliohms,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LedResistorResult {
    pub exact_resistance_ohms: f64,
    pub standard_e24_mohm: Milliohms,
    pub resistor_power_watts: f64,
    pub actual_current_amps: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ipc2152TraceResult {
    pub max_current_amps: f64,
    pub temp_rise_kelvin: f64,
    pub trace_resistance_ohms: f64,
    pub voltage_drop_volts: f64,
}

/// Computes voltage divider output and divider current (`FEAT-114`).
pub fn calc_voltage_divider(
    vin_volts: f64,
    r_top: Milliohms,
    r_bot: Milliohms,
) -> Result<VoltageDividerResult, CoreError> {
    let rt = r_top.to_si_f64();
    let rb = r_bot.to_si_f64();
    if rt <= 0.0 || rb <= 0.0 {
        return Err(CoreError::CalculatorError(
            "divider resistances must be > 0".to_string(),
        ));
    }
    let i_div = vin_volts / (rt + rb);
    Ok(VoltageDividerResult {
        vout_volts: vin_volts * rb / (rt + rb),
        divider_current_amps: i_div,
        r_top_mohm: r_top,
        r_bot_mohm: r_bot,
    })
}

/// Computes LED current-limiting resistor and nearest standard E24 value (`FEAT-114`).
pub fn calc_led_resistor(
    v_supply_volts: f64,
    v_forward_volts: f64,
    target_current_amps: f64,
) -> Result<LedResistorResult, CoreError> {
    if v_supply_volts <= v_forward_volts || target_current_amps <= 0.0 {
        return Err(CoreError::CalculatorError(
            "V_supply must exceed V_forward and I_target > 0".to_string(),
        ));
    }
    let v_drop = v_supply_volts - v_forward_volts;
    let exact_r = v_drop / target_current_amps;
    let std_r = nearest_e_series_mohm(Milliohms::from_si_f64(exact_r)?, 24)?;
    let actual_i = v_drop / std_r.to_si_f64();
    Ok(LedResistorResult {
        exact_resistance_ohms: exact_r,
        standard_e24_mohm: std_r,
        resistor_power_watts: v_drop * actual_i,
        actual_current_amps: actual_i,
    })
}

/// IPC-2152 / IPC-2221 copper trace current capacity, temp rise, and DC drop (`FEAT-114`).
pub fn calc_ipc2152_trace(
    width_mils: f64,
    copper_oz: f64,
    length_mm: f64,
    current_amps: f64,
    external_layer: bool,
) -> Result<Ipc2152TraceResult, CoreError> {
    if width_mils <= 0.0 || copper_oz <= 0.0 || length_mm < 0.0 || current_amps < 0.0 {
        return Err(CoreError::CalculatorError(
            "invalid PCB trace dimensions".to_string(),
        ));
    }
    let area_sq_mils = width_mils * (copper_oz * 1.378);
    let k = if external_layer { 0.048 } else { 0.024 };
    let a_pow = det_exp(0.725 * det_ln(area_sq_mils));
    let temp_rise = if current_amps == 0.0 {
        0.0
    } else {
        det_exp(det_ln(current_amps / (k * a_pow)) / 0.44)
    };
    let dt10_pow = det_exp(0.44 * det_ln(10.0));
    let i_max_10k = k * dt10_pow * a_pow;

    let area_m2 = area_sq_mils * 6.4516e-10;
    let r_ohms = 1.724e-8 * (1.0 + 0.00393 * temp_rise) * (length_mm * 1.0e-3) / area_m2;
    Ok(Ipc2152TraceResult {
        max_current_amps: i_max_10k,
        temp_rise_kelvin: temp_rise,
        trace_resistance_ohms: r_ohms,
        voltage_drop_volts: current_amps * r_ohms,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decodes_color_bands_eia96_and_finds_nearest_e_series() {
        let r4 = decode_resistor_color_bands(&["yellow", "violet", "red", "gold"]).expect("4.7k");
        assert_eq!(r4.nominal_mohm, Milliohms(4_700_000));
        assert!((r4.tolerance_pct - 5.0).abs() < 1e-9);

        let r6 = decode_resistor_color_bands(&["brown", "black", "black", "red", "brown", "blue"])
            .expect("10k 1% 10ppm");
        assert_eq!(r6.nominal_mohm, Milliohms(10_000_000));
        assert_eq!(r6.tempco_ppm_per_k, 10);

        let smd = decode_eia96_smd("01C").expect("01C");
        assert_eq!(smd.nominal_mohm, Milliohms(10_000_000));

        let near96 = nearest_e_series_mohm(Milliohms(31_250_000), 96).expect("E96");
        assert_eq!(near96, Milliohms(31_600_000));
    }

    #[test]
    fn calculates_divider_led_and_ipc2152_trace() {
        let div =
            calc_voltage_divider(5.0, Milliohms(10_000_000), Milliohms(10_000_000)).expect("div");
        assert!((div.vout_volts - 2.5).abs() < 1e-9);

        let led = calc_led_resistor(5.0, 2.0, 0.020).expect("led");
        assert_eq!(led.standard_e24_mohm, Milliohms(150_000));

        let tr = calc_ipc2152_trace(25.0, 1.0, 50.0, 1.5, true).expect("trace");
        assert!(tr.max_current_amps > 1.5);
        assert!(tr.temp_rise_kelvin > 0.0 && tr.temp_rise_kelvin < 10.0);
        assert!(tr.voltage_drop_volts > 0.0);
    }
}
