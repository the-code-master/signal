// Module: engine/crates/signal-graph/src/hierarchy_ops.rs — Cross-Sheet Portals, Repeat(N) Channels & Assembly Variants
// Invariants: Deterministic portal pairing (FEAT-005), Repeat(N) refdes/net expansion (FEAT-006), DNP/MPN variants (FEAT-007).
// Spec: architecture/01-product-vision-and-features.md, architecture/17-extended-feature-specifications-ui-and-rust.md
use crate::GraphError;
use signal_geom::Point2Nm;
use signal_proto::v1::{AssemblyVariantEntry, MultiChannelRepeatRule, Point2dNm, SheetPortalEntry};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SheetNetPort {
    pub sheet_id: String,
    pub net_name: String,
    pub anchor_nm: Point2Nm,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChannelExpandedInstance {
    pub channel_index: u32,
    pub expanded_refdes: String,
    pub expanded_pin_nets: BTreeMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedBomVariantItem {
    pub entity_id: String,
    pub populated: bool,
    pub effective_mpn: String,
}

/// Builds sorted cross-sheet net portal jump entries (`FEAT-005`).
pub fn build_cross_sheet_portal_index(ports: &[SheetNetPort]) -> Vec<SheetPortalEntry> {
    let mut by_net: BTreeMap<&str, Vec<&SheetNetPort>> = BTreeMap::new();
    for p in ports {
        by_net.entry(p.net_name.as_str()).or_default().push(p);
    }

    let mut entries = Vec::new();
    for (net, mut group) in by_net {
        group.sort_by_key(|p| p.sheet_id.as_str());
        for i in 0..group.len() {
            for j in 0..group.len() {
                if i != j && group[i].sheet_id != group[j].sheet_id {
                    entries.push(SheetPortalEntry {
                        net_name: net.to_string(),
                        source_sheet_id: group[i].sheet_id.clone(),
                        target_sheet_id: group[j].sheet_id.clone(),
                        portal_anchor_nm: Some(Point2dNm {
                            x_nm: group[j].anchor_nm.x.0,
                            y_nm: group[j].anchor_nm.y.0,
                        }),
                    });
                }
            }
        }
    }
    entries
}

fn split_refdes(refdes: &str) -> (&str, u32) {
    let prefix_end = refdes
        .find(|c: char| c.is_ascii_digit())
        .unwrap_or(refdes.len());
    let num = refdes[prefix_end..].parse::<u32>().unwrap_or(1);
    (&refdes[..prefix_end], num)
}

/// Expands a `Repeat(N)` multi-channel block template across `1..=repeat_count` (`FEAT-006`).
pub fn expand_multi_channel_block(
    rule: &MultiChannelRepeatRule,
    template_instances: &[(String, BTreeMap<String, String>)],
    global_rails: &BTreeSet<String>,
) -> Result<Vec<ChannelExpandedInstance>, GraphError> {
    if rule.repeat_count == 0 || rule.repeat_count > 256 {
        return Err(GraphError::InvalidCommandOrSlice(
            "Repeat(N) count must be in 1..=256".to_string(),
        ));
    }
    let offset_step = rule
        .refdes_offset_rule
        .trim_start_matches('+')
        .parse::<u32>()
        .unwrap_or(100);

    let mut out = Vec::with_capacity((rule.repeat_count as usize) * template_instances.len());
    for ch in 1..=rule.repeat_count {
        for (base_refdes, pin_nets) in template_instances {
            let (prefix, base_num) = split_refdes(base_refdes);
            let new_refdes = format!("{prefix}{}", ch * offset_step + base_num);
            let mut ch_nets = BTreeMap::new();
            for (pin, net) in pin_nets {
                let expanded_net = if global_rails.contains(net) {
                    net.clone()
                } else if rule.net_naming_template.contains("{net}")
                    && rule.net_naming_template.contains("{ch}")
                {
                    rule.net_naming_template
                        .replace("{net}", net)
                        .replace("{ch}", &ch.to_string())
                } else {
                    format!("{net}_CH{ch}")
                };
                ch_nets.insert(pin.clone(), expanded_net);
            }
            out.push(ChannelExpandedInstance {
                channel_index: ch,
                expanded_refdes: new_refdes,
                expanded_pin_nets: ch_nets,
            });
        }
    }
    Ok(out)
}

/// Resolves populated/DNP status and effective MPNs for a target `variant_id` (`FEAT-007`).
pub fn resolve_assembly_variant(
    base_entities_mpn: &[(String, String)],
    overrides: &[AssemblyVariantEntry],
    active_variant_id: &str,
) -> Vec<ResolvedBomVariantItem> {
    let mut ov_map: BTreeMap<&str, &AssemblyVariantEntry> = BTreeMap::new();
    for ov in overrides {
        if ov.variant_id == active_variant_id {
            ov_map.insert(ov.entity_id.as_str(), ov);
        }
    }

    base_entities_mpn
        .iter()
        .map(|(entity_id, default_mpn)| {
            if let Some(ov) = ov_map.get(entity_id.as_str()) {
                ResolvedBomVariantItem {
                    entity_id: entity_id.clone(),
                    populated: !ov.do_not_populate,
                    effective_mpn: if ov.alternate_mpn.is_empty() {
                        default_mpn.clone()
                    } else {
                        ov.alternate_mpn.clone()
                    },
                }
            } else {
                ResolvedBomVariantItem {
                    entity_id: entity_id.clone(),
                    populated: true,
                    effective_mpn: default_mpn.clone(),
                }
            }
        })
        .collect()
}
