// Module: engine/crates/signal-module-sdk/src/spice_parser.rs — Multi-Dialect SPICE / Spectre Netlist & Subckt Parser
// Invariants: Parses NGSPICE/LTspice/PSpice/Spectre cards (.model, .subckt, R/C/L/V/I/D/M/Q/X); caps subckt depth <= 64 (F-029).
// Spec: architecture/03-rust-eda-engine.md
use crate::SdkError;
use signal_core::parse_spice_si;
use std::collections::BTreeMap;

const MAX_SUBCKT_DEPTH: usize = 64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpiceDialect {
    Ngspice,
    Ltspice,
    Pspice,
    Spectre,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SpiceModelCard {
    pub name: String,
    pub family: String,
    pub params: BTreeMap<String, f64>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SpiceElement {
    pub instance_name: String,
    pub primitive: char,
    pub nodes: Vec<String>,
    pub model_or_value: String,
    pub numeric_value: Option<f64>,
    pub params: BTreeMap<String, f64>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SpiceSubcktDef {
    pub name: String,
    pub ports: Vec<String>,
    pub elements: Vec<SpiceElement>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParsedNetlist {
    pub dialect: SpiceDialect,
    pub models: BTreeMap<String, SpiceModelCard>,
    pub subckts: BTreeMap<String, SpiceSubcktDef>,
    pub top_elements: Vec<SpiceElement>,
}

fn fold_continuations(src: &str) -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();
    for raw in src.lines() {
        let trimmed = raw.trim();
        if trimmed.is_empty()
            || trimmed.starts_with('*')
            || trimmed.starts_with(';')
            || trimmed.starts_with("//")
        {
            continue;
        }
        let clean = trimmed.split(';').next().unwrap_or(trimmed).trim();
        if let Some(cont) = clean.strip_prefix('+') {
            if let Some(last) = lines.last_mut() {
                last.push(' ');
                last.push_str(cont.trim());
            }
        } else {
            lines.push(clean.to_string());
        }
    }
    lines
}

fn parse_kv_params(tokens: &[&str]) -> BTreeMap<String, f64> {
    let mut map = BTreeMap::new();
    for tok in tokens {
        let cleaned = tok.trim_matches(|c| c == '(' || c == ')');
        if let Some((k, v)) = cleaned.split_once('=')
            && let Ok(val) = parse_spice_si(v.trim())
        {
            map.insert(k.trim().to_ascii_lowercase(), val);
        }
    }
    map
}

fn parse_element_line(line: &str, dialect: SpiceDialect) -> Result<SpiceElement, SdkError> {
    // Handle Spectre syntax: `R1 (inp out) resistor r=10k`
    if dialect == SpiceDialect::Spectre && line.contains('(') && line.contains(')') {
        let (before, rest) = line
            .split_once('(')
            .ok_or_else(|| SdkError::SpiceParseError(line.to_string()))?;
        let (nodes_str, after) = rest
            .split_once(')')
            .ok_or_else(|| SdkError::SpiceParseError(line.to_string()))?;
        let inst = before.trim().to_string();
        let nodes: Vec<String> = nodes_str
            .split_whitespace()
            .map(ToString::to_string)
            .collect();
        let after_toks: Vec<&str> = after.split_whitespace().collect();
        let dev_type = (*after_toks.first().unwrap_or(&"resistor")).to_ascii_lowercase();
        let prim = match dev_type.as_str() {
            "resistor" => 'R',
            "capacitor" => 'C',
            "inductor" => 'L',
            "vsource" => 'V',
            "isource" => 'I',
            "diode" => 'D',
            "bsim4" | "mos1" | "nmos" | "pmos" => 'M',
            "bjt" => 'Q',
            _ => inst.chars().next().unwrap_or('X').to_ascii_uppercase(),
        };
        let params = parse_kv_params(&after_toks[1..]);
        let num_val = params
            .get("r")
            .or_else(|| params.get("c"))
            .or_else(|| params.get("l"))
            .or_else(|| params.get("dc"))
            .copied();
        return Ok(SpiceElement {
            instance_name: inst,
            primitive: prim,
            nodes,
            model_or_value: dev_type,
            numeric_value: num_val,
            params,
        });
    }

    let toks: Vec<&str> = line.split_whitespace().collect();
    let inst = (*toks
        .first()
        .ok_or_else(|| SdkError::SpiceParseError("empty card".to_string()))?)
    .to_string();
    let prim = inst.chars().next().unwrap_or('X').to_ascii_uppercase();
    let node_arity = match prim {
        'R' | 'C' | 'L' | 'V' | 'I' | 'D' => 2,
        'Q' => 3,
        'M' => 4,
        'X' => {
            // In `X1 n1 n2 ... subckt_name [k=v ...]`, find last non-kv token as subckt_name
            let non_kv_count = toks[1..].iter().take_while(|t| !t.contains('=')).count();
            non_kv_count.saturating_sub(1)
        }
        _ => 2,
    };

    if toks.len() < 1 + node_arity + 1 {
        return Err(SdkError::SpiceParseError(format!(
            "insufficient tokens in `{line}`"
        )));
    }
    let nodes: Vec<String> = toks[1..1 + node_arity]
        .iter()
        .map(|s| (*s).to_string())
        .collect();
    let val_tok = toks[1 + node_arity];
    let numeric_value = parse_spice_si(val_tok).ok();
    let params = parse_kv_params(&toks[2 + node_arity..]);

    Ok(SpiceElement {
        instance_name: inst,
        primitive: prim,
        nodes,
        model_or_value: val_tok.to_string(),
        numeric_value,
        params,
    })
}

/// Parses NGSPICE / LTspice / PSpice / Spectre netlists & `.subckt` hierarchies (`F-029`).
pub fn parse_spice_netlist(src: &str, dialect: SpiceDialect) -> Result<ParsedNetlist, SdkError> {
    let lines = fold_continuations(src);
    let mut models = BTreeMap::new();
    let mut subckts = BTreeMap::new();
    let mut top_elements = Vec::new();
    let mut active_subckt: Option<SpiceSubcktDef> = None;

    for line in lines {
        let lower = line.to_ascii_lowercase();
        if lower.starts_with("simulator ")
            || lower == ".end"
            || lower.starts_with(".tran")
            || lower.starts_with(".op")
        {
            continue;
        }
        if lower.starts_with(".model ") {
            let toks: Vec<&str> = line.split_whitespace().collect();
            if toks.len() >= 3 {
                let name = toks[1].to_string();
                let fam = toks[2]
                    .trim_matches(|c| c == '(' || c == ')')
                    .to_ascii_uppercase();
                let params = parse_kv_params(&toks[2..]);
                models.insert(
                    name.clone(),
                    SpiceModelCard {
                        name,
                        family: fam,
                        params,
                    },
                );
            }
            continue;
        }
        if lower.starts_with(".subckt ") {
            let toks: Vec<&str> = line.split_whitespace().collect();
            if toks.len() < 3 {
                return Err(SdkError::SpiceParseError(
                    "malformed .subckt header".to_string(),
                ));
            }
            let ports = toks[2..]
                .iter()
                .take_while(|t| !t.contains('='))
                .map(|s| (*s).to_string())
                .collect();
            active_subckt = Some(SpiceSubcktDef {
                name: toks[1].to_string(),
                ports,
                elements: Vec::new(),
            });
            continue;
        }
        if lower.starts_with(".ends") {
            if let Some(def) = active_subckt.take() {
                subckts.insert(def.name.clone(), def);
            }
            continue;
        }

        let elem = parse_element_line(&line, dialect)?;
        if let Some(sub) = &mut active_subckt {
            sub.elements.push(elem);
        } else {
            top_elements.push(elem);
        }
    }

    Ok(ParsedNetlist {
        dialect,
        models,
        subckts,
        top_elements,
    })
}

/// Flattens `X` subcircuit instances into primitive elements with bomb-cap depth `<= 64` (`F-029`).
pub fn flatten_netlist_hierarchy(netlist: &ParsedNetlist) -> Result<Vec<SpiceElement>, SdkError> {
    let mut out = Vec::new();
    flatten_rec(
        &netlist.top_elements,
        &netlist.subckts,
        "",
        &BTreeMap::new(),
        0,
        &mut out,
    )?;
    Ok(out)
}

fn flatten_rec(
    elems: &[SpiceElement],
    subckts: &BTreeMap<String, SpiceSubcktDef>,
    prefix: &str,
    port_map: &BTreeMap<String, String>,
    depth: usize,
    out: &mut Vec<SpiceElement>,
) -> Result<(), SdkError> {
    if depth > MAX_SUBCKT_DEPTH {
        return Err(SdkError::SpiceParseError(
            "subckt hierarchy depth > 64".to_string(),
        ));
    }
    let map_node = |n: &str| -> String {
        if n == "0" || n.eq_ignore_ascii_case("gnd") {
            "0".to_string()
        } else if let Some(mapped) = port_map.get(n) {
            mapped.clone()
        } else if prefix.is_empty() {
            n.to_string()
        } else {
            format!("{prefix}.{n}")
        }
    };

    for el in elems {
        let fq_name = if prefix.is_empty() {
            el.instance_name.clone()
        } else {
            format!("{prefix}.{}", el.instance_name)
        };
        if el.primitive == 'X' {
            let def = subckts.get(&el.model_or_value).ok_or_else(|| {
                SdkError::SpiceParseError(format!("unknown subckt `{}`", el.model_or_value))
            })?;
            if def.ports.len() != el.nodes.len() {
                return Err(SdkError::SpiceParseError(format!(
                    "port count mismatch on `{fq_name}`"
                )));
            }
            let mut child_map = BTreeMap::new();
            for (formal, actual) in def.ports.iter().zip(&el.nodes) {
                child_map.insert(formal.clone(), map_node(actual));
            }
            flatten_rec(&def.elements, subckts, &fq_name, &child_map, depth + 1, out)?;
        } else {
            let mut cloned = el.clone();
            cloned.instance_name = fq_name;
            cloned.nodes = el.nodes.iter().map(|n| map_node(n)).collect();
            out.push(cloned);
        }
    }
    Ok(())
}
