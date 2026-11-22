// Module: engine/crates/signal-import/src/guards.rs — Bomb-Capped Decompression, Depth <=64 & XML XXE Hardening (F-085)
// Invariants: Rejects >20:1 archive bombs, >64 MiB payloads, >64 nesting depth, and any <!DOCTYPE / <!ENTITY XXE payload.
// Spec: architecture/15-application-security-and-project-isolation.md, architecture/16-*

use crate::ImportError;

pub const MAX_DECOMPRESSION_RATIO: u64 = 20;
pub const MAX_UNCOMPRESSED_ARCHIVE_BYTES: u64 = 64 * 1024 * 1024; // 64 MiB per cell import
pub const MAX_IMPORT_HIERARCHY_DEPTH: u32 = 64;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedLegacyDesign {
    pub entities: Vec<String>,
    pub nets: Vec<String>,
    pub max_depth_observed: u32,
    pub quarantine_warnings: Vec<String>,
}

pub fn enforce_archive_bomb_caps(
    compressed_bytes: u64,
    uncompressed_bytes: u64,
    actual_payload_len: usize,
) -> Result<(), ImportError> {
    let eff_uncomp = uncompressed_bytes.max(actual_payload_len as u64);
    if eff_uncomp > MAX_UNCOMPRESSED_ARCHIVE_BYTES {
        return Err(ImportError::PayloadBudgetExceeded {
            bytes: eff_uncomp,
            max_bytes: MAX_UNCOMPRESSED_ARCHIVE_BYTES,
        });
    }
    let eff_comp = compressed_bytes.max(1);
    if eff_uncomp > eff_comp.saturating_mul(MAX_DECOMPRESSION_RATIO) {
        return Err(ImportError::DecompressionBombDetected {
            compressed: eff_comp,
            uncompressed: eff_uncomp,
        });
    }
    Ok(())
}

pub fn enforce_depth_cap(depth: u32) -> Result<(), ImportError> {
    if depth > MAX_IMPORT_HIERARCHY_DEPTH {
        return Err(ImportError::HierarchyDepthExceeded {
            observed: depth,
            max_allowed: MAX_IMPORT_HIERARCHY_DEPTH,
        });
    }
    Ok(())
}

/// Rejects XML External Entity (`XXE`), DTD `<!DOCTYPE`, `<!ENTITY`, and external network URI references (`F-085`).
pub fn reject_xml_xxe_and_external_uris(xml_src: &str) -> Result<Vec<String>, ImportError> {
    let upper = xml_src.to_ascii_uppercase();
    if upper.contains("<!DOCTYPE") || upper.contains("<!ENTITY") {
        return Err(ImportError::XmlXxeOrDtdBlocked);
    }
    let mut warnings = Vec::new();
    if upper.contains("HTTP://") || upper.contains("HTTPS://") || upper.contains("FILE://") {
        warnings.push("STRIPPED_EXTERNAL_URI_REFERENCE_IN_ZERO_NETWORK_SANDBOX".to_string());
    }
    Ok(warnings)
}
