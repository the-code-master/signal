// Module: engine/crates/signal-graph/src/lib.rs — Multi-Domain ECS Design Graph, Topological CRDT & Schematic Ops
// Invariants: Rejects stale fencing epochs with 412; enforces Topological CRDT & Causal Selective Undo (F-040..044, F-119).
// Spec: architecture/03-rust-eda-engine.md, architecture/10-api-and-data-contracts.md, architecture/17-extended-feature-specifications-ui-and-rust.md
#![forbid(unsafe_code)]

use signal_proto::v1::{ApplyTransactionRequest, ApplyTransactionResponse};
use std::collections::HashMap;
use thiserror::Error;

pub mod beautify;
pub mod breadboard_map;
pub mod bus_bundle;
pub mod command_query_a11y;
pub mod constraint_matrix;
pub mod cow_arena;
pub mod crdt_validator;
pub mod ecs_crdt;
pub mod ephemeral_presence;
pub mod hierarchy_ops;
pub mod pin_swap_reannotate;
pub mod snippet_stamp;
pub mod subgraph_matcher;

pub use beautify::{
    SchematicBlock, beautify_schematic_layout, count_two_layer_crossings,
    route_orthogonal_wire_astar,
};
pub use breadboard_map::{
    StripKind, TERMINAL_STRIP_HOLES, build_breadboard_strips_from_netlist,
    classify_breadboard_strip, sync_breadboard_twin,
};
pub use bus_bundle::{expand_protocol_bundle, parse_bus_slice};
pub use command_query_a11y::{
    QueryableEntity, build_wcag_accessibility_tree, evaluate_selection_query,
    execute_omni_command_dsl, route_keyboard_a11y_wire,
};
pub use constraint_matrix::{ConstraintMatrixResolver, NetClassSpec};
pub use ecs_crdt::{
    CoWGraphSnapshot, DesignGraphEngine, EcsEntity, ElectricalPinRole, GenEntityId,
    TopologicalConflictCard, TopologicalConflictKind,
};
pub use ephemeral_presence::{
    CollaboratorPresenceState, EphemeralPresenceHub, PRESENCE_MIN_INTERVAL_US,
};
pub use hierarchy_ops::{
    ChannelExpandedInstance, ResolvedBomVariantItem, SheetNetPort, build_cross_sheet_portal_index,
    expand_multi_channel_block, resolve_assembly_variant,
};
pub use pin_swap_reannotate::{
    AnnotatableInstance, SwappablePinAnchor, propose_optimal_pin_swaps, reannotate_sheet_instances,
};
pub use snippet_stamp::{
    GoldenSnippetTemplate, SnippetComponentTemplate, SnippetTraceTemplate,
    StampedComponentInstance, StampedPreRoutedTrace, StampedSnippetBundle, stamp_golden_snippet,
};
pub use subgraph_matcher::{StageComponentNode, StagePrimitiveKind, explain_circuit_stages};

#[derive(Debug, Error, PartialEq, Eq)]
pub enum GraphError {
    #[error("missing fencing authority on mutating graph transaction")]
    MissingFencing,
    #[error("stale fencing epoch {provided} < active epoch {active} (HTTP 412)")]
    StaleFencingEpoch { provided: u64, active: u64 },
    #[error("revision conflict: expected {expected}, current {current}")]
    RevisionConflict { expected: u64, current: u64 },
    #[error("unknown graph branch `{0}`")]
    UnknownBranch(String),
    #[error("invalid schematic command, query, or bus slice: {0}")]
    InvalidCommandOrSlice(String),
}

#[derive(Debug)]
pub struct GraphLedger {
    active_epoch: u64,
    revision: u64,
    idempotency_cache: HashMap<String, ApplyTransactionResponse>,
}

impl GraphLedger {
    pub fn new(initial_epoch: u64, initial_revision: u64) -> Self {
        Self {
            active_epoch: initial_epoch,
            revision: initial_revision,
            idempotency_cache: HashMap::new(),
        }
    }

    pub const fn revision(&self) -> u64 {
        self.revision
    }

    pub const fn active_epoch(&self) -> u64 {
        self.active_epoch
    }

    pub fn apply_tx(
        &mut self,
        req: &ApplyTransactionRequest,
    ) -> Result<ApplyTransactionResponse, GraphError> {
        let fencing = req.fencing.as_ref().ok_or(GraphError::MissingFencing)?;
        if fencing.fencing_epoch < self.active_epoch {
            return Err(GraphError::StaleFencingEpoch {
                provided: fencing.fencing_epoch,
                active: self.active_epoch,
            });
        }
        self.active_epoch = fencing.fencing_epoch;

        if !req.idempotency_key.is_empty()
            && let Some(cached) = self.idempotency_cache.get(&req.idempotency_key)
        {
            return Ok(cached.clone());
        }
        if req.expected_base_revision != self.revision {
            return Err(GraphError::RevisionConflict {
                expected: req.expected_base_revision,
                current: self.revision,
            });
        }

        self.revision += 1;
        let resp = ApplyTransactionResponse {
            committed_revision: self.revision,
            graph_merkle_root: "ab12cd34ef56ab12cd34ef56ab12cd34ef56ab12cd34ef56ab12cd34ef56ab12"
                .to_string(),
            has_topological_conflict: false,
            conflict_card_summary: String::new(),
        };
        if !req.idempotency_key.is_empty() {
            self.idempotency_cache
                .insert(req.idempotency_key.clone(), resp.clone());
        }
        Ok(resp)
    }
}
