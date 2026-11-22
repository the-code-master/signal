// Module: engine/crates/workspace-hack/src/lib.rs — Unified Workspace Feature Set Anchor
// Invariants: Zero runtime code; #![forbid(unsafe_code)] enforced via workspace lints (F-143).
// Spec: docs/engineering/RUST_ENGINEERING_GUIDE.md
#![forbid(unsafe_code)]

use prost as _;
use serde as _;
