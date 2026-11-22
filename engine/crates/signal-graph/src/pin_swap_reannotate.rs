// Module: engine/crates/signal-graph/src/pin_swap_reannotate.rs — Bipartite Pin Swapper & Spatial Refdes Reannotator
// Invariants: Exact Kuhn-Munkres / bitmask DP minimum Manhattan wirelength pin swap (FEAT-008); spatial refdes sweep (FEAT-009).
// Spec: architecture/01-product-vision-and-features.md, architecture/17-extended-feature-specifications-ui-and-rust.md
use signal_geom::Point2Nm;
use signal_proto::v1::{
    PinSwapProposal, ReannotateDirection, ReannotateSheetResponse, RefdesRemapPair,
};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SwappablePinAnchor {
    pub pin_number: u32,
    pub pin_location_nm: Point2Nm,
    pub target_net_anchor_nm: Point2Nm,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnnotatableInstance {
    pub entity_id: String,
    pub current_refdes: String,
    pub sheet_index: u32,
    pub position_nm: Point2Nm,
}

/// Solves exact bipartite minimum-wirelength assignment over an equivalent pin swap group (`FEAT-008`).
pub fn propose_optimal_pin_swaps(
    entity_id: &str,
    swappable_pins: &[SwappablePinAnchor],
) -> Vec<PinSwapProposal> {
    let n = swappable_pins.len();
    if !(2..=16).contains(&n) {
        return Vec::new();
    }

    let cost = |pin_idx: usize, net_idx: usize| -> i64 {
        let d = swappable_pins[pin_idx]
            .pin_location_nm
            .manhattan_i128(swappable_pins[net_idx].target_net_anchor_nm);
        d.min(i128::from(i64::MAX / 32)) as i64
    };

    let mut original_cost = 0_i64;
    for i in 0..n {
        original_cost = original_cost.saturating_add(cost(i, i));
    }

    // Exact bitmask dynamic programming over subsets of target nets (O(N * 2^N), N <= 16)
    let states = 1_usize << n;
    let mut dp = vec![i64::MAX / 2; states];
    let mut choice = vec![0_usize; states];
    dp[0] = 0;

    for mask in 0..(states - 1) {
        let pin_idx = mask.count_ones() as usize;
        let cur = dp[mask];
        if cur >= i64::MAX / 4 {
            continue;
        }
        for net_idx in 0..n {
            if (mask & (1 << net_idx)) == 0 {
                let next_mask = mask | (1 << net_idx);
                let cand = cur + cost(pin_idx, net_idx);
                if cand < dp[next_mask] {
                    dp[next_mask] = cand;
                    choice[next_mask] = net_idx;
                }
            }
        }
    }

    let best_cost = dp[states - 1];
    if best_cost >= original_cost {
        return Vec::new();
    }

    let mut assigned_net_for_pin = vec![0_usize; n];
    let mut cur_mask = states - 1;
    for pin_idx in (0..n).rev() {
        let net_idx = choice[cur_mask];
        assigned_net_for_pin[pin_idx] = net_idx;
        cur_mask ^= 1 << net_idx;
    }

    let mut proposals = Vec::new();
    for (pin_idx, &net_idx) in assigned_net_for_pin.iter().enumerate() {
        if pin_idx != net_idx {
            let saved = (cost(pin_idx, pin_idx) - cost(pin_idx, net_idx)).max(0);
            proposals.push(PinSwapProposal {
                entity_id: entity_id.to_string(),
                from_pin_number: swappable_pins[net_idx].pin_number,
                to_pin_number: swappable_pins[pin_idx].pin_number,
                wirelength_saved_nm: saved,
            });
        }
    }
    proposals
}

fn refdes_alpha_prefix(refdes: &str) -> String {
    let end = refdes
        .find(|c: char| c.is_ascii_digit() || c == '?')
        .unwrap_or(refdes.len());
    let p = &refdes[..end];
    if p.is_empty() {
        "U".to_string()
    } else {
        p.to_string()
    }
}

/// Spatially reannotates component reference designators (`R1..Rn`, `C1..Cn`, etc.) (`FEAT-009`).
pub fn reannotate_sheet_instances(
    instances: &[AnnotatableInstance],
    direction: ReannotateDirection,
    sheet_base_step: u32,
    new_revision: u64,
    band_tolerance_nm: i64,
) -> ReannotateSheetResponse {
    let band = band_tolerance_nm.max(1);
    let mut sorted = instances.to_vec();

    sorted.sort_by_key(|inst| match direction {
        ReannotateDirection::TopDownLeftToRight => (
            inst.sheet_index,
            inst.position_nm.x.0 / band,
            inst.position_nm.y.0,
            inst.entity_id.clone(),
        ),
        ReannotateDirection::HierarchicalSheetBlock => (
            inst.sheet_index,
            inst.position_nm.y.0 / band,
            inst.position_nm.x.0,
            inst.entity_id.clone(),
        ),
        _ => (
            inst.sheet_index,
            inst.position_nm.y.0 / band,
            inst.position_nm.x.0,
            inst.entity_id.clone(),
        ),
    });

    let mut counters: BTreeMap<(u32, String), u32> = BTreeMap::new();
    let mut remapped = Vec::with_capacity(sorted.len());

    for inst in sorted {
        let prefix = refdes_alpha_prefix(&inst.current_refdes);
        let scope_sheet = if direction == ReannotateDirection::HierarchicalSheetBlock {
            inst.sheet_index.max(1)
        } else {
            0
        };
        let next_idx = counters
            .entry((scope_sheet, prefix.clone()))
            .and_modify(|c| *c += 1)
            .or_insert(1);
        let num = if direction == ReannotateDirection::HierarchicalSheetBlock {
            scope_sheet * sheet_base_step.max(100) + *next_idx
        } else {
            *next_idx
        };
        remapped.push(RefdesRemapPair {
            entity_id: inst.entity_id,
            old_refdes: inst.current_refdes,
            new_refdes: format!("{prefix}{num}"),
        });
    }

    ReannotateSheetResponse {
        committed_revision: new_revision,
        remapped,
    }
}
