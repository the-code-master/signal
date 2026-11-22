// Module: engine/crates/signal-graph/src/lib.rs — Multi-Domain Design Graph Fencing & Idempotent Transaction Ledger
// Invariants: Rejects stale fencing epochs with 412; deduplicates Idempotency-Key retries deterministically (F-102, F-119).
// Spec: architecture/03-rust-eda-engine.md, architecture/10-api-and-data-contracts.md
#![forbid(unsafe_code)]

use signal_geom as _;
use signal_proto::v1::{ApplyTransactionRequest, ApplyTransactionResponse};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum GraphError {
    #[error("missing fencing authority on mutating graph transaction")]
    MissingFencing,
    #[error("stale fencing epoch {provided} < active epoch {active} (HTTP 412)")]
    StaleFencingEpoch { provided: u64, active: u64 },
    #[error("revision conflict: expected {expected}, current {current}")]
    RevisionConflict { expected: u64, current: u64 },
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
