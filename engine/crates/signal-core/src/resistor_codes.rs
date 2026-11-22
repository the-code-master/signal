// Module: engine/crates/signal-core/src/resistor_codes.rs — 4/5/6-Band Color, EIA-96 SMD & IEC 60063 E-Series Decoders
// Invariants: Exact IEC 60063 E12/E24/E48/E96 tables (`01C` = 10 kOhm); geometric log-distance E-series matching (`FEAT-114`).
// Spec: architecture/01-product-vision-and-features.md, architecture/17-extended-feature-specifications-ui-and-rust.md
use crate::CoreError;
use crate::core_math::det_ln;
use crate::units::Milliohms;

const E12_MANTISSAS: [u16; 12] = [100, 120, 150, 180, 220, 270, 330, 390, 470, 560, 680, 820];
const E24_MANTISSAS: [u16; 24] = [
    100, 110, 120, 130, 150, 160, 180, 200, 220, 240, 270, 300, 330, 360, 390, 430, 470, 510, 560,
    620, 680, 750, 820, 910,
];
const E48_MANTISSAS: [u16; 48] = [
    100, 105, 110, 115, 121, 127, 133, 140, 147, 154, 162, 169, 178, 187, 196, 205, 215, 226, 237,
    249, 261, 274, 287, 301, 316, 332, 348, 365, 383, 402, 422, 442, 464, 487, 511, 536, 562, 590,
    619, 649, 681, 715, 750, 787, 825, 866, 909, 953,
];
const E96_MANTISSAS: [u16; 96] = [
    100, 102, 105, 107, 110, 113, 115, 118, 121, 124, 127, 130, 133, 137, 140, 143, 147, 150, 154,
    158, 162, 165, 169, 174, 178, 182, 187, 191, 196, 200, 205, 210, 215, 221, 226, 232, 237, 243,
    249, 255, 261, 267, 274, 280, 287, 294, 301, 309, 316, 324, 332, 340, 348, 357, 365, 374, 383,
    392, 402, 412, 422, 432, 442, 453, 464, 475, 487, 499, 511, 523, 536, 549, 562, 576, 590, 604,
    619, 634, 649, 665, 681, 698, 715, 732, 750, 768, 787, 806, 825, 845, 866, 887, 909, 931, 953,
    976,
];

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DecodedResistor {
    pub nominal_mohm: Milliohms,
    pub tolerance_pct: f64,
    pub tempco_ppm_per_k: i32,
}

fn color_digit(color: &str) -> Result<i64, CoreError> {
    match color.trim().to_ascii_lowercase().as_str() {
        "black" => Ok(0),
        "brown" => Ok(1),
        "red" => Ok(2),
        "orange" => Ok(3),
        "yellow" => Ok(4),
        "green" => Ok(5),
        "blue" => Ok(6),
        "violet" | "purple" => Ok(7),
        "grey" | "gray" => Ok(8),
        "white" => Ok(9),
        other => Err(CoreError::CalculatorError(format!(
            "invalid digit band `{other}`"
        ))),
    }
}

fn color_multiplier_ohms(color: &str) -> Result<f64, CoreError> {
    match color.trim().to_ascii_lowercase().as_str() {
        "silver" => Ok(0.01),
        "gold" => Ok(0.1),
        "black" => Ok(1.0),
        "brown" => Ok(10.0),
        "red" => Ok(100.0),
        "orange" => Ok(1_000.0),
        "yellow" => Ok(10_000.0),
        "green" => Ok(100_000.0),
        "blue" => Ok(1_000_000.0),
        "violet" | "purple" => Ok(10_000_000.0),
        "grey" | "gray" => Ok(100_000_000.0),
        "white" => Ok(1_000_000_000.0),
        other => Err(CoreError::CalculatorError(format!(
            "invalid multiplier band `{other}`"
        ))),
    }
}

fn color_tolerance(color: &str) -> Result<f64, CoreError> {
    match color.trim().to_ascii_lowercase().as_str() {
        "brown" => Ok(1.0),
        "red" => Ok(2.0),
        "green" => Ok(0.5),
        "blue" => Ok(0.25),
        "violet" | "purple" => Ok(0.1),
        "grey" | "gray" => Ok(0.05),
        "gold" => Ok(5.0),
        "silver" => Ok(10.0),
        other => Err(CoreError::CalculatorError(format!(
            "invalid tolerance band `{other}`"
        ))),
    }
}

fn color_tempco(color: &str) -> Result<i32, CoreError> {
    match color.trim().to_ascii_lowercase().as_str() {
        "black" => Ok(250),
        "brown" => Ok(100),
        "red" => Ok(50),
        "orange" => Ok(15),
        "yellow" => Ok(25),
        "green" => Ok(20),
        "blue" => Ok(10),
        "violet" | "purple" => Ok(5),
        "grey" | "gray" => Ok(1),
        other => Err(CoreError::CalculatorError(format!(
            "invalid tempco band `{other}`"
        ))),
    }
}

