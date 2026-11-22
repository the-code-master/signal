# Module: services/common/db/rls.py — Fail-Closed PostgreSQL 19+ Row-Level Security Context Manager
# Invariants: Enforces NOBYPASSRLS role + transaction-scoped set_config('app.org_id', ..., true) (F-084).
# Spec: architecture/15-application-security-and-project-isolation.md, docs/engineering/FASTAPI_ENGINEERING_GUIDE.md
from __future__ import annotations

from collections.abc import AsyncGenerator, Callable
from contextlib import asynccontextmanager
from typing import Protocol

from services.common.schemas import PrincipalContext


class AsyncSqlSessionProtocol(Protocol):
    async def execute(self, sql: str, params: tuple[str, ...] = ()) -> None: ...
    async def commit(self) -> None: ...
    async def rollback(self) -> None: ...


@asynccontextmanager
async def with_rls_tx[S: AsyncSqlSessionProtocol](
    session_factory: Callable[[], S],
    principal: PrincipalContext,
) -> AsyncGenerator[S]:
    if not principal.org_id.strip():
        raise ValueError("PrincipalContext.org_id must be non-empty before opening RLS transaction")
    session = session_factory()
    try:
        await session.execute("SET LOCAL ROLE signal_app_rls")
        await session.execute("SELECT set_config('app.org_id', $1, true)", (principal.org_id,))
        await session.execute("SELECT set_config('app.actor_hmac', $1, true)", (principal.actor_hmac,))
        yield session
        await session.commit()
    except Exception:
        await session.rollback()
        raise
