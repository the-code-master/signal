// Module: engine/crates/signal-graph/src/crdt_validator.rs — Topological-Invariant CRDT Mutation Validator
// Invariants: Rejects VCC<->GND shorts and multi-push-pull driver collisions into TopologicalConflictCard (F-040, F-124).
// Spec: architecture/03-rust-eda-engine.md, architecture/16-architectural-audit-flaws-and-futuristic-innovations.md
use crate::cow_arena::{
    CoWGraphSnapshot, EcsEntity, ElectricalPinRole, GenEntityId, TopologicalConflictCard,
    TopologicalConflictKind,
};
use signal_geom::Point2Nm;
use signal_proto::v1::{GraphMutationKind, GraphMutationOp};
use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

fn is_power_rail_name(net: &str) -> bool {
    matches!(
        net.to_ascii_uppercase().as_str(),
        "VCC" | "VDD" | "VBUS" | "3V3" | "5V" | "1V8"
    )
}

fn is_ground_rail_name(net: &str) -> bool {
    matches!(
        net.to_ascii_uppercase().as_str(),
        "GND" | "VSS" | "AGND" | "DGND" | "PGND"
    )
}

pub(crate) fn apply_single_op_checked(
    snap: &mut CoWGraphSnapshot,
    op: &GraphMutationOp,
    actor: &str,
    touched: &mut BTreeSet<String>,
) -> Option<TopologicalConflictCard> {
    let kind = GraphMutationKind::try_from(op.kind).unwrap_or(GraphMutationKind::Unspecified);
    let pos = op
        .position_nm
        .as_ref()
        .map_or(Point2Nm::ORIGIN, |p| Point2Nm::new(p.x_nm, p.y_nm));

    match kind {
        GraphMutationKind::PlaceInstance | GraphMutationKind::MoveInstance => {
            let base_id = op.entity_id.split('.').next().unwrap_or(&op.entity_id);
            touched.insert(format!("ent:{base_id}"));
            let mut ent = snap.entity(base_id).cloned().unwrap_or(EcsEntity {
                slot: GenEntityId {
                    index: 0,
                    generation: 0,
                },
                entity_id: base_id.to_string(),
                component_type: if op.param_key.is_empty() {
                    "COMP".to_string()
                } else {
                    op.param_key.clone()
                },
                position_nm: pos,
                params_si: BTreeMap::new(),
                pin_roles: BTreeMap::new(),
                pin_nets: BTreeMap::new(),
            });
            ent.position_nm = pos;
            if !op.param_key.is_empty() && op.param_si_value != 0 {
                ent.params_si
                    .insert(op.param_key.clone(), op.param_si_value);
            }
            snap.upsert_entity(ent);
        }
        GraphMutationKind::UpdateParameter => {
            touched.insert(format!("ent:{}", op.entity_id));
            if let Some(mut ent) = snap.entity(&op.entity_id).cloned() {
                ent.params_si
                    .insert(op.param_key.clone(), op.param_si_value);
                snap.upsert_entity(ent);
            }
        }
        GraphMutationKind::DeleteEntity => {
            touched.insert(format!("ent:{}", op.entity_id));
            snap.remove_entity(&op.entity_id);
        }
        GraphMutationKind::ConnectNet => {
            return apply_connect_net_checked(snap, op, actor, pos, touched);
        }
        _ => {
            touched.insert(format!("ent:{}", op.entity_id));
        }
    }
    None
}

fn apply_connect_net_checked(
    snap: &mut CoWGraphSnapshot,
    op: &GraphMutationOp,
    actor: &str,
    pos: Point2Nm,
    touched: &mut BTreeSet<String>,
) -> Option<TopologicalConflictCard> {
    let (ent_id, pin_id) = op.entity_id.split_once('.').unwrap_or((&op.entity_id, "1"));
    touched.insert(format!("ent:{ent_id}"));
    touched.insert(format!("net:{}", op.net_name));

    let mut ent = snap.entity(ent_id).cloned().unwrap_or(EcsEntity {
        slot: GenEntityId {
            index: 0,
            generation: 0,
        },
        entity_id: ent_id.to_string(),
        component_type: "COMP".to_string(),
        position_nm: pos,
        params_si: BTreeMap::new(),
        pin_roles: BTreeMap::new(),
        pin_nets: BTreeMap::new(),
    });

    if let Some(existing) = ent.pin_nets.get(pin_id)
        && ((is_power_rail_name(existing) && is_ground_rail_name(&op.net_name))
            || (is_ground_rail_name(existing) && is_power_rail_name(&op.net_name)))
    {
        return Some(TopologicalConflictCard {
            op_id: op.op_id.clone(),
            actor_pseudo_id: actor.to_string(),
            kind: TopologicalConflictKind::PowerGroundShort {
                net_a: existing.clone(),
                net_b: op.net_name.clone(),
            },
            summary: format!(
                "CRDT rejected VCC<->GND short on {ent_id}.{pin_id} ({existing} vs {})",
                op.net_name
            ),
        });
    }

    let role = ent
        .pin_roles
        .get(pin_id)
        .copied()
        .unwrap_or(ElectricalPinRole::Passive);
    if (role == ElectricalPinRole::PowerSupplyRail && is_ground_rail_name(&op.net_name))
        || (role == ElectricalPinRole::GroundRail && is_power_rail_name(&op.net_name))
    {
        return Some(TopologicalConflictCard {
            op_id: op.op_id.clone(),
            actor_pseudo_id: actor.to_string(),
            kind: TopologicalConflictKind::PowerGroundShort {
                net_a: format!("{role:?}"),
                net_b: op.net_name.clone(),
            },
            summary: format!(
                "CRDT rejected rail short connecting {role:?} {ent_id}.{pin_id} to {}",
                op.net_name
            ),
        });
    }

    if role == ElectricalPinRole::PushPullOutput
        && let Some(members) = snap.nets.get(&op.net_name)
    {
        let self_ref = format!("{ent_id}.{pin_id}");
        let mut drivers = vec![self_ref.clone()];
        for m in members.iter().filter(|m| *m != &self_ref) {
            if let Some((me, mp)) = m.split_once('.')
                && snap.entity(me).and_then(|e| e.pin_roles.get(mp)).copied()
                    == Some(ElectricalPinRole::PushPullOutput)
            {
                drivers.push(m.clone());
            }
        }
        if drivers.len() > 1 {
            return Some(TopologicalConflictCard {
                op_id: op.op_id.clone(),
                actor_pseudo_id: actor.to_string(),
                kind: TopologicalConflictKind::MultiPushPullDriverCollision {
                    net_name: op.net_name.clone(),
                    drivers: drivers.clone(),
                },
                summary: format!(
                    "CRDT rejected multi-driver collision on `{}`: {:?}",
                    op.net_name, drivers
                ),
            });
        }
    }

    ent.pin_nets.insert(pin_id.to_string(), op.net_name.clone());
    snap.upsert_entity(ent);
    let net_set = Arc::make_mut(&mut snap.nets)
        .entry(op.net_name.clone())
        .or_insert_with(|| Arc::new(BTreeSet::new()));
    Arc::make_mut(net_set).insert(format!("{ent_id}.{pin_id}"));
    None
}
