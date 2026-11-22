// Module: engine/crates/signal-import/src/ic_parsers.rs — Clean-Room GDSII Binary Stream & LEF/DEF IC Parsers (I-10)
// Invariants: Validates 4-byte big-endian GDSII record frames & SREF cell depth <= 64; parses LEF/DEF macros & nets.
// Spec: architecture/03-rust-eda-engine.md, architecture/15-application-security-and-project-isolation.md

use crate::ImportError;
use crate::guards::{ParsedLegacyDesign, enforce_depth_cap};

/// Parses GDSII binary stream records (`[len_hi, len_lo, rec_type, data_type, payload...]`, `I-10`):
/// - `0x05` `BGNSTR`, `0x06` `STRNAME`, `0x07` `ENDSTR`, `0x08` `BOUNDARY`, `0x09` `PATH`, `0x0A` `SREF`.
pub fn parse_gdsii_stream(bytes: &[u8]) -> Result<ParsedLegacyDesign, ImportError> {
    if bytes.len() < 4 {
        return Err(ImportError::MalformedCadSyntax(
            "GDSII stream shorter than 4-byte record header",
        ));
    }
    let mut offset = 0_usize;
    let mut entities = Vec::new();
    let mut nets = Vec::new();
    let mut struct_depth = 0_u32;
    let mut max_depth = 1_u32;

    while offset < bytes.len() {
        if offset + 4 > bytes.len() {
            return Err(ImportError::MalformedCadSyntax(
                "truncated GDSII record header",
            ));
        }
        let rec_len = u16::from_be_bytes([bytes[offset], bytes[offset + 1]]) as usize;
        if rec_len < 4 || offset + rec_len > bytes.len() {
            return Err(ImportError::MalformedCadSyntax(
                "invalid GDSII record byte length",
            ));
        }
        let rec_type = bytes[offset + 2];
        let payload = &bytes[offset + 4..offset + rec_len];

        match rec_type {
            0x05 => {
                // BGNSTR
                struct_depth += 1;
                enforce_depth_cap(struct_depth)?;
                max_depth = max_depth.max(struct_depth);
            }
            0x06 => {
                // STRNAME (ASCII padded with optional NUL)
                let name = String::from_utf8_lossy(payload)
                    .trim_matches('\0')
                    .trim()
                    .to_string();
                if !name.is_empty() {
                    entities.push(format!("CELL:{name}"));
                }
            }
            0x07 => {
                // ENDSTR
                struct_depth = struct_depth.saturating_sub(1);
            }
            0x08 | 0x09 => {
                // BOUNDARY / PATH polygon geometry
                entities.push(format!("POLY_AT_{offset}"));
            }
            0x0A => {
                // SREF hierarchical instance
                let d = struct_depth + 1;
                enforce_depth_cap(d)?;
                max_depth = max_depth.max(d);
                entities.push(format!("SREF_AT_{offset}"));
            }
            0x12 => {
                // SNAME referenced structure name
                let sname = String::from_utf8_lossy(payload)
                    .trim_matches('\0')
                    .trim()
                    .to_string();
                if !sname.is_empty() {
                    nets.push(format!("REF:{sname}"));
                }
            }
            _ => {}
        }
        offset += rec_len;
    }

    if entities.is_empty() {
        return Err(ImportError::MalformedCadSyntax(
            "GDSII stream contained zero structures or shapes",
        ));
    }
    Ok(ParsedLegacyDesign {
        entities,
        nets,
        max_depth_observed: max_depth,
        quarantine_warnings: Vec::new(),
    })
}

/// Parses ASIC LEF/DEF ASCII physical exchange files (`MACRO`, `COMPONENTS`, `NETS`, `I-10`).
pub fn parse_lef_def(src: &str) -> Result<ParsedLegacyDesign, ImportError> {
    let mut entities = Vec::new();
    let mut nets = Vec::new();
    let mut section = "";

    for raw_line in src.lines() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let upper = line.to_ascii_uppercase();
        if upper.starts_with("COMPONENTS ") {
            section = "COMPONENTS";
            continue;
        }
        if upper.starts_with("END COMPONENTS") || upper.starts_with("END NETS") {
            section = "";
            continue;
        }
        if upper.starts_with("NETS ") {
            section = "NETS";
            continue;
        }
        if let Some(rest) = line.strip_prefix("MACRO ") {
            let name = rest
                .split_whitespace()
                .next()
                .unwrap_or_default()
                .trim_end_matches(';');
            if !name.is_empty() {
                entities.push(format!("MACRO:{name}"));
            }
            continue;
        }
        if let Some(rest) = line.strip_prefix("- ") {
            let ident = rest.split_whitespace().next().unwrap_or_default();
            if !ident.is_empty() {
                match section {
                    "COMPONENTS" => entities.push(ident.to_string()),
                    "NETS" => nets.push(ident.to_string()),
                    _ => {}
                }
            }
        }
    }

    if entities.is_empty() && nets.is_empty() {
        return Err(ImportError::MalformedCadSyntax(
            "no MACRO/COMPONENTS/NETS found in LEF/DEF",
        ));
    }
    Ok(ParsedLegacyDesign {
        entities,
        nets,
        max_depth_observed: 2,
        quarantine_warnings: Vec::new(),
    })
}
