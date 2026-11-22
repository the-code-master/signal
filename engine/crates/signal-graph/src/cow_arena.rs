// Module: engine/crates/signal-graph/src/cow_arena.rs — Generational-Arena Arc CoW Snapshot
// Invariants: Structural O(1) Arc CoW clones (F-044); canonical SHA-512/256 Merkle root over sorted entities.
// Spec: architecture/03-rust-eda-engine.md, architecture/16-architectural-audit-flaws-and-futuristic-innovations.md
use signal_core::sha512_256_hex;
use signal_geom::Point2Nm;
use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GenEntityId {
    pub index: u32,
    pub generation: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ElectricalPinRole {
    Passive,
    Input,
    PushPullOutput,
    OpenDrain,
    PowerSupplyRail,
    GroundRail,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EcsEntity {
    pub slot: GenEntityId,
    pub entity_id: String,
    pub component_type: String,
    pub position_nm: Point2Nm,
    pub params_si: BTreeMap<String, i64>,
    pub pin_roles: BTreeMap<String, ElectricalPinRole>,
    pub pin_nets: BTreeMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TopologicalConflictKind {
    PowerGroundShort {
        net_a: String,
        net_b: String,
    },
    MultiPushPullDriverCollision {
        net_name: String,
        drivers: Vec<String>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TopologicalConflictCard {
    pub op_id: String,
    pub actor_pseudo_id: String,
    pub kind: TopologicalConflictKind,
    pub summary: String,
}

#[derive(Debug, Clone)]
struct ArenaSlot {
    generation: u32,
    occupied: Option<Arc<EcsEntity>>,
}

/// Structural Copy-on-Write (`Arc`-backed) generational-arena ECS snapshot (`F-044`).
#[derive(Debug, Clone)]
pub struct CoWGraphSnapshot {
    slots: Arc<Vec<ArenaSlot>>,
    free_indices: Arc<Vec<u32>>,
    id_lookup: Arc<BTreeMap<String, GenEntityId>>,
    pub(crate) nets: Arc<BTreeMap<String, Arc<BTreeSet<String>>>>,
    pub(crate) revision: u64,
}

impl Default for CoWGraphSnapshot {
    fn default() -> Self {
        Self::new(0)
    }
}

impl CoWGraphSnapshot {
    pub fn new(revision: u64) -> Self {
        Self {
            slots: Arc::new(Vec::new()),
            free_indices: Arc::new(Vec::new()),
            id_lookup: Arc::new(BTreeMap::new()),
            nets: Arc::new(BTreeMap::new()),
            revision,
        }
    }

    pub const fn revision(&self) -> u64 {
        self.revision
    }

    pub fn entity(&self, entity_id: &str) -> Option<&EcsEntity> {
        let handle = self.id_lookup.get(entity_id)?;
        let slot = self.slots.get(handle.index as usize)?;
        if slot.generation == handle.generation {
            slot.occupied.as_deref()
        } else {
            None
        }
    }

    pub fn entities_sorted(&self) -> Vec<&EcsEntity> {
        self.id_lookup
            .values()
            .filter_map(|h| self.slots.get(h.index as usize)?.occupied.as_deref())
            .collect()
    }

    pub fn merkle_root(&self) -> String {
        let mut canonical = String::new();
        use std::fmt::Write as _;
        let _ = write!(&mut canonical, "rev={};", self.revision);
        for ent in self.entities_sorted() {
            let _ = write!(
                &mut canonical,
                "E:{}:{}@{},{}:P{:?}:N{:?};",
                ent.entity_id,
                ent.component_type,
                ent.position_nm.x.0,
                ent.position_nm.y.0,
                ent.params_si,
                ent.pin_nets
            );
        }
        sha512_256_hex(canonical.as_bytes())
    }

    pub fn upsert_entity(&mut self, mut entity: EcsEntity) -> GenEntityId {
        if let Some(&existing) = self.id_lookup.get(&entity.entity_id) {
            entity.slot = existing;
            Arc::make_mut(&mut self.slots)[existing.index as usize].occupied =
                Some(Arc::new(entity));
            return existing;
        }
        if let Some(free_idx) = Arc::make_mut(&mut self.free_indices).pop() {
            let slots = Arc::make_mut(&mut self.slots);
            let generation = slots[free_idx as usize].generation;
            let handle = GenEntityId {
                index: free_idx,
                generation,
            };
            entity.slot = handle;
            let eid = entity.entity_id.clone();
            slots[free_idx as usize].occupied = Some(Arc::new(entity));
            Arc::make_mut(&mut self.id_lookup).insert(eid, handle);
            handle
        } else {
            let slots = Arc::make_mut(&mut self.slots);
            let handle = GenEntityId {
                index: slots.len() as u32,
                generation: 1,
            };
            entity.slot = handle;
            let eid = entity.entity_id.clone();
            slots.push(ArenaSlot {
                generation: 1,
                occupied: Some(Arc::new(entity)),
            });
            Arc::make_mut(&mut self.id_lookup).insert(eid, handle);
            handle
        }
    }

    pub fn remove_entity(&mut self, entity_id: &str) -> bool {
        let Some(handle) = Arc::make_mut(&mut self.id_lookup).remove(entity_id) else {
            return false;
        };
        let slots = Arc::make_mut(&mut self.slots);
        if let Some(slot) = slots.get_mut(handle.index as usize)
            && slot.generation == handle.generation
        {
            slot.occupied = None;
            slot.generation = slot.generation.wrapping_add(1);
            Arc::make_mut(&mut self.free_indices).push(handle.index);
        }
        let prefix = format!("{entity_id}.");
        for members in Arc::make_mut(&mut self.nets).values_mut() {
            Arc::make_mut(members).retain(|pin_ref| !pin_ref.starts_with(&prefix));
        }
        true
    }
}
