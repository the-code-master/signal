// Module: engine/crates/signal-ic/src/lib.rs — Analog/Digital IC, UPF/DFT, Chiplet Halo, CPO Photonics & Cryo-CMOS
// Invariants: Signed *.halo.gds.sig zero-lateral-movement federation (F-125); overflow-proof 4K Fermi-Dirac & baseband CPO (F-148).
// Spec: architecture/03-rust-eda-engine.md, architecture/16-architectural-audit-flaws-and-futuristic-innovations.md
#![forbid(unsafe_code)]

use thiserror::Error;

pub mod analog_char_bspdn;
pub mod cryo_cmos;
pub mod digital_sta_eco_ucie;
pub mod mpw_shuttle;
pub mod photonics_cpo;
pub mod upf_dft_pdk;

pub use analog_char_bspdn::{
    BacksidePdnResult, CommonCentroidPelgromReport, GmIdOperatingPoint, LibertyNldmLvfTable,
    characterize_nldm_7x7_and_lvf, generate_abba_common_centroid_pelgrom,
    lookup_gmid_4d_operating_point, solve_backside_bspdn_ir_and_thermal,
};
pub use cryo_cmos::{
    BAND_TAIL_FLOOR_TEMP_MKELVIN, CryoMosOperatingState, compute_cryo_effective_temp_mkelvin,
    log_fermi_dirac_f0, run_photonics_and_cryo_sim, solve_cryo_cmos_fermi_continuation,
};
pub use digital_sta_eco_ucie::{
    LithoCmpReport, StaAndMetalEcoReport, StaTimingPath, build_signed_chiplet_halo_abstract,
    compute_dme_zero_skew_tap_nm, evaluate_hopkins_litho_and_cmp_fill,
    evaluate_sta_and_post_mask_metal_eco, route_xy_dor_noc_hops,
};
pub use mpw_shuttle::{ShuttleSubmissionMetrics, validate_mpw_shuttle_tile};
pub use photonics_cpo::{
    ComplexEnvelope, CpoLinkStepSummary, eval_microring_resonator_transfer,
    scatter_directional_coupler_2x2, simulate_cpo_mzm_link_baseband,
};
pub use upf_dft_pdk::{
    DftScanIjtagReport, UpfCrossDomainArc, UpfIsolationShifterRule, UpfPowerDomain,
    analyze_ic_signoff_and_eco, decrypt_ieee1735_pdk_session_envelope,
    synthesize_scan_atpg_and_ijtag, verify_ieee1801_upf_power_intent,
};

#[derive(Debug, Error, PartialEq, Eq)]
pub enum IcError {
    #[error("invalid analog gm/Id or channel length ({0} nm)")]
    InvalidAnalogSizingQuery(i64),
    #[error("invalid IC clock period ({0} fs)")]
    InvalidClockPeriod(i64),
    #[error("IEEE 1735 v2 encrypted foundry PDK MAC verification failed")]
    Ieee1735EnvelopeTampered,
    #[error("optical carrier wavelength ({0} pm) outside O/C/L telecom band or non-positive step")]
    InvalidOpticalCarrier(i64),
    #[error("cryogenic stage temperature ({0} mK) outside 10 mK .. 500 K range")]
    InvalidCryoTemperature(i64),
    #[error("Cryo-CMOS Fermi-Dirac continuation diverged")]
    CryoContinuationDiverged,
    #[error("Open-Silicon MPW shuttle requires SKY130, GF180MCU, or IHP_SG13G2 PDK")]
    UnsupportedOpenSiliconPdk,
}
