# Module: ai/orchestrator/src/orchestrator/schemas.py — CoW Neuro-Symbolic Orchestrator, Guardian & Proof Schemas
# Invariants: StrictSchema (extra="forbid", frozen=True); Int64Str on 64-bit SI/timestamps; zero LLM float guesses.
# Spec: architecture/05-ai-agent-and-datasheet-rag.md, architecture/16-*.md
from __future__ import annotations

from typing import Literal

from services.common.schemas import Int64Str, StrictSchema

RustKernelKind = Literal["RUST_SALTATION_ADJOINT", "RUST_MNA_SPICE", "RUST_FORMAL_IC3"]


class RagCitationPin(StrictSchema):
    mpn: str
    doc_blake3: str
    page_number: int
    bbox_norm: tuple[float, float, float, float]
    derated_summary: str


class SynthesisSpecRequest(StrictSchema):
    org_id: str
    actor_hmac: str
    active_workbench: str
    base_branch_id: str
    base_graph_merkle_root: str
    user_goal_prompt: str
    cone_token_count: int
    now_utc_ms: Int64Str


class DelegatedGraphMutation(StrictSchema):
    op_id: str
    entity_id: str
    param_key: str
    solved_by_kernel: RustKernelKind
    solved_si_value: Int64Str


class RustVerifierOutcome(StrictSchema):
    physics_determinism_epoch: int
    spice_assertions_passed: bool
    drc_erc_clean: bool
    formal_properties_unsat: bool
    solved_si_value: Int64Str


class ProofBundleRecord(StrictSchema):
    proof_bundle_id: str
    cow_branch_id: str
    base_graph_merkle_root: str
    physics_determinism_epoch: int
    spice_assertions_passed: bool
    formal_properties_unsat: bool
    drc_erc_clean: bool
    ml_dsa_65_sig_hex: str
    ed25519_sig_hex: str


class SynthesisRunResponse(StrictSchema):
    cow_branch_id: str
    resolved_llm_backend: str
    citations: tuple[RagCitationPin, ...]
    proposed_mutations: tuple[DelegatedGraphMutation, ...]
    proof_bundle: ProofBundleRecord | None
    ready_for_human_review: bool


class GuardianCheckRequest(StrictSchema):
    branch_id: str
    last_canvas_input_utc_ms: Int64Str
    now_utc_ms: Int64Str
    idle_threshold_ms: Int64Str = 1_500
    active_pointer_drag: bool = False
    is_commit_boundary: bool = False
    pending_findings: tuple[str, ...] = ()


class GuardianCheckResponse(StrictSchema):
    suppressed_mid_edit: bool
    trigger_reason: Literal["SUPPRESSED_ACTIVE_EDIT", "QUIESCENT_IDLE", "COMMIT_BOUNDARY"]
    emitted_alerts: tuple[str, ...]


class MergeProofCheckRequest(StrictSchema):
    proof_bundle: ProofBundleRecord
    current_main_merkle_root: str


class MergeProofCheckResponse(StrictSchema):
    merge_allowed: bool
    main_advanced_since_branch: bool
    reverified_after_rebase: bool
    delta_reverify_duration_ps: Int64Str
    verified_merkle_root: str
    rejection_reason: str | None = None
