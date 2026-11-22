// Module: engine/crates/signal-import/src/pcb_sch_parsers.rs — KiCad, Altium, Eagle XML, OrCAD/EDIF & LTspice Parsers (I-10)
// Invariants: Zero unsafe code, zero network access, hard nesting depth <= 64, and XML DTD/XXE rejection (F-062, F-085).
// Spec: architecture/15-application-security-and-project-isolation.md, architecture/16-*

use crate::ImportError;
use crate::guards::{ParsedLegacyDesign, enforce_depth_cap, reject_xml_xxe_and_external_uris};

/// Parses KiCad `v6–v9` S-expression schematics/PCBs (`.kicad_sch` / `.kicad_pcb`) with `depth <= 64`.
pub fn parse_kicad_sexpr(src: &str) -> Result<ParsedLegacyDesign, ImportError> {
    let mut depth = 0_u32;
    let mut max_depth = 0_u32;
    let mut in_string = false;
    let mut entities = Vec::new();
    let mut nets = Vec::new();
    let mut current_atom = String::new();
    let mut expect_tag_kind: Option<&'static str> = None;

    for ch in src.chars() {
        if ch == '"' {
            in_string = !in_string;
            continue;
        }
        if in_string {
            current_atom.push(ch);
            continue;
        }
        match ch {
            '(' => {
                depth += 1;
                enforce_depth_cap(depth)?;
                max_depth = max_depth.max(depth);
                current_atom.clear();
            }
            ')' => {
                flush_kicad_atom(
                    &mut current_atom,
                    &mut expect_tag_kind,
                    &mut entities,
                    &mut nets,
                );
                if depth == 0 {
                    return Err(ImportError::MalformedCadSyntax(
                        "unbalanced `)` in KiCad S-expr",
                    ));
                }
                depth -= 1;
            }
            c if c.is_whitespace() => {
                flush_kicad_atom(
                    &mut current_atom,
                    &mut expect_tag_kind,
                    &mut entities,
                    &mut nets,
                );
            }
            c => current_atom.push(c),
        }
    }
    if depth != 0 {
        return Err(ImportError::MalformedCadSyntax(
            "unclosed `(` in KiCad S-expr",
        ));
    }
    Ok(ParsedLegacyDesign {
        entities,
        nets,
        max_depth_observed: max_depth.max(1),
        quarantine_warnings: Vec::new(),
    })
}

fn flush_kicad_atom(
    atom: &mut String,
    expect_kind: &mut Option<&'static str>,
    entities: &mut Vec<String>,
    nets: &mut Vec<String>,
) {
    if atom.is_empty() {
        return;
    }
    let val = std::mem::take(atom);
    if let Some(kind) = expect_kind.take() {
        match kind {
            "ENT" => entities.push(val),
            "NET" => nets.push(val),
            _ => {}
        }
        return;
    }
    match val.as_str() {
        "symbol" | "footprint" | "module" | "comp" => *expect_kind = Some("ENT"),
        "net" | "global_label" | "label" => *expect_kind = Some("NET"),
        _ => {}
    }
}

/// Parses Altium ASCII `.SchDoc` / `.PcbDoc` pipe-delimited record streams (`|RECORD=1|DESIGNATOR=U1|...`).
pub fn parse_altium_records(src: &str) -> Result<ParsedLegacyDesign, ImportError> {
    let mut entities = Vec::new();
    let mut nets = Vec::new();
    let mut max_depth = 1_u32;

    for line in src.lines() {
        let trimmed = line.trim();
        if !trimmed.contains("RECORD=") {
            continue;
        }
        let mut rec_id = "";
        let mut designator = None;
        let mut net_label = None;
        for kv in trimmed.split('|').filter(|s| !s.is_empty()) {
            if let Some((k, v)) = kv.split_once('=') {
                match k.trim().to_ascii_uppercase().as_str() {
                    "RECORD" => rec_id = v.trim(),
                    "DESIGNATOR" | "SOURCEDESIGNATOR" => designator = Some(v.trim().to_string()),
                    "NET" | "TEXT" => net_label = Some(v.trim().to_string()),
                    "OWNERINDEX" => max_depth = max_depth.max(2),
                    _ => {}
                }
            }
        }
        if rec_id == "1"
            && let Some(d) = designator
        {
            entities.push(d);
        } else if matches!(rec_id, "25" | "27")
            && let Some(n) = net_label
        {
            nets.push(n);
        }
    }
    if entities.is_empty() && nets.is_empty() {
        return Err(ImportError::MalformedCadSyntax(
            "no valid |RECORD=...| lines in Altium payload",
        ));
    }
    Ok(ParsedLegacyDesign {
        entities,
        nets,
        max_depth_observed: max_depth,
        quarantine_warnings: Vec::new(),
    })
}

