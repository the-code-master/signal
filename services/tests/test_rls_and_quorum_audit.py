# Module: services/tests/test_rls_and_quorum_audit.py — PostgreSQL 19+ RLS & 3x Quorum Audit Outbox Tests
# Invariants: Verifies NOBYPASSRLS session scoping, GDPR PII rejection (F-100), and 503 fail-closed quorum loss (F-147).
# Spec: architecture/07-security-encryption-dlp-and-sftp.md, architecture/15-application-security-and-project-isolation.md
from __future__ import annotations

import asyncio

import pytest
from audit_service.outbox import FailClosedQuorumOutbox
from audit_service.schemas import Tier0AuditEvent
from fastapi import HTTPException
from pydantic import SecretStr, ValidationError

from services.common.db.rls import with_rls_tx
from services.common.schemas import PrincipalContext


class _FakeAsyncPgSession:
    def __init__(self) -> None:
        self.calls: list[tuple[str, tuple[str, ...]]] = []
        self.committed = False
        self.rolled_back = False

    async def execute(self, sql: str, params: tuple[str, ...] = ()) -> None:
        self.calls.append((sql, params))

    async def commit(self) -> None:
        self.committed = True

    async def rollback(self) -> None:
        self.rolled_back = True


def test_with_rls_tx_enforces_nobypassrls_role_and_tenant_gucs() -> None:
    principal = PrincipalContext(
        org_id="11111111-2222-4333-8444-555555555555",
        org_slug="signal-design-inc",
        provider_user_id="local-guest-0001",
        actor_hmac="actor_hmac_0123456789abcdef",
        org_role="ORG_ADMIN",
        authorized_provider_repo_ids=("local_gh_repo_1001",),
        session_token=SecretStr("dev-tok"),
    )
    fake = _FakeAsyncPgSession()

    async def _run() -> None:
        async with with_rls_tx(lambda: fake, principal) as tx:
            await tx.execute("SELECT 1")

    asyncio.run(_run())
    assert fake.committed is True
    assert fake.calls[0][0] == "SET LOCAL ROLE signal_app_rls"
    assert fake.calls[1] == ("SELECT set_config('app.org_id', $1, true)", (principal.org_id,))


def test_tier0_quorum_audit_blocks_raw_pii_and_fails_closed_below_3_replicas() -> None:
    with pytest.raises(ValidationError):
        Tier0AuditEvent(
            org_id="org-1",
            actor_hmac="alice@example.com",
            action_kind="SFTP_RELEASE",
            target_resource_id="rel-1",
            occurred_at_ms=1795324800000,
        )

    valid_evt = Tier0AuditEvent(
        org_id="org-1",
        actor_hmac="actor_hmac_0123456789abcdef",
        action_kind="SFTP_RELEASE",
        target_resource_id="rel-1",
        occurred_at_ms=1795324800000,
    )

    async def _healthy(_h: str, _e: Tier0AuditEvent) -> bool:
        return True

    async def _down(_h: str, _e: Tier0AuditEvent) -> bool:
        return False

    async def _run() -> None:
        ok_outbox = FailClosedQuorumOutbox((_healthy, _healthy, _healthy))
        receipt = await ok_outbox.commit_tier0_event(valid_evt)
        assert receipt.quorum_acks == 3
        assert receipt.outbox_seq == 1

        degraded_outbox = FailClosedQuorumOutbox((_healthy, _healthy, _down))
        with pytest.raises(HTTPException) as exc_info:
            await degraded_outbox.commit_tier0_event(valid_evt)
        assert exc_info.value.status_code == 503

    asyncio.run(_run())
