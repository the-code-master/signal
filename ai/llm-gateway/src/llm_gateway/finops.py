# Module: ai/llm-gateway/src/llm_gateway/finops.py — Per-User & Per-Org Token and GPU-Second FinOps Circuit-Breaker
# Invariants: Sliding-window quota enforcement prevents runaway agent loops & GPU starvation (F-098).
# Spec: architecture/05-ai-agent-and-datasheet-rag.md, architecture/15-*.md
from __future__ import annotations

from dataclasses import dataclass

from llm_gateway.schemas import FinOpsQuotaPolicy


class FinOpsQuotaExceededError(Exception):
    """Raised when user or org sliding-window AI token/GPU quota is exhausted (F-098)."""


@dataclass(frozen=True)
class _UsageEvent:
    timestamp_ms: int
    tokens: int
    gpu_ms: int


class FinOpsCircuitBreaker:
    def __init__(self, policy: FinOpsQuotaPolicy | None = None) -> None:
        self._policy = policy or FinOpsQuotaPolicy()
        self._user_ledger: dict[str, list[_UsageEvent]] = {}
        self._org_ledger: dict[str, list[_UsageEvent]] = {}

    def _prune(self, events: list[_UsageEvent], now_ms: int) -> list[_UsageEvent]:
        cutoff = now_ms - self._policy.window_ms
        return [ev for ev in events if ev.timestamp_ms > cutoff]

    def check_and_record(
        self,
        org_id: str,
        actor_hmac: str,
        tokens: int,
        gpu_ms: int,
        now_ms: int,
    ) -> None:
        user_events = self._prune(self._user_ledger.get(actor_hmac, []), now_ms)
        org_events = self._prune(self._org_ledger.get(org_id, []), now_ms)

        user_tokens = sum(e.tokens for e in user_events) + tokens
        user_gpu = sum(e.gpu_ms for e in user_events) + gpu_ms
        org_tokens = sum(e.tokens for e in org_events) + tokens
        org_gpu = sum(e.gpu_ms for e in org_events) + gpu_ms

        if user_tokens > self._policy.user_max_tokens_per_min:
            raise FinOpsQuotaExceededError(
                f"User token budget exceeded: {user_tokens} > {self._policy.user_max_tokens_per_min}"
            )
        if user_gpu > self._policy.user_max_gpu_ms_per_min:
            raise FinOpsQuotaExceededError(
                f"User GPU budget exceeded: {user_gpu} ms > {self._policy.user_max_gpu_ms_per_min} ms"
            )
        if org_tokens > self._policy.org_max_tokens_per_min:
            raise FinOpsQuotaExceededError(
                f"Org token budget exceeded: {org_tokens} > {self._policy.org_max_tokens_per_min}"
            )
        if org_gpu > self._policy.org_max_gpu_ms_per_min:
            raise FinOpsQuotaExceededError(
                f"Org GPU budget exceeded: {org_gpu} ms > {self._policy.org_max_gpu_ms_per_min} ms"
            )

        sample = _UsageEvent(timestamp_ms=now_ms, tokens=tokens, gpu_ms=gpu_ms)
        user_events.append(sample)
        org_events.append(sample)
        self._user_ledger[actor_hmac] = user_events
        self._org_ledger[org_id] = org_events