/// Parses Eagle `.sch` / `.brd` XML after hard-blocking `<!DOCTYPE` / `<!ENTITY` XXE (`F-085`).
pub fn parse_eagle_xml(src: &str) -> Result<ParsedLegacyDesign, ImportError> {
    let warnings = reject_xml_xxe_and_external_uris(src)?;
    let mut depth = 0_u32;
    let mut max_depth = 0_u32;
    let mut entities = Vec::new();
    let mut nets = Vec::new();

    for raw_tag in src.split('<').skip(1) {
        let Some((tag_body, _)) = raw_tag.split_once('>') else {
            return Err(ImportError::MalformedCadSyntax(
                "unterminated XML tag in Eagle file",
            ));
        };
        let t = tag_body.trim();
        if t.starts_with('?') || t.starts_with('!') {
            continue;
        }
        if t.starts_with('/') {
            depth = depth.saturating_sub(1);
            continue;
        }
        let self_closing = t.ends_with('/');
        if !self_closing {
            depth += 1;
            enforce_depth_cap(depth)?;
            max_depth = max_depth.max(depth);
        }
        let tag_name = t
            .split_whitespace()
            .next()
            .unwrap_or_default()
            .trim_end_matches('/');
        if matches!(tag_name, "part" | "element")
            && let Some(name) = extract_xml_attr(t, "name")
        {
            entities.push(name);
        } else if matches!(tag_name, "net" | "signal")
            && let Some(name) = extract_xml_attr(t, "name")
        {
            nets.push(name);
        }
    }

    Ok(ParsedLegacyDesign {
        entities,
        nets,
        max_depth_observed: max_depth.max(1),
        quarantine_warnings: warnings,
    })
}

fn extract_xml_attr(tag_body: &str, attr_key: &str) -> Option<String> {
    let pat = format!("{attr_key}=\"");
    let start = tag_body.find(&pat)? + pat.len();
    let rest = &tag_body[start..];
    let end = rest.find('"')?;
    Some(rest[..end].to_string())
}

/// Parses OrCAD / EDIF 2 0 0 `(edif ... (instance ...) (net ...))` netlists (`depth <= 64`).
pub fn parse_orcad_edif(src: &str) -> Result<ParsedLegacyDesign, ImportError> {
    let mut base = parse_kicad_sexpr(src)?;
    for token_pair in src.split('(') {
        let words: Vec<&str> = token_pair
            .split(|c: char| c.is_whitespace() || c == ')')
            .filter(|s| !s.is_empty())
            .collect();
        if words.len() >= 2 {
            if words[0].eq_ignore_ascii_case("instance") {
                base.entities.push(words[1].to_string());
            } else if words[0].eq_ignore_ascii_case("net") {
                base.nets.push(words[1].to_string());
            }
        }
    }
    Ok(base)
}

/// Parses LTspice `.asc` schematic decks (`SYMBOL`, `SYMATTR InstName`, `FLAG`).
pub fn parse_ltspice_asc(src: &str) -> Result<ParsedLegacyDesign, ImportError> {
    let mut entities = Vec::new();
    let mut nets = Vec::new();
    for line in src.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        match parts.as_slice() {
            ["SYMATTR", "InstName", refdes, ..] => entities.push((*refdes).to_string()),
            ["FLAG", _x, _y, net_name, ..] => nets.push((*net_name).to_string()),
            _ => {}
        }
    }
    if entities.is_empty() && nets.is_empty() {
        return Err(ImportError::MalformedCadSyntax(
            "empty or invalid LTspice .asc deck",
        ));
    }
    Ok(ParsedLegacyDesign {
        entities,
        nets,
        max_depth_observed: 1,
        quarantine_warnings: Vec::new(),
    })
}
