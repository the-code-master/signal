// Module: engine/crates/signal-pcb/src/pkg_hdi_fab.rs — Package Delay, IPC-2226 HDI, Signed Fab Profiles & Minimal ECO
// Invariants: Verifies HybridSignature on .fabprofile.json (F-135); ECO rip-up preserves 100% of unaffected nets (F-134).
// Spec: architecture/03-rust-eda-engine.md, architecture/16-architectural-audit-flaws-and-futuristic-innovations.md

use crate::{PcbError, RoutedTrackSegment};
use signal_core::{HybridSignature, SecretBox};
use std::collections::BTreeSet;

/// Sums die-to-ball package pin delay (`pkg_delay_fs`) + routed PCB trace flight time (`FEAT-015`).
pub fn compute_total_channel_flight_time_fs(
    pkg_die_to_ball_fs: i64,
    routed_trace_length_nm: i64,
    effective_er: f64,
) -> i64 {
    let v_m_per_s = 299_792_458.0 / effective_er.max(1.0).sqrt();
    let pcb_delay_fs =
        (((routed_trace_length_nm.max(0) as f64) * 1.0e-9 / v_m_per_s) * 1.0e15).round() as i64;
    pkg_die_to_ball_fs.saturating_add(pcb_delay_fs)
}

#[derive(Debug, Clone, PartialEq)]
pub struct HdiMicroviaAudit {
    pub aspect_ratio: f64,
    pub ipc2226_aspect_compliant: bool,
    pub cumulative_reflow_strain_pct: f64,
    pub barrel_reliability_pass: bool,
}

/// Validates IPC-2226 HDI laser microvia aspect ratio (`<= 0.80`) & $N\times$ `260°C` reflow z-axis CTE copper strain (`FEAT-018`).
pub fn validate_ipc2226_hdi_microvia(
    dielectric_thickness_nm: i64,
    laser_drill_diameter_nm: i64,
    plating_thickness_nm: i64,
    reflow_cycles: u32,
) -> HdiMicroviaAudit {
    let ar = (dielectric_thickness_nm.max(1) as f64) / (laser_drill_diameter_nm.max(1) as f64);
    // FR4/ABF z-axis delta-CTE ~ 45 ppm/K over 235 K rise; thicker plating lowers stress concentration
    let plating_factor = (20_000.0 / (plating_thickness_nm.max(5_000) as f64)).clamp(0.6, 2.5);
    let per_cycle_strain_pct = 0.22 * ar * plating_factor;
    let cum_strain_pct = per_cycle_strain_pct * (reflow_cycles.max(1) as f64).sqrt();
    HdiMicroviaAudit {
        aspect_ratio: ar,
        ipc2226_aspect_compliant: ar <= 0.80,
        cumulative_reflow_strain_pct: cum_strain_pct,
        barrel_reliability_pass: ar <= 0.80 && cum_strain_pct <= 1.5,
    }
}

/// Vendor-parameterized signed Fab Capability Profile matching `fab-capability-profile.schema.json` (`F-135`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SignedFabCapabilityProfile {
    pub fab_vendor_id: String,
    pub process_class: String,
    pub min_trace_nm: i64,
    pub min_space_nm: i64,
    pub min_mechanical_drill_nm: i64,
    pub min_laser_microvia_nm: i64,
    pub min_annular_ring_nm: i64,
    pub max_aspect_ratio_milli: u32,
    pub signature: HybridSignature,
}

impl SignedFabCapabilityProfile {
    fn canonical_message(&self) -> String {
        format!(
            "FABPROFILE|1.0.0|{}|{}|{}|{}|{}|{}|{}|{}",
            self.fab_vendor_id,
            self.process_class,
            self.min_trace_nm,
            self.min_space_nm,
            self.min_mechanical_drill_nm,
            self.min_laser_microvia_nm,
            self.min_annular_ring_nm,
            self.max_aspect_ratio_milli
        )
    }

    pub fn builtin_signed(vendor_id: &str, seed: &SecretBox<[u8; 32]>) -> Result<Self, PcbError> {
        let up = vendor_id.trim().to_ascii_uppercase();
        let (proc_cls, tr, sp, mdrill, uvia, aring, max_ar) = match up.as_str() {
            "JLCPCB" => (
                "6L_STANDARD",
                90_000,
                90_000,
                200_000,
                100_000,
                125_000,
                10_000,
            ),
            "PCBWAY" => (
                "HDI_1PLUSNPLUS1",
                75_000,
                75_000,
                150_000,
                75_000,
                100_000,
                12_000,
            ),
            "EUROCIRCUITS" => (
                "POOL_6D", 100_000, 100_000, 150_000, 100_000, 125_000, 10_000,
            ),
            "SIERRA_CIRCUITS" => ("ULTRA_HDI", 50_000, 50_000, 150_000, 50_000, 75_000, 14_000),
            "TTM_TECH" => (
                "BACKPLANE_224G",
                65_000,
                65_000,
                150_000,
                65_000,
                90_000,
                16_000,
            ),
            _ => return Err(PcbError::UnknownFabVendor(vendor_id.to_string())),
        };
        let mut prof = Self {
            fab_vendor_id: up,
            process_class: proc_cls.to_string(),
            min_trace_nm: tr,
            min_space_nm: sp,
            min_mechanical_drill_nm: mdrill,
            min_laser_microvia_nm: uvia,
            min_annular_ring_nm: aring,
            max_aspect_ratio_milli: max_ar,
            signature: HybridSignature {
                ed25519_sig: [0_u8; 64],
                ml_dsa_65_sig: Vec::new(),
            },
        };
        let msg = prof.canonical_message();
        prof.signature = HybridSignature::sign_deterministic(seed, seed, msg.as_bytes());
        Ok(prof)
    }

    pub fn verify_authenticity(&self, seed: &SecretBox<[u8; 32]>) -> Result<(), PcbError> {
        let msg = self.canonical_message();
        let (ed_vk, pq_vk) = HybridSignature::derive_verifier_pair(seed, seed);
        self.signature
            .verify_both(&ed_vk, &pq_vk, msg.as_bytes())
            .map_err(|_| PcbError::FabProfileSignatureInvalid(self.fab_vendor_id.clone()))
    }
}

/// Minimal-Perturbation Incremental PCB ECO (`F-134`): rips up ONLY segments belonging to `dirty_nets`
/// while preserving 100% of unaffected copper segments verbatim.
pub fn apply_minimal_perturbation_pcb_eco(
    existing_routed: &[RoutedTrackSegment],
    dirty_nets: &BTreeSet<String>,
    replacement_segments: Vec<RoutedTrackSegment>,
) -> (Vec<RoutedTrackSegment>, usize) {
    let mut preserved: Vec<RoutedTrackSegment> = existing_routed
        .iter()
        .filter(|seg| !dirty_nets.contains(&seg.net_name))
        .cloned()
        .collect();
    let untouched_count = preserved.len();
    preserved.extend(replacement_segments);
    (preserved, untouched_count)
}
