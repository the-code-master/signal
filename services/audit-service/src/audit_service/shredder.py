# Module: services/audit-service/src/audit_service/shredder.py — GDPR Right-to-Erasure Cryptographic Salt Shredder
# Invariants: Destroys per-user user_pii_salt (F-100) without mutating a single RFC-3161 WORM Merkle block.
# Spec: architecture/07-security-encryption-dlp-and-sftp.md, architecture/16-*.md
from __future__ import annotations

import hashlib
import hmac
import os

from audit_service.outbox import FailClosedQuorumOutbox
from audit_service.schemas import GdprSaltShredReceipt, Tier0AuditEvent


async def _default_replica_ack(_entry_hash: str, _event: Tier0AuditEvent) -> bool:
    return True


class GdprPiiSaltVault:
    """Stores per-subject cryptographic HMAC salts separately from immutable WORM logs (F-100)."""

    def __init__(self) -> None:
        self._salts: dict[tuple[str, str], bytes] = {}
        self._shredded_subjects: set[tuple[str, str]] = set()

    def ensure_subject_salt(self, org_id: str, provider_user_id: str) -> bytes:
        key = (org_id, provider_user_id)
        if key in self._shredded_subjects:
            raise ValueError("SUBJECT_SALT_SHREDDED")
        if key not in self._salts:
            self._salts[key] = os.urandom(32)
        return self._salts[key]

    def derive_pseudonym(self, org_id: str, provider_user_id: str) -> str:
        salt = self.ensure_subject_salt(org_id, provider_user_id)
        digest = hmac.new(salt, provider_user_id.encode("utf-8"), hashlib.sha256).hexdigest()
        return f"actor_hmac_{digest[:16]}"

    def is_shredded(self, org_id: str, provider_user_id: str) -> bool:
        return (org_id, provider_user_id) in self._shredded_subjects

    def shred_subject_salt(
        self,
        org_id: str,
        provider_user_id: str,
        erasure_ticket_ref: str,
        worm_outbox: FailClosedQuorumOutbox,
    ) -> GdprSaltShredReceipt:
        key = (org_id, provider_user_id)
        tip_before = worm_outbox.tip_hash
        seq_before = worm_outbox.seq

        self._salts.pop(key, None)
        self._shredded_subjects.add(key)

        tip_after = worm_outbox.tip_hash
        seq_after = worm_outbox.seq
        return GdprSaltShredReceipt(
            subject_erased=True,
            erasure_ticket_ref=erasure_ticket_ref,
            worm_outbox_seq=seq_after,
            worm_tip_hash_before=tip_before,
            worm_tip_hash_after=tip_after,
            worm_chain_unmutated=(tip_before == tip_after and seq_before == seq_after),
            actor_hmac_unlinkable=(key not in self._salts and key in self._shredded_subjects),
        )


GLOBAL_WORM_OUTBOX = FailClosedQuorumOutbox((_default_replica_ack, _default_replica_ack, _default_replica_ack))
GDPR_SALT_VAULT = GdprPiiSaltVault()
