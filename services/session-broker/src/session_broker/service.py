# Module: services/session-broker/src/session_broker/service.py — Pure Session Broker Orchestration
# Invariants: Zero direct DB mutations without RLS; delegates cell lease acquisition cleanly.
# Spec: architecture/04-kubernetes-runtime-and-collaboration.md
from __future__ import annotations

from session_broker.repository import acquire_cell_lease
from session_broker.schemas import SessionLeaseResponse


async def broker_project_session(proj_slug: str) -> SessionLeaseResponse:
    return await acquire_cell_lease(proj_slug)
