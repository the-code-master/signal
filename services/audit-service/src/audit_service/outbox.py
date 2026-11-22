# Module: services/audit-service/src/audit_service/outbox.py — Fail-Closed 3x Quorum Tier-0 Audit Outbox
# Invariants: HTTP 503 if <3 replicas ack; chains SHA-256 Merkle hashes for RFC-3161 WORM (F-087, F-147).
# Spec: architecture/07-security-encryption-dlp-and-sftp.md, architecture/16-*.md
from __future__ import annotations

import hashlib
from collections.abc import Awaitable, Callable

from fastapi import HTTPException

from audit_service.schemas import QuorumCommitReceipt, Tier0AuditEvent

REQUIRED_QUORUM_ACKS = 3
ReplicaWriter = Callable[[str, Tier0AuditEvent], Awaitable[bool]]


class FailClosedQuorumOutbox:
    def __init__(self, replicas: tuple[ReplicaWriter, ...]) -> None:
        self._replicas = replicas
        self._seq = 0
        self._tip_hash = "0" * 64

    @property
    def seq(self) -> int:
        return self._seq

    @property
    def tip_hash(self) -> str:
        return self._tip_hash

    async def commit_tier0_event(self, event: Tier0AuditEvent) -> QuorumCommitReceipt:
        payload = f"{self._tip_hash}|{event.org_id}|{event.actor_hmac}|{event.action_kind}|{event.occurred_at_ms}"
        entry_hash = hashlib.sha256(payload.encode()).hexdigest()

        acks = 0
        for writer in self._replicas:
            if await writer(entry_hash, event):
                acks += 1

        if acks < REQUIRED_QUORUM_ACKS:
            raise HTTPException(
                status_code=503,
                detail=f"Tier-0 Audit Outbox failed closed (F-147): {acks}/{REQUIRED_QUORUM_ACKS} replicas acked",
            )

        self._seq += 1
        self._tip_hash = entry_hash
        return QuorumCommitReceipt(
            outbox_seq=self._seq,
            quorum_acks=acks,
            entry_merkle_hash=entry_hash,
        )
