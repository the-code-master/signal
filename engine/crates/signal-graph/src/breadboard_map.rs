// Module: engine/crates/signal-graph/src/breadboard_map.rs — 5-Hole Strip & Power Bus Breadboard Twin Bijection (FEAT-111)
// Invariants: Enforces 5-hole terminal strip (`A..E` / `F..J`) + power rail connectivity & detects net/rail shorts.
// Spec: architecture/01-product-vision-and-features.md, architecture/17-extended-feature-specifications-ui-and-rust.md
use signal_proto::v1::{
    BreadboardStripMapping, SyncBreadboardTwinRequest, SyncBreadboardTwinResponse,
};
use std::collections::{BTreeMap, BTreeSet};

pub const TERMINAL_STRIP_HOLES: u32 = 5;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StripKind {
    TerminalFiveHole { row: u32, top_bank: bool }, // A..E (0..5) vs F..J (0..5) across IC ravine
    PowerBusRail { positive: bool, top_edge: bool },
}

/// Parses canonical strip IDs (`"T_R12_AE"`, `"T_R12_FJ"`, `"RAIL_TOP_VCC"`, `"RAIL_BOT_GND"`).
pub fn classify_breadboard_strip(strip_id: &str) -> StripKind {
    let up = strip_id.trim().to_ascii_uppercase();
    if up.contains("RAIL") || up.contains("BUS") {
        let positive = up.contains("VCC") || up.contains("PLUS") || up.contains("POS");
        let top_edge = !up.contains("BOT");
        return StripKind::PowerBusRail { positive, top_edge };
    }
    let top_bank = !up.ends_with("FJ");
    let digits: String = up.chars().filter(char::is_ascii_digit).collect();
    let row = digits.parse::<u32>().unwrap_or(1);
    StripKind::TerminalFiveHole { row, top_bank }
}

fn is_power_net(net: &str) -> bool {
    matches!(
        net.trim().to_ascii_uppercase().as_str(),
        "VCC" | "VDD" | "5V" | "3V3" | "VBUS"
    )
}

fn is_ground_net(net: &str) -> bool {
    matches!(
        net.trim().to_ascii_uppercase().as_str(),
        "GND" | "VSS" | "AGND" | "DGND"
    )
}

/// Synchronizes Breadboard Twin strip occupancy against schematic net connectivity (`FEAT-111`),
/// detecting multi-hole collisions, >5-hole overflows, distinct-net strip shorts, and `VCC`↔`GND` shorts.
pub fn sync_breadboard_twin(
    req: &SyncBreadboardTwinRequest,
    current_revision: u64,
) -> (
    SyncBreadboardTwinResponse,
    BTreeMap<String, BTreeSet<String>>,
) {
    let mut strip_nets: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    let mut strip_holes_used: BTreeMap<String, BTreeSet<u32>> = BTreeMap::new();
    let mut short_detected = false;

    for m in &req.strips {
        let kind = classify_breadboard_strip(&m.strip_id);
        // Check hole bounds & double-insertion into the exact same physical hole
        let holes_set = strip_holes_used.entry(m.strip_id.clone()).or_default();
        for &hole_idx in &m.occupied_hole_indices {
            if matches!(kind, StripKind::TerminalFiveHole { .. })
                && hole_idx >= TERMINAL_STRIP_HOLES
            {
                short_detected = true;
            }
            if !holes_set.insert(hole_idx) {
                short_detected = true;
            }
        }

        // Check rail polarity short (e.g. GND wired to positive bus rail or VCC wired to negative rail)
        if let StripKind::PowerBusRail { positive, .. } = kind
            && ((positive && is_ground_net(&m.net_name))
                || (!positive && is_power_net(&m.net_name)))
        {
            short_detected = true;
        }

        let nets_on_strip = strip_nets.entry(m.strip_id.clone()).or_default();
        if !m.net_name.trim().is_empty() {
            nets_on_strip.insert(m.net_name.trim().to_string());
        }
        // Two distinct schematic nets plugged into the same electrically-common 5-hole strip or rail = SHORT!
        if nets_on_strip.len() > 1 {
            short_detected = true;
        }
    }

    let next_rev = if short_detected {
        current_revision
    } else {
        current_revision + 1
    };

    (
        SyncBreadboardTwinResponse {
            short_circuit_detected: short_detected,
            synced_revision: next_rev,
        },
        strip_nets,
    )
}

/// Synthesizes canonical 5-hole terminal strip assignments from schematic net pin counts (`FEAT-111`).
pub fn build_breadboard_strips_from_netlist(
    net_pin_counts: &BTreeMap<String, u32>,
) -> Vec<BreadboardStripMapping> {
    let mut out = Vec::new();
    let mut next_row = 1_u32;

    for (net, &pins) in net_pin_counts {
        if pins == 0 {
            continue;
        }
        if is_power_net(net) {
            out.push(BreadboardStripMapping {
                strip_id: "RAIL_TOP_VCC".to_string(),
                net_name: net.clone(),
                occupied_hole_indices: (0..pins.min(25)).collect(),
            });
            continue;
        }
        if is_ground_net(net) {
            out.push(BreadboardStripMapping {
                strip_id: "RAIL_BOT_GND".to_string(),
                net_name: net.clone(),
                occupied_hole_indices: (0..pins.min(25)).collect(),
            });
            continue;
        }

        let mut rem = pins;
        while rem > 0 {
            let take = rem.min(TERMINAL_STRIP_HOLES);
            out.push(BreadboardStripMapping {
                strip_id: format!("T_R{next_row}_AE"),
                net_name: net.clone(),
                occupied_hole_indices: (0..take).collect(),
            });
            rem -= take;
            next_row += 1;
        }
    }
    out
}
