// Module: engine/crates/signal-parts/src/lib.rs — 3-Tier Universal Component Pinning, IPC-7351C & 5-Gate Verification
// Invariants: Rejects unpinned revisions & NDA Global CAS leaks; enforces 4-way pin/pad/SPICE/STEP bijection (18-*).
// Spec: architecture/18-universal-component-and-datasheet-repository.md
#![forbid(unsafe_code)]

use signal_proto::v1::VerificationBadge;
use thiserror::Error;

pub mod avl_provenance;
pub mod bijection_symbols;
pub mod padstack_ipc7351_step;
pub mod resolver_3tier;

pub use avl_provenance::{
    EnterpriseAvlCatalogEntry, query_avl_where_used, sign_authorized_lot_lineage,
    verify_distributor_provenance,
};
pub use bijection_symbols::{
    build_dual_standard_symbol_asts, split_symbol_into_functional_banks,
    verify_four_way_master_pin_bijection,
};
pub use padstack_ipc7351_step::{
    WindowpanePasteAperture, generate_ipc7351_land_pattern, sculpt_ipc7525_windowpane_paste,
    synthesize_watertight_step_ap242, verify_five_gate_part_alignment,
};
pub use resolver_3tier::{FederatedPartsRepository, global_cas_shard_prefix};

