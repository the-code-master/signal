// Module: engine/crates/signal-diff-sim/src/lib.rs — Revolve Binomial Checkpointing & Saltation Planner
// Invariants: Bounds reverse-mode SPICE adjoint checkpoint slots to O(log2(steps)) (F-113).
// Spec: architecture/03-rust-eda-engine.md
#![forbid(unsafe_code)]

use signal_sim as _;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RevolveSchedule {
    pub total_steps: u64,
    pub checkpoint_slots: u32,
    pub saltation_enabled: bool,
}

pub const fn plan_revolve_checkpoints(total_steps: u64) -> RevolveSchedule {
    let slots = if total_steps <= 1 {
        1
    } else {
        (64 - (total_steps - 1).leading_zeros()) + 2
    };
    RevolveSchedule {
        total_steps,
        checkpoint_slots: slots,
        saltation_enabled: true,
    }
}