pub fn decode_resistor_color_bands(bands: &[&str]) -> Result<DecodedResistor, CoreError> {
    match bands.len() {
        4 => {
            let sig = color_digit(bands[0])? * 10 + color_digit(bands[1])?;
            let ohms = (sig as f64) * color_multiplier_ohms(bands[2])?;
            Ok(DecodedResistor {
                nominal_mohm: Milliohms::from_si_f64(ohms)?,
                tolerance_pct: color_tolerance(bands[3])?,
                tempco_ppm_per_k: 0,
            })
        }
        5 | 6 => {
            let sig =
                color_digit(bands[0])? * 100 + color_digit(bands[1])? * 10 + color_digit(bands[2])?;
            let ohms = (sig as f64) * color_multiplier_ohms(bands[3])?;
            let tempco = if bands.len() == 6 {
                color_tempco(bands[5])?
            } else {
                0
            };
            Ok(DecodedResistor {
                nominal_mohm: Milliohms::from_si_f64(ohms)?,
                tolerance_pct: color_tolerance(bands[4])?,
                tempco_ppm_per_k: tempco,
            })
        }
        n => Err(CoreError::CalculatorError(format!(
            "expected 4, 5, or 6 color bands, got {n}"
        ))),
    }
}

pub fn decode_eia96_smd(code: &str) -> Result<DecodedResistor, CoreError> {
    let trimmed = code.trim().to_ascii_uppercase();
    if trimmed.len() == 3
        && trimmed.as_bytes()[0].is_ascii_digit()
        && trimmed.as_bytes()[1].is_ascii_digit()
        && trimmed.as_bytes()[2].is_ascii_alphabetic()
        && trimmed.as_bytes()[2] != b'R'
    {
        let idx: usize = trimmed[..2]
            .parse()
            .map_err(|_| CoreError::CalculatorError(trimmed.clone()))?;
        if !(1..=96).contains(&idx) {
            return Err(CoreError::CalculatorError(format!(
                "EIA-96 index `{idx}` out of 01..96"
            )));
        }
        let mult = match trimmed.as_bytes()[2] {
            b'Z' => 0.001,
            b'Y' | b'R' => 0.01,
            b'X' | b'S' => 0.1,
            b'A' => 1.0,
            b'B' | b'H' => 10.0,
            b'C' => 100.0,
            b'D' => 1_000.0,
            b'E' => 10_000.0,
            b'F' => 100_000.0,
            other => {
                return Err(CoreError::CalculatorError(format!(
                    "invalid EIA-96 multiplier `{}`",
                    char::from(other)
                )));
            }
        };
        let ohms = f64::from(E96_MANTISSAS[idx - 1]) * mult;
        return Ok(DecodedResistor {
            nominal_mohm: Milliohms::from_si_f64(ohms)?,
            tolerance_pct: 1.0,
            tempco_ppm_per_k: 100,
        });
    }

    if trimmed.contains('R') {
        let ohms: f64 = trimmed
            .replace('R', ".")
            .parse()
            .map_err(|_| CoreError::CalculatorError(trimmed.clone()))?;
        return Ok(DecodedResistor {
            nominal_mohm: Milliohms::from_si_f64(ohms)?,
            tolerance_pct: 1.0,
            tempco_ppm_per_k: 100,
        });
    }

    if (trimmed.len() == 3 || trimmed.len() == 4) && trimmed.bytes().all(|b| b.is_ascii_digit()) {
        let sig_len = trimmed.len() - 1;
        let sig: f64 = trimmed[..sig_len]
            .parse()
            .map_err(|_| CoreError::CalculatorError(trimmed.clone()))?;
        let exp: i32 = trimmed[sig_len..]
            .parse()
            .map_err(|_| CoreError::CalculatorError(trimmed.clone()))?;
        return Ok(DecodedResistor {
            nominal_mohm: Milliohms::from_si_f64(sig * 10.0_f64.powi(exp))?,
            tolerance_pct: if trimmed.len() == 3 { 5.0 } else { 1.0 },
            tempco_ppm_per_k: 100,
        });
    }
    Err(CoreError::CalculatorError(format!(
        "unrecognized SMD code `{code}`"
    )))
}

pub fn nearest_e_series_mohm(target: Milliohms, e_series: u32) -> Result<Milliohms, CoreError> {
    if target.0 <= 0 {
        return Err(CoreError::CalculatorError(
            "target resistance must be > 0".to_string(),
        ));
    }
    let table: &[u16] = match e_series {
        12 => &E12_MANTISSAS,
        24 => &E24_MANTISSAS,
        48 => &E48_MANTISSAS,
        96 => &E96_MANTISSAS,
        other => {
            return Err(CoreError::CalculatorError(format!(
                "unsupported E-series `E{other}`"
            )));
        }
    };

    let ohms = target.to_si_f64();
    let ln_target = det_ln(ohms);
    let decade = (ln_target / std::f64::consts::LN_10).floor() as i32;
    let mut best_mohm = target.0;
    let mut best_log_err = f64::INFINITY;

    for d in (decade - 1)..=(decade + 1) {
        let scale = 10.0_f64.powi(d - 2);
        for &m in table {
            let cand_ohms = f64::from(m) * scale;
            let log_err = (det_ln(cand_ohms) - ln_target).abs();
            if log_err < best_log_err {
                best_log_err = log_err;
                best_mohm = Milliohms::from_si_f64(cand_ohms)?.0;
            }
        }
    }
    Ok(Milliohms(best_mohm))
}
