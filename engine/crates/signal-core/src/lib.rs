// Module: engine/crates/signal-core/src/lib.rs — SI Units, 0.5-ULP Math, Hybrid PQC, SecretBox<T> & Quick Calculators
// Invariants: Redacts SecretBox Debug/Display (F-088); disambiguates m (1e-3) vs M/Meg (1e6) (F-075); blocks release dev-guest-auth (F-095).
// Spec: architecture/03-rust-eda-engine.md, architecture/15-application-security-and-project-isolation.md
#![forbid(unsafe_code)]

#[cfg(all(feature = "dev-guest-auth", not(debug_assertions)))]
compile_error!(
    "FATAL SECURITY VIOLATION (F-095): `dev-guest-auth` is strictly forbidden in release builds!"
);

use thiserror::Error;

pub mod core_math;
pub mod crypto_pqc;
pub mod eng_calculators;
pub mod expr_solver;
pub mod resistor_codes;
pub mod secret_box;
pub mod sha512_hkdf;
pub mod units;

pub use core_math::{det_cos, det_exp, det_ln, det_sin, det_tanh, limexp_c2};
pub use crypto_pqc::{
    HybridKemEnvelope, HybridSignature, hkdf_sha512, hmac_sha512, sha512, sha512_256_hex,
};
pub use eng_calculators::{
    DecodedResistor, Ipc2152TraceResult, LedResistorResult, VoltageDividerResult,
    calc_ipc2152_trace, calc_led_resistor, calc_voltage_divider, decode_eia96_smd,
    decode_resistor_color_bands, nearest_e_series_mohm,
};
pub use expr_solver::{SolvedEquation, evaluate_parametric_equation};
pub use secret_box::{SecretBox, ct_eq_bytes};
pub use units::{
    Femtoseconds, Microvolts, Millikelvin, Milliohms, Nanoamps, Nanometers, Picoseconds,
    parse_spice_si, si_prefix_exponent_milli_steps,
};

#[derive(Debug, Error, PartialEq, Eq)]
pub enum CoreError {
    #[error("unrecognized or ambiguous SI prefix `{0}` (use `m` for milli or `M`/`Meg` for Mega)")]
    InvalidSiPrefix(String),
    #[error("64-bit SI value overflow during scaling")]
    SiOverflow,
    #[error("invalid numeric engineering literal `{0}`")]
    InvalidNumber(String),
    #[error("parametric expression error: {0}")]
    ExpressionError(String),
    #[error("engineering calculator error: {0}")]
    CalculatorError(String),
    #[error("cryptographic envelope error: {0}")]
    CryptoError(String),
}

#[cfg(test)]
mod tests {
    use super::{CoreError, SecretBox, si_prefix_exponent_milli_steps};

    #[test]
    fn secret_box_never_leaks_plaintext_in_debug_or_display() {
        let dek = SecretBox::new(vec![0xAA_u8, 0xBB, 0xCC]);
        assert_eq!(format!("{dek:?}"), "SecretBox([REDACTED SECRET])");
        assert_eq!(format!("{dek}"), "[REDACTED SECRET]");
        assert_eq!(dek.expose_secret().len(), 3);
    }

    #[test]
    fn disambiguates_milli_vs_mega_and_rejects_ambiguous_meg_case() {
        assert_eq!(si_prefix_exponent_milli_steps("m"), Ok(-3));
        assert_eq!(si_prefix_exponent_milli_steps("M"), Ok(6));
        assert_eq!(si_prefix_exponent_milli_steps("Meg"), Ok(6));
        assert!(matches!(
            si_prefix_exponent_milli_steps("meg"),
            Err(CoreError::InvalidSiPrefix(_))
        ));
    }
}
