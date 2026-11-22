# Module: ai/orchestrator/src/orchestrator/guardian.py — Quiescent-Idle & Commit-Boundary Design Guardian (F-065)
# Invariants: Never interrupts mid-drag or rapid typing (< idle_threshold_ms); fires at quiescent pause or commit.
# Spec: architecture/05-ai-agent-and-datasheet-rag.md, architecture/16-*.md
from __future__ import annotations

from orchestrator.schemas import GuardianCheckRequest, GuardianCheckResponse


class QuiescentDesignGuardian:
    def evaluate(self, req: GuardianCheckRequest) -> GuardianCheckResponse:
        if req.is_commit_boundary and not req.active_pointer_drag:
            return GuardianCheckResponse(
                suppressed_mid_edit=False,
                trigger_reason="COMMIT_BOUNDARY",
                emitted_alerts=req.pending_findings,
            )

        elapsed_ms = req.now_utc_ms - req.last_canvas_input_utc_ms
        if req.active_pointer_drag or elapsed_ms < req.idle_threshold_ms:
            return GuardianCheckResponse(
                suppressed_mid_edit=True,
                trigger_reason="SUPPRESSED_ACTIVE_EDIT",
                emitted_alerts=(),
            )

        return GuardianCheckResponse(
            suppressed_mid_edit=False,
            trigger_reason="QUIESCENT_IDLE",
            emitted_alerts=req.pending_findings,
        )
