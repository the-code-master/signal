// Module: engine/crates/signal-core/src/units.rs — Strongly-Typed 64-bit SI Primitives & SPICE Unit Parser
// Invariants: Disambiguates m (1e-3) vs M/Meg (1e6) (F-075); rejects ambiguous lowercase `meg`; checks i64 overflow.
// Spec: architecture/03-rust-eda-engine.md, architecture/16-architectural-audit-flaws-and-futuristic-innovations.md
use crate::CoreError;
use std::ops::{Add, Neg, Sub};

macro_rules! define_si_unit {
    ($name:ident, $scale_to_base:expr) => {
        #[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name(pub i64);

        impl $name {
            pub const ZERO: Self = Self(0);

            pub const fn raw(self) -> i64 {
                self.0
            }

            pub fn checked_add(self, rhs: Self) -> Result<Self, CoreError> {
                self.0
                    .checked_add(rhs.0)
                    .map(Self)
                    .ok_or(CoreError::SiOverflow)
            }

            pub fn checked_sub(self, rhs: Self) -> Result<Self, CoreError> {
                self.0
                    .checked_sub(rhs.0)
                    .map(Self)
                    .ok_or(CoreError::SiOverflow)
            }

            pub fn to_si_f64(self) -> f64 {
                (self.0 as f64) * $scale_to_base
            }

            pub fn from_si_f64(value: f64) -> Result<Self, CoreError> {
                if !value.is_finite() {
                    return Err(CoreError::InvalidNumber("non-finite SI value".to_string()));
                }
                let scaled = (value / $scale_to_base).round();
                if scaled < (i64::MIN as f64) || scaled >= (i64::MAX as f64) {
                    return Err(CoreError::SiOverflow);
                }
                Ok(Self(scaled as i64))
            }
        }

        impl Add for $name {
            type Output = Self;
            fn add(self, rhs: Self) -> Self::Output {
                Self(self.0.saturating_add(rhs.0))
            }
        }

        impl Sub for $name {
            type Output = Self;
            fn sub(self, rhs: Self) -> Self::Output {
                Self(self.0.saturating_sub(rhs.0))
            }
        }

        impl Neg for $name {
            type Output = Self;
            fn neg(self) -> Self::Output {
                Self(self.0.saturating_neg())
            }
        }
    };
}

define_si_unit!(Nanometers, 1.0e-9);
define_si_unit!(Picoseconds, 1.0e-12);
define_si_unit!(Femtoseconds, 1.0e-15);
define_si_unit!(Microvolts, 1.0e-6);
define_si_unit!(Nanoamps, 1.0e-9);
define_si_unit!(Milliohms, 1.0e-3);
define_si_unit!(Millikelvin, 1.0e-3);

pub fn si_prefix_exponent_milli_steps(prefix: &str) -> Result<i32, CoreError> {
    match prefix {
        "f" => Ok(-15),
        "p" => Ok(-12),
        "n" => Ok(-9),
        "u" | "µ" | "μ" => Ok(-6),
        "m" => Ok(-3),
        "" => Ok(0),
        "k" | "K" => Ok(3),
        "M" | "Meg" | "MEG" => Ok(6),
        "G" => Ok(9),
        "T" => Ok(12),
        other => Err(CoreError::InvalidSiPrefix(other.to_string())),
    }
}

fn pow10_from_exp(exp: i32) -> f64 {
    match exp {
        -15 => 1.0e-15,
        -12 => 1.0e-12,
        -9 => 1.0e-9,
        -6 => 1.0e-6,
        -3 => 1.0e-3,
        0 => 1.0,
        3 => 1.0e3,
        6 => 1.0e6,
        9 => 1.0e9,
        12 => 1.0e12,
        other => 10.0_f64.powi(other),
    }
}

fn strip_known_unit_tail(suffix: &str) -> &str {
    for unit in [
        "ohm", "Ohm", "OHM", "Hz", "hz", "Wb", "Ω", "V", "A", "F", "H", "s", "W", "K",
    ] {
        if let Some(prefix) = suffix.strip_suffix(unit)
            && (!prefix.is_empty() || suffix == unit)
        {
            return prefix;
        }
    }
    suffix
}