#[derive(Debug, Error, PartialEq, Eq)]
pub enum PartsError {
    #[error("invalid universal part URN `{0}`")]
    InvalidPartUrn(String),
    #[error("invalid BLAKE3 digest length ({0} chars; expected 64 hex chars)")]
    InvalidBlake3Digest(usize),
    #[error("NDA-tainted Org part `{0}` cannot be published to Tier-1 Global CAS")]
    NdaPartLeakIntoGlobalCas(String),
    #[error("part `{upid}` hash `{provided}` mismatches parts.lock.json `{expected}`")]
    PartsLockDigestMismatch {
        upid: String,
        expected: String,
        provided: String,
    },
    #[error("part `{0}` is missing a pinned entry in .signal/parts.lock.json")]
    UnpinnedPartInLockfile(String),
    #[error("universal part `{0}` not found in Tier-2 Org Overlay or Tier-1 Global CAS")]
    PartNotFound(String),
    #[error("4-way Master Pin Table bijection violation: {0}")]
    PinBijectionViolation(String),
    #[error("invalid IPC-7351 package geometry: {0}")]
    InvalidPackageGeometry(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LockedPartEntry {
    pub part_urn: String,
    pub revision_blake3: String,
    pub badge: VerificationBadge,
}

impl LockedPartEntry {
    pub fn validate(&self) -> Result<(), PartsError> {
        if !self.part_urn.starts_with("urn:signal:part:") {
            return Err(PartsError::InvalidPartUrn(self.part_urn.clone()));
        }
        if self.revision_blake3.len() != 64
            || !self.revision_blake3.bytes().all(|b| b.is_ascii_hexdigit())
        {
            return Err(PartsError::InvalidBlake3Digest(self.revision_blake3.len()));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use signal_geom::Point2Nm;
    use signal_proto::v1::{
        GenerateIpc7351FootprintRequest, Ipc7351DensityTier, MasterPinRecord, Point3dNm,
        ProjectPartsLockEntry, ProjectPartsLockfile, QueryAvlWhereUsedRequest,
        ResolveUniversalPartRequest, SplitSymbolBanksRequest, UniversalComponentRevision,
        VerifyDistributorProvenanceRequest,
    };
    use std::collections::BTreeMap;

    #[test]
    fn enforces_3tier_nda_isolation_and_parts_lock_pinning() {
        let mut repo = FederatedPartsRepository::new();
        let hash = "a1".repeat(32);
        let mut nda_part = UniversalComponentRevision {
            upid: "urn:signal:part:org-custom-asic:r1".into(),
            revision_hash_blake3: hash.clone(),
            orderable_mpn: "ASIC-NDA-01".into(),
            org_private_nda_taint: true,
            ..Default::default()
        };
        assert_eq!(
            repo.publish_tier1_global_part(nda_part.clone()),
            Err(PartsError::NdaPartLeakIntoGlobalCas(nda_part.upid.clone()))
        );
        repo.publish_tier2_org_overlay_part("org-alpha", nda_part.clone())
            .expect("tier2 ok");

        nda_part.org_private_nda_taint = false;
        nda_part.upid = "urn:signal:part:ti-tps62840:r1".into();
        let shard = repo.publish_tier1_global_part(nda_part).expect("tier1 ok");
        assert_eq!(shard, "a1");

        repo.register_tier3_project_lockfile(ProjectPartsLockfile {
            project_id: "proj-1".into(),
            locked_at_utc_ms: 1_000,
            entries: vec![ProjectPartsLockEntry {
                upid: "urn:signal:part:org-custom-asic:r1".into(),
                revision_hash_blake3: hash.clone(),
                padstack_hash_blake3: hash.clone(),
                from_org_avl_overlay: true,
            }],
        });

        let resolved = repo
            .resolve_universal_part(
                &ResolveUniversalPartRequest {
                    org_id: "org-alpha".into(),
                    project_id: "proj-1".into(),
                    upid: "urn:signal:part:org-custom-asic:r1".into(),
                    pinned_blake3_hash: String::new(),
                },
                0.96,
            )
            .expect("resolve org part");
        assert!(resolved.resolved_from_org_avl_overlay);

        // Cross-org isolation: org-beta cannot resolve org-alpha's Tier-2 NDA part!
        assert!(
            repo.resolve_universal_part(
                &ResolveUniversalPartRequest {
                    org_id: "org-beta".into(),
                    project_id: "proj-2".into(),
                    upid: "urn:signal:part:org-custom-asic:r1".into(),
                    pinned_blake3_hash: hash,
                },
                0.96,
            )
            .is_err()
        );
    }

    #[test]
    fn generates_ipc7351_qfn_verifies_5_gate_gold_and_splits_banks() {
        let fp = generate_ipc7351_land_pattern(&GenerateIpc7351FootprintRequest {
            package_family: "QFN-16".into(),
            pin_count: 16,
            lead_pitch_nm: 500_000,
            body_width_nm: 3_000_000,
            body_length_nm: 3_000_000,
            body_height_nm: 800_000,
            density_tier: Ipc7351DensityTier::Nominal as i32,
        })
        .expect("ipc7351 ok");
        let pad = fp.generated_padstacks[0].clone();
        let wp = sculpt_ipc7525_windowpane_paste(&pad, 650);
        assert_eq!(wp.area_coverage_permille, 650);

        let pins = vec![
            MasterPinRecord {
                pin_number: "1".into(),
                canonical_signal_name: "VCC".into(),
                symbol_unit_index: 1,
                pad_designator: "1".into(),
                spice_port_id: 1,
                step_lead_centroid_nm: Some(Point3dNm {
                    x_nm: -1_500_000,
                    y_nm: 0,
                    z_nm: 0,
                }),
            },
            MasterPinRecord {
                pin_number: "2".into(),
                canonical_signal_name: "CLK_IN".into(),
                symbol_unit_index: 1,
                pad_designator: "2".into(),
                spice_port_id: 2,
                step_lead_centroid_nm: Some(Point3dNm {
                    x_nm: 1_500_000,
                    y_nm: 0,
                    z_nm: 0,
                }),
            },
        ];
        let asts = build_dual_standard_symbol_asts("urn:signal:part:qfn:r1", &pins, 1);
        assert_eq!(asts.len(), 2);

        let split = split_symbol_into_functional_banks(
            &pins,
            &SplitSymbolBanksRequest {
                upid: "urn:signal:part:qfn:r1".into(),
                max_pins_per_bank: 32,
            },
        );
        assert_eq!(split.banks.len(), 2);

        let pad_centers = vec![
            ("1".into(), Point2Nm::new(-1_500_000, 0), pad.clone()),
            ("2".into(), Point2Nm::new(1_500_000, 0), pad),
        ];
        let align =
            verify_five_gate_part_alignment(&pins, &pad_centers, 120_000, 200_000, true, true);
        assert!(align.bijection_valid);
        assert!(align.min_lead_pad_overlap_ratio >= 0.99);
        assert_eq!(align.awarded_badge, VerificationBadge::GoldAttested as i32);
    }

    #[test]
    fn scans_avl_where_used_and_verifies_franchise_provenance() {
        let key = [0x5A_u8; 32];
        let sig = sign_authorized_lot_lineage(&key, "TPS62840DLCR", "DIGIKEY");
        let prov = verify_distributor_provenance(
            &VerifyDistributorProvenanceRequest {
                orderable_mpn: "TPS62840DLCR".into(),
                distributor_code: "DIGIKEY".into(),
                lot_trace_signature: sig,
            },
            &key,
        );
        assert!(prov.authorized_franchise_verified);

        let gray = verify_distributor_provenance(
            &VerifyDistributorProvenanceRequest {
                orderable_mpn: "TPS62840DLCR".into(),
                distributor_code: "UNKNOWN_BROKER".into(),
                lot_trace_signature: "deadbeef".into(),
            },
            &key,
        );
        assert!(!gray.authorized_franchise_verified);

        let avl = vec![EnterpriseAvlCatalogEntry {
            org_id: "org-1".into(),
            upid: "urn:signal:part:tps62840:r1".into(),
            orderable_mpn: "TPS62840DLCR".into(),
            approved_alternates: vec!["TPS62842DLCR".into()],
        }];
        let mut locks = BTreeMap::new();
        locks.insert(
            ("org-1".into(), "proj-radar".into()),
            ProjectPartsLockfile {
                project_id: "proj-radar".into(),
                locked_at_utc_ms: 100,
                entries: vec![ProjectPartsLockEntry {
                    upid: "urn:signal:part:tps62840:r1".into(),
                    revision_hash_blake3: "b2".repeat(32),
                    padstack_hash_blake3: "b2".repeat(32),
                    from_org_avl_overlay: false,
                }],
            },
        );
        let wu = query_avl_where_used(
            &QueryAvlWhereUsedRequest {
                org_id: "org-1".into(),
                target_mpn: "TPS62840DLCR".into(),
            },
            &avl,
            &locks,
        );
        assert_eq!(wu.affected_project_ids, vec!["proj-radar".to_string()]);
        assert_eq!(wu.suggested_drop_in_mpn, "TPS62842DLCR");
    }
}
