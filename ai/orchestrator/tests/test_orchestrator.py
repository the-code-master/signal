# Module: ai/orchestrator/tests/test_orchestrator.py — CoW Synthesis, Quiescent Guardian & Merge Proof Delta Tests
# Invariants: Verifies cow/* isolation, Rust math delegation, mid-drag guardian suppression, and F-133 rebase re-verify.
# Spec: architecture/05-ai-agent-and-datasheet-rag.md
from __future__ import annotations

import pytest
from orchestrator import (
    ConeBudgetExceededError,
    GuardianCheckRequest,
    MergeProofCheckRequest,
    NeuroSymbolicOrchestrator,
    QuiescentDesignGuardian,
    SynthesisSpecRequest,
    verify_proof_bundle_at_merge,
)


def test_neuro_symbolic_cow_synthesis_and_cone_budget_gate() -> None:
    orch = NeuroSymbolicOrchestrator()
    res = orch.run_speculative_synthesis(
        SynthesisSpecRequest(
            org_id="org-1",
            actor_hmac="actor_hmac_01",
            active_workbench="sim-viewer",
            base_branch_id="main",
            base_graph_merkle_root="merkle_main_r100",
            user_goal_prompt="optimize snubber capacitor for SiC half-bridge",
            cone_token_count=4_200,
            now_utc_ms=1_000,
        )
    )
    assert res.cow_branch_id.startswith("cow/ai-")
    assert res.ready_for_human_review is True
    assert res.proof_bundle is not None
    assert res.proposed_mutations[0].solved_by_kernel == "RUST_SALTATION_ADJOINT"

    with pytest.raises(ConeBudgetExceededError):
        orch.run_speculative_synthesis(
            SynthesisSpecRequest(
                org_id="org-1",
                actor_hmac="actor_hmac_01",
                active_workbench="sim-viewer",
                base_branch_id="main",
                base_graph_merkle_root="merkle_main_r100",
                user_goal_prompt="whole SoC dump",
                cone_token_count=20_000,
                now_utc_ms=2_000,
            )
        )


def test_quiescent_guardian_suppresses_mid_drag_and_fires_on_idle_or_commit() -> None:
    guardian = QuiescentDesignGuardian()
    alerts = ("C_IN_1 voltage derating < 2x V_bus",)

    mid_drag = guardian.evaluate(
        GuardianCheckRequest(
            branch_id="main",
            last_canvas_input_utc_ms=10_000,
            now_utc_ms=12_000,
            active_pointer_drag=True,
            pending_findings=alerts,
        )
    )
    assert mid_drag.suppressed_mid_edit is True
    assert mid_drag.emitted_alerts == ()

    burst_typing = guardian.evaluate(
        GuardianCheckRequest(
            branch_id="main",
            last_canvas_input_utc_ms=10_000,
            now_utc_ms=10_400,
            idle_threshold_ms=1_500,
            pending_findings=alerts,
        )
    )
    assert burst_typing.suppressed_mid_edit is True

    quiescent = guardian.evaluate(
        GuardianCheckRequest(
            branch_id="main",
            last_canvas_input_utc_ms=10_000,
            now_utc_ms=11_800,
            idle_threshold_ms=1_500,
            pending_findings=alerts,
        )
    )
    assert quiescent.suppressed_mid_edit is False
    assert quiescent.trigger_reason == "QUIESCENT_IDLE"
    assert quiescent.emitted_alerts == alerts

    commit_gate = guardian.evaluate(
        GuardianCheckRequest(
            branch_id="main",
            last_canvas_input_utc_ms=10_000,
            now_utc_ms=10_100,
            is_commit_boundary=True,
            pending_findings=alerts,
        )
    )
    assert commit_gate.trigger_reason == "COMMIT_BOUNDARY"
    assert commit_gate.emitted_alerts == alerts


def test_merge_time_proof_delta_reverification_gate() -> None:
    orch = NeuroSymbolicOrchestrator()
    synth = orch.run_speculative_synthesis(
        SynthesisSpecRequest(
            org_id="org-1",
            actor_hmac="actor_hmac_01",
            active_workbench="sim-viewer",
            base_branch_id="main",
            base_graph_merkle_root="merkle_main_r100",
            user_goal_prompt="optimize snubber",
            cone_token_count=2_000,
            now_utc_ms=5_000,
        )
    )
    assert synth.proof_bundle is not None

    # 1. Fast-path when main has not moved:
    same_root = verify_proof_bundle_at_merge(
        MergeProofCheckRequest(
            proof_bundle=synth.proof_bundle,
            current_main_merkle_root="merkle_main_r100",
        )
    )
    assert same_root.merge_allowed is True
    assert same_root.reverified_after_rebase is False

    # 2. Main advanced cleanly -> triggers delta re-verification and allows merge:
    rebased_ok = verify_proof_bundle_at_merge(
        MergeProofCheckRequest(
            proof_bundle=synth.proof_bundle,
            current_main_merkle_root="merkle_main_r101_clean",
        )
    )
    assert rebased_ok.merge_allowed is True
    assert rebased_ok.main_advanced_since_branch is True
    assert rebased_ok.reverified_after_rebase is True
    assert rebased_ok.delta_reverify_duration_ps > 0

    # 3. Main advanced with conflicting change -> delta re-verify blocks merge!
    rebased_conflict = verify_proof_bundle_at_merge(
        MergeProofCheckRequest(
            proof_bundle=synth.proof_bundle,
            current_main_merkle_root="merkle_main_r102_conflict",
        )
    )
    assert rebased_conflict.merge_allowed is False
    assert rebased_conflict.rejection_reason is not None
    assert "F-133" in rebased_conflict.rejection_reason