/// Unambiguous SPICE/EDA engineering value parser (`F-075`).
/// Rules: `m` is strictly milli (`1e-3`), `Meg` / `MEG` / `M` is strictly mega (`1e6`),
/// and lowercase `meg` is rejected as ambiguous (`CoreError::InvalidSiPrefix`).
pub fn parse_spice_si(raw: &str) -> Result<f64, CoreError> {
    let s = raw.trim();
    if s.is_empty() {
        return Err(CoreError::InvalidNumber(
            "empty engineering literal".to_string(),
        ));
    }

    let bytes = s.as_bytes();
    let mut idx = 0;
    if bytes[idx] == b'+' || bytes[idx] == b'-' {
        idx += 1;
    }
    let mut saw_digit = false;
    while idx < bytes.len() && bytes[idx].is_ascii_digit() {
        saw_digit = true;
        idx += 1;
    }
    if idx < bytes.len() && bytes[idx] == b'.' {
        idx += 1;
        while idx < bytes.len() && bytes[idx].is_ascii_digit() {
            saw_digit = true;
            idx += 1;
        }
    }
    if !saw_digit {
        return Err(CoreError::InvalidNumber(s.to_string()));
    }

    if idx < bytes.len() && (bytes[idx] == b'e' || bytes[idx] == b'E') {
        let next = bytes.get(idx + 1).copied();
        if next.is_some_and(|c| c == b'+' || c == b'-' || c.is_ascii_digit()) {
            idx += 1;
            if bytes[idx] == b'+' || bytes[idx] == b'-' {
                idx += 1;
            }
            let exp_start = idx;
            while idx < bytes.len() && bytes[idx].is_ascii_digit() {
                idx += 1;
            }
            if idx == exp_start {
                return Err(CoreError::InvalidNumber(s.to_string()));
            }
        }
    }

    let num_str = &s[..idx];
    let tail = s[idx..].trim();
    let mantissa: f64 = num_str
        .parse()
        .map_err(|_| CoreError::InvalidNumber(s.to_string()))?;
    if !mantissa.is_finite() {
        return Err(CoreError::InvalidNumber(s.to_string()));
    }
    if tail.is_empty() {
        return Ok(mantissa);
    }

    if tail.to_ascii_lowercase().starts_with("meg") && tail != "Meg" && tail != "MEG" {
        let stripped = strip_known_unit_tail(tail);
        if stripped != "Meg" && stripped != "MEG" {
            return Err(CoreError::InvalidSiPrefix(stripped.to_string()));
        }
    }

    let prefix = if tail == "m" || tail == "M" || tail == "Meg" || tail == "MEG" {
        tail
    } else {
        strip_known_unit_tail(tail)
    };
    let exp = si_prefix_exponent_milli_steps(prefix)?;
    Ok(mantissa * pow10_from_exp(exp))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_all_spice_prefixes_and_disambiguates_m_vs_meg() {
        assert!((parse_spice_si("10f").expect("f") - 10e-15).abs() < 1e-27);
        assert!((parse_spice_si("4.7pF").expect("p") - 4.7e-12).abs() < 1e-24);
        assert!((parse_spice_si("100n").expect("n") - 100e-9).abs() < 1e-20);
        assert!((parse_spice_si("2.2uH").expect("u") - 2.2e-6).abs() < 1e-18);
        assert!((parse_spice_si("2.2µF").expect("mu") - 2.2e-6).abs() < 1e-18);
        assert!((parse_spice_si("50m").expect("m") - 0.05).abs() < 1e-15);
        assert!((parse_spice_si("10k").expect("k") - 10_000.0).abs() < 1e-9);
        assert!((parse_spice_si("1.5Meg").expect("Meg") - 1.5e6).abs() < 1e-6);
        assert!((parse_spice_si("1.5M").expect("M") - 1.5e6).abs() < 1e-6);
        assert!((parse_spice_si("2.4GHz").expect("G") - 2.4e9).abs() < 1e-3);
        assert!((parse_spice_si("1T").expect("T") - 1.0e12).abs() < 1.0);
        assert!(matches!(
            parse_spice_si("10meg"),
            Err(CoreError::InvalidSiPrefix(_))
        ));
    }

    #[test]
    fn si_newtypes_convert_and_detect_overflow() {
        let v = Microvolts::from_si_f64(3.3).expect("3.3V");
        assert_eq!(v, Microvolts(3_300_000));
        assert!((v.to_si_f64() - 3.3).abs() < 1e-12);
        assert_eq!(
            Nanometers(i64::MAX).checked_add(Nanometers(1)),
            Err(CoreError::SiOverflow)
        );
        assert_eq!(Femtoseconds(500) + Femtoseconds(250), Femtoseconds(750));
        assert_eq!(Nanoamps(1_000) - Nanoamps(250), Nanoamps(750));
        assert_eq!(Milliohms::from_si_f64(10.0), Ok(Milliohms(10_000)));
        assert_eq!(Millikelvin::from_si_f64(298.15), Ok(Millikelvin(298_150)));
    }
}
