// Module: engine/crates/signal-sim/src/spice_loader.rs — SPICE Subcircuit Deck Loader & pnjlim Junction Damper
// Invariants: Bridges signal-module-sdk flattened SPICE elements into MnaCircuit; C^1 pnjlim step limiter (F-025, F-029).
// Spec: architecture/03-rust-eda-engine.md
use crate::circuit::MnaCircuit;
use signal_core::{det_ln, parse_spice_si};
use signal_lib_semiconductor::ShockleyDiode;
use signal_module_sdk::{SpiceDialect, flatten_netlist_hierarchy, parse_spice_netlist};
use std::collections::BTreeMap;

/// Classic SPICE `pnjlim` junction Newton step damper preventing exponential blow-up (`F-025`).
pub fn pnjlim(v_new: f64, v_old: f64, vt: f64, v_crit: f64) -> f64 {
    if v_new > v_crit && (v_new - v_old).abs() > 2.0 * vt {
        if v_old > 0.0 {
            let arg = 1.0 + (v_new - v_old) / vt;
            if arg > 0.0 {
                v_old + vt * det_ln(arg)
            } else {
                v_crit
            }
        } else {
            vt * det_ln(v_new / vt)
        }
    } else {
        v_new
    }
}

pub(crate) fn load_mna_from_spice_deck(
    src: &str,
) -> Result<(MnaCircuit, BTreeMap<String, usize>), String> {
    let parsed = parse_spice_netlist(src, SpiceDialect::Ngspice).map_err(|e| e.to_string())?;
    let flat = flatten_netlist_hierarchy(&parsed).map_err(|e| e.to_string())?;
    let mut node_map = BTreeMap::new();
    node_map.insert("0".to_string(), 0_usize);
    for el in &flat {
        for n in &el.nodes {
            if !node_map.contains_key(n) {
                let nxt = node_map.len();
                node_map.insert(n.clone(), nxt);
            }
        }
    }
    let mut ckt = MnaCircuit::new(node_map.len().saturating_sub(1));
    for el in &flat {
        let n0 = node_map[&el.nodes[0]];
        let n1 = node_map[&el.nodes[1]];
        let val = el
            .numeric_value
            .or_else(|| parse_spice_si(&el.model_or_value).ok())
            .unwrap_or(1.0);
        match el.primitive {
            'R' => ckt.add_resistor(&el.instance_name, n0, n1, val),
            'C' => ckt.add_capacitor(&el.instance_name, n0, n1, val),
            'L' => {
                let _ = ckt.add_inductor(&el.instance_name, n0, n1, val, 1.0e-3);
            }
            'V' => {
                let _ = ckt.add_dc_ac_vsource(&el.instance_name, n0, n1, val, 0.0);
            }
            'D' => ckt.add_diode(&el.instance_name, n0, n1, ShockleyDiode::silicon_1n4148()),
            _ => {}
        }
    }
    Ok((ckt, node_map))
}
