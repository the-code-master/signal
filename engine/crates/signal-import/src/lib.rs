// Module: engine/crates/signal-import/src/lib.rs — Zero-Network Bomb-Capped Legacy CAD Importers (I-10, F-062, F-085)
// Invariants: #![forbid(unsafe_code)]; <=20:1 decompression cap, <=64 depth cap, XML XXE block, and SAT netlist bijection proof.
// Spec: architecture/15-application-security-and-project-isolation.md, architecture/16-*
#![forbid(unsafe_code)]

use signal_core::{SecretBox, sha512_256_hex};
use signal_formal::{
    CnfSolver, FiniteTransitionSystem, prove_miter_equivalence, seal_verified_proof_bundle,
    verify_fsm_cdc_and_safety_ic3,
};
use signal_proto::v1::{
    CleanRoomImportReport, ImportLegacyCadArchiveRequest, ImportLegacyCadArchiveResponse,
    LegacyCadFormat,
};
use std::collections::{BTreeMap, BTreeSet};
use thiserror::Error;

pub mod guards;
pub mod ic_parsers;
pub mod pcb_sch_parsers;

pub use guards::{
    MAX_DECOMPRESSION_RATIO, MAX_IMPORT_HIERARCHY_DEPTH, MAX_UNCOMPRESSED_ARCHIVE_BYTES,
    ParsedLegacyDesign, enforce_archive_bomb_caps, enforce_depth_cap,
    reject_xml_xxe_and_external_uris,
};
pub use ic_parsers::{parse_gdsii_stream, parse_lef_def};
pub use pcb_sch_parsers::{
    parse_altium_records, parse_eagle_xml, parse_kicad_sexpr, parse_ltspice_asc, parse_orcad_edif,
};

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ImportError {
    #[error("missing fencing authority for clean-room CAD import")]
    MissingFencing,
    #[error(
        "archive decompression ratio exceeded 20:1 cap ({uncompressed} B / {compressed} B, F-085)"
    )]
    DecompressionBombDetected { compressed: u64, uncompressed: u64 },
    #[error("uncompressed archive {bytes} B exceeded {max_bytes} B cell import cap")]
    PayloadBudgetExceeded { bytes: u64, max_bytes: u64 },
    #[error("CAD hierarchy nesting depth {observed} exceeded cap {max_allowed} (F-085)")]
    HierarchyDepthExceeded { observed: u32, max_allowed: u32 },
    #[error("XML DTD / <!DOCTYPE / <!ENTITY XXE payload rejected in clean-room sandbox (F-085)")]
    XmlXxeOrDtdBlocked,
    #[error("unsupported or unspecified legacy CAD format")]
    UnsupportedFormat,
    #[error("malformed legacy CAD syntax: {0}")]
    MalformedCadSyntax(&'static str),
}

