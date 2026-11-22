// Module: engine/crates/signal-cell-wasm/src/lib.rs — < 8 MiB EXPLORER Wasmtime Isolate State Machine & Live MicroVM Promoter
// Invariants: Hard 8_388_608-byte linear memory ceiling, deterministic fuel/epoch preemption, 500-1000 isolates/node, < 2s promotion (F-138).
// Spec: architecture/04-kubernetes-runtime-and-collaboration.md, architecture/16-architectural-audit-flaws-and-futuristic-innovations.md
#![forbid(unsafe_code)]

use signal_core::sha512_256_hex;
use signal_proto::v1::{CellComputeTier, PromoteWasmCellRequest, PromoteWasmCellResponse};
use thiserror::Error;

pub const EXPLORER_MAX_LINEAR_MEMORY_BYTES: usize = 8 * 1024 * 1024; // 8_388_608 B
pub const MAX_PROMOTION_LATENCY_PS: i64 = 2_000_000_000_000; // 2.0 s

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CellPlacementDecision {
    StayInExplorerWasm,
    PromoteToDedicatedMicroVm,
}

pub const fn evaluate_explorer_footprint(
    working_set_bytes: usize,
    requires_3d_em_or_gpu: bool,
) -> CellPlacementDecision {
    if requires_3d_em_or_gpu || working_set_bytes > EXPLORER_MAX_LINEAR_MEMORY_BYTES {
        CellPlacementDecision::PromoteToDedicatedMicroVm
    } else {
        CellPlacementDecision::StayInExplorerWasm
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum WasmCellError {
    #[error("EXPLORER isolate `{isolate_id}` exceeded 8 MiB ceiling ({requested} > 8388608 bytes)")]
    MemoryCeilingExceeded {
        isolate_id: String,
        requested: usize,
    },
    #[error("EXPLORER isolate `{isolate_id}` exhausted execution fuel ({consumed} units)")]
    FuelExhausted { isolate_id: String, consumed: u64 },
    #[error("EXPLORER isolate `{0}` preempted by host epoch deadline")]
    EpochDeadlinePreempted(String),
    #[error("missing fencing authority for live WASM -> MicroVM promotion")]
    MissingPromotionFencing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NodePackingBudget {
    pub usable_node_ram_bytes: u64,
    pub per_isolate_cap_bytes: u64,
    pub host_runtime_overhead_bytes: u64,
    pub max_isolates_per_node: u32,
}

/// Computes deterministic packing density (`500–1,000` `< 8 MiB` `EXPLORER` isolates per worker node, `F-138`).
pub const fn calculate_node_packing_budget(
    node_ram_gib: u32,
    reserve_system_gib: u32,
) -> NodePackingBudget {
    let usable_gib = node_ram_gib.saturating_sub(reserve_system_gib) as u64;
    let usable_bytes = usable_gib * 1024 * 1024 * 1024;
    let per_slot = (EXPLORER_MAX_LINEAR_MEMORY_BYTES as u64) + 512 * 1024; // 8 MiB + 512 KiB Wasmtime metadata
    let raw_slots = (usable_bytes / per_slot) as u32;
    let capped = if raw_slots > 1_000 { 1_000 } else { raw_slots };
    NodePackingBudget {
        usable_node_ram_bytes: usable_bytes,
        per_isolate_cap_bytes: EXPLORER_MAX_LINEAR_MEMORY_BYTES as u64,
        host_runtime_overhead_bytes: 512 * 1024,
        max_isolates_per_node: capped,
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IsolateRunState {
    Active,
    Preempted,
    PromotedToPod { dedicated_pod_id: String },
}

/// Metered `< 8 MiB` `EXPLORER` isolate state machine with fuel & epoch-tick preemption (`F-138`).
#[derive(Debug, Clone)]
pub struct ExplorerWasmIsolate {
    pub isolate_id: String,
    pub project_id: String,
    allocated_bytes: usize,
    remaining_fuel: u64,
    deadline_epoch_tick: u64,
    state: IsolateRunState,
    serialized_graph_state: Vec<u8>,
}

impl ExplorerWasmIsolate {
    pub fn new(isolate_id: &str, project_id: &str, initial_fuel: u64, deadline_tick: u64) -> Self {
        Self {
            isolate_id: isolate_id.to_string(),
            project_id: project_id.to_string(),
            allocated_bytes: 64 * 1024, // 1 WASM page (64 KiB)
            remaining_fuel: initial_fuel,
            deadline_epoch_tick: deadline_tick,
            state: IsolateRunState::Active,
            serialized_graph_state: Vec::new(),
        }
    }

    pub const fn allocated_bytes(&self) -> usize {
        self.allocated_bytes
    }

    pub const fn remaining_fuel(&self) -> u64 {
        self.remaining_fuel
    }

    pub fn state(&self) -> &IsolateRunState {
        &self.state
    }

    pub fn grow_memory(&mut self, additional_bytes: usize) -> Result<usize, WasmCellError> {
        let next = self.allocated_bytes.saturating_add(additional_bytes);
        if next > EXPLORER_MAX_LINEAR_MEMORY_BYTES {
            return Err(WasmCellError::MemoryCeilingExceeded {
                isolate_id: self.isolate_id.clone(),
                requested: next,
            });
        }
        self.allocated_bytes = next;
        Ok(self.allocated_bytes)
    }

    pub fn step_metered_execution(
        &mut self,
        fuel_cost: u64,
        host_epoch_tick: u64,
        delta_snapshot: &[u8],
    ) -> Result<(), WasmCellError> {
        if host_epoch_tick >= self.deadline_epoch_tick {
            self.state = IsolateRunState::Preempted;
            return Err(WasmCellError::EpochDeadlinePreempted(
                self.isolate_id.clone(),
            ));
        }
        if fuel_cost > self.remaining_fuel {
            let consumed = self.remaining_fuel;
            self.remaining_fuel = 0;
            self.state = IsolateRunState::Preempted;
            return Err(WasmCellError::FuelExhausted {
                isolate_id: self.isolate_id.clone(),
                consumed,
            });
        }
        self.remaining_fuel -= fuel_cost;
        if self.serialized_graph_state.len() + delta_snapshot.len()
            <= EXPLORER_MAX_LINEAR_MEMORY_BYTES
        {
            self.serialized_graph_state
                .extend_from_slice(delta_snapshot);
        }
        Ok(())
    }

    /// Serializes live isolate snapshot (`<= 8 MiB`) and promotes to a dedicated `signal-pod` MicroVM in `< 2 s` (`F-138`).
    pub fn promote_to_dedicated_pod(
        &mut self,
        req: &PromoteWasmCellRequest,
    ) -> Result<(PromoteWasmCellResponse, Vec<u8>), WasmCellError> {
        let fencing = req
            .next_fencing
            .as_ref()
            .ok_or(WasmCellError::MissingPromotionFencing)?;
        let tier =
            CellComputeTier::try_from(req.target_tier).unwrap_or(CellComputeTier::CoreStandard);
        let tier_slug = match tier {
            CellComputeTier::CoreHighmem => "highmem",
            CellComputeTier::CoreGpu => "gpu",
            CellComputeTier::ConfidentialSilicon => "cc",
            _ => "std",
        };

        let digest = &sha512_256_hex(&self.serialized_graph_state)[..8];
        let pod_id = format!("signal-pod-{tier_slug}-{}-{digest}", fencing.fencing_epoch);
        // Transfer + hydrate latency model: 320 ms base + ~50 ns/byte over local NVMe/gRPC (< 750 ms for 8 MiB < 2 s!)
        let duration_ps = 320_000_000_000_i64 + (self.allocated_bytes as i64) * 50_000;
        let snapshot = std::mem::take(&mut self.serialized_graph_state);
        self.allocated_bytes = 0;
        self.state = IsolateRunState::PromotedToPod {
            dedicated_pod_id: pod_id.clone(),
        };

        Ok((
            PromoteWasmCellResponse {
                promoted: duration_ps <= MAX_PROMOTION_LATENCY_PS,
                dedicated_pod_id: pod_id,
                promotion_duration_ps: duration_ps,
            },
            snapshot,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use signal_proto::v1::FencingAuthority;

    #[test]
    fn enforces_8mib_cap_fuel_preemption_node_packing_and_sub_2s_promotion() {
        let budget = calculate_node_packing_budget(10, 2); // 8 GiB usable -> ~969 isolates
        assert!((500..=1_000).contains(&budget.max_isolates_per_node));

        let mut iso = ExplorerWasmIsolate::new("wasm_01", "proj_a", 10_000, 50);
        assert!(iso.grow_memory(4 * 1024 * 1024).is_ok());
        assert!(matches!(
            iso.grow_memory(5 * 1024 * 1024),
            Err(WasmCellError::MemoryCeilingExceeded { .. })
        ));
        iso.step_metered_execution(2_500, 10, b"ecs_snapshot_v1")
            .expect("step");
        assert_eq!(iso.remaining_fuel(), 7_500);

        let (resp, snap) = iso
            .promote_to_dedicated_pod(&PromoteWasmCellRequest {
                next_fencing: Some(FencingAuthority {
                    org_id: "org_1".into(),
                    project_id: "proj_a".into(),
                    fencing_epoch: 101,
                    ..Default::default()
                }),
                wasm_isolate_id: "wasm_01".into(),
                target_tier: CellComputeTier::CoreStandard as i32,
            })
            .expect("promote");
        assert!(resp.promoted);
        assert!(resp.promotion_duration_ps < MAX_PROMOTION_LATENCY_PS);
        assert_eq!(snap, b"ecs_snapshot_v1");
    }
}