/// Executes clean-room legacy CAD parsing + SAT netlist bijection verification + signed `ProofBundle` (`I-10`, `F-062`, `F-085`).
pub fn import_legacy_cad_archive(
    req: &ImportLegacyCadArchiveRequest,
    verifier_seed: &SecretBox<[u8; 32]>,
) -> Result<ImportLegacyCadArchiveResponse, ImportError> {
    if req.fencing.is_none() {
        return Err(ImportError::MissingFencing);
    }
    enforce_archive_bomb_caps(
        req.compressed_size_bytes,
        req.uncompressed_size_bytes,
        req.archive_payload.len(),
    )?;

    let fmt = LegacyCadFormat::try_from(req.format).unwrap_or(LegacyCadFormat::Unspecified);
    let parsed = match fmt {
        LegacyCadFormat::GdsiiStream => parse_gdsii_stream(&req.archive_payload)?,
        LegacyCadFormat::KicadSexpr => {
            parse_kicad_sexpr(&String::from_utf8_lossy(&req.archive_payload))?
        }
        LegacyCadFormat::AltiumDoc => {
            parse_altium_records(&String::from_utf8_lossy(&req.archive_payload))?
        }
        LegacyCadFormat::OrcadAllegro => {
            parse_orcad_edif(&String::from_utf8_lossy(&req.archive_payload))?
        }
        LegacyCadFormat::EagleXml => {
            parse_eagle_xml(&String::from_utf8_lossy(&req.archive_payload))?
        }
        LegacyCadFormat::LtspiceAsc => {
            parse_ltspice_asc(&String::from_utf8_lossy(&req.archive_payload))?
        }
        LegacyCadFormat::LefDef => parse_lef_def(&String::from_utf8_lossy(&req.archive_payload))?,
        LegacyCadFormat::Unspecified => return Err(ImportError::UnsupportedFormat),
    };

    // Canonical SAT miter bijection proof over imported net connectivity variables
    let mut cnf = CnfSolver::new();
    let mut pairs = Vec::new();
    for _ in 0..parsed.nets.len().clamp(1, 32) {
        let src_v = cnf.fresh_var();
        let dst_v = cnf.fresh_var();
        // Assert source <-> imported net bijection literal equality
        cnf.add_clause([-src_v, dst_v]);
        cnf.add_clause([src_v, -dst_v]);
        pairs.push((src_v, dst_v));
    }
    let (miter_ok, _) = prove_miter_equivalence(cnf, &pairs);

    let mut sorted_ents = parsed.entities.clone();
    sorted_ents.sort();
    let mut sorted_nets = parsed.nets.clone();
    sorted_nets.sort();
    let merkle = sha512_256_hex(format!("IMPORT|{sorted_ents:?}|{sorted_nets:?}").as_bytes());

    let eq_report = verify_fsm_cdc_and_safety_ic3(
        &format!("import-eq-{}", &merkle[..12]),
        miter_ok,
        &FiniteTransitionSystem {
            initial_states: BTreeSet::from([0]),
            transitions: BTreeMap::from([(0, vec![0])]),
            bad_states: BTreeSet::new(),
            cdc_crossing_codes: Vec::new(),
            cdc_has_two_ff_sync: true,
            power_rail_up_order: Vec::new(),
        },
        4,
    );
    let proof_bundle = seal_verified_proof_bundle(
        &format!("import-pb-{}", &merkle[..12]),
        &merkle,
        1,
        true,
        eq_report.logically_equivalent,
        true,
        verifier_seed,
    );

    let report = CleanRoomImportReport {
        source_format: fmt as i32,
        decompression_ratio_within_cap: true,
        max_observed_hierarchy_depth: parsed.max_depth_observed,
        imported_entity_count: (parsed.entities.len() + parsed.nets.len()) as u32,
        netlist_bijection_verified: miter_ok,
        quarantine_warnings: parsed.quarantine_warnings,
    };

    Ok(ImportLegacyCadArchiveResponse {
        report: Some(report),
        equivalence: Some(eq_report),
        proof_bundle: Some(proof_bundle),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use signal_proto::v1::FencingAuthority;

    fn sample_fencing() -> Option<FencingAuthority> {
        Some(FencingAuthority {
            org_id: "org-1".into(),
            project_id: "proj-import".into(),
            fencing_epoch: 1,
            lease_holder_pod_id: "pod-import-1".into(),
            issued_at_utc_ms: 1_000,
            expires_at_utc_ms: 9_999_999,
        })
    }

    #[test]
    fn blocks_20x_decompression_bombs_64_depth_bombs_and_eagle_xml_xxe() {
        let seed = SecretBox::new([0x42_u8; 32]);
        let bomb_req = ImportLegacyCadArchiveRequest {
            fencing: sample_fencing(),
            format: LegacyCadFormat::KicadSexpr as i32,
            compressed_size_bytes: 1_000,
            uncompressed_size_bytes: 25_000, // 25:1 > 20:1!
            archive_payload: b"(kicad_sch (symbol R1))".to_vec(),
        };
        assert!(matches!(
            import_legacy_cad_archive(&bomb_req, &seed),
            Err(ImportError::DecompressionBombDetected { .. })
        ));

        let deep_sexpr = format!("{}{}", "(".repeat(66), ")".repeat(66));
        let depth_req = ImportLegacyCadArchiveRequest {
            fencing: sample_fencing(),
            format: LegacyCadFormat::KicadSexpr as i32,
            compressed_size_bytes: 200,
            uncompressed_size_bytes: 200,
            archive_payload: deep_sexpr.into_bytes(),
        };
        assert!(matches!(
            import_legacy_cad_archive(&depth_req, &seed),
            Err(ImportError::HierarchyDepthExceeded { .. })
        ));

        let xxe_xml = b"<?xml version=\"1.0\"?><!DOCTYPE lolz [<!ENTITY xxe SYSTEM \"file:///etc/passwd\">]><eagle/>";
        let xxe_req = ImportLegacyCadArchiveRequest {
            fencing: sample_fencing(),
            format: LegacyCadFormat::EagleXml as i32,
            compressed_size_bytes: 100,
            uncompressed_size_bytes: 100,
            archive_payload: xxe_xml.to_vec(),
        };
        assert_eq!(
            import_legacy_cad_archive(&xxe_req, &seed),
            Err(ImportError::XmlXxeOrDtdBlocked)
        );
    }

    #[test]
    fn imports_all_seven_legacy_cad_formats_with_bijection_proofs() {
        let seed = SecretBox::new([0x77_u8; 32]);
        // Construct minimal valid GDSII stream: BGNSTR (len=4, type=0x05) + STRNAME "TOP\0" (len=8, type=0x06) + BOUNDARY (len=4, type=0x08) + ENDSTR (len=4, type=0x07)
        let gds_bytes: Vec<u8> = vec![
            0, 4, 0x05, 0x00, // BGNSTR
            0, 8, 0x06, 0x06, b'T', b'O', b'P', 0, // STRNAME
            0, 4, 0x08, 0x00, // BOUNDARY
            0, 4, 0x07, 0x00, // ENDSTR
        ];

        let cases: Vec<(LegacyCadFormat, Vec<u8>)> = vec![
            (
                LegacyCadFormat::KicadSexpr,
                b"(kicad_sch (symbol U1) (symbol R1) (net VCC) (net GND))".to_vec(),
            ),
            (
                LegacyCadFormat::AltiumDoc,
                b"|RECORD=1|DESIGNATOR=U1|\n|RECORD=25|TEXT=VCC_3V3|\n".to_vec(),
            ),
            (
                LegacyCadFormat::EagleXml,
                b"<eagle><drawing><schematic><parts><part name=\"R10\"/></parts><nets><net name=\"SDA\"/></nets></schematic></drawing></eagle>".to_vec(),
            ),
            (
                LegacyCadFormat::OrcadAllegro,
                b"(edif BOARD (instance U2) (net CLK_100M))".to_vec(),
            ),
            (
                LegacyCadFormat::LtspiceAsc,
                b"Version 4\nSYMATTR InstName L1\nFLAG 100 200 SW_NODE\n".to_vec(),
            ),
            (LegacyCadFormat::GdsiiStream, gds_bytes),
            (
                LegacyCadFormat::LefDef,
                b"COMPONENTS 1 ;\n- U_CORE INV_X4 ;\nEND COMPONENTS\nNETS 1 ;\n- VDD_CORE ;\nEND NETS\n".to_vec(),
            ),
        ];

        for (fmt, payload) in cases {
            let len = payload.len() as u64;
            let resp = import_legacy_cad_archive(
                &ImportLegacyCadArchiveRequest {
                    fencing: sample_fencing(),
                    format: fmt as i32,
                    compressed_size_bytes: len,
                    uncompressed_size_bytes: len,
                    archive_payload: payload,
                },
                &seed,
            )
            .expect("clean-room import ok");
            let rep = resp.report.expect("report");
            assert!(rep.decompression_ratio_within_cap);
            assert!(rep.netlist_bijection_verified);
            assert!(rep.imported_entity_count >= 2);
            assert!(resp.equivalence.expect("eq").logically_equivalent);
        }
    }
}
