# Module: services/common/idempotency.py — Mandatory Idempotency-Key UUID Header Validator
# Invariants: Enforces valid UUID Idempotency-Key on all state-mutating POST/PUT/PATCH endpoints (F-102).
# Spec: architecture/10-api-and-data-contracts.md, docs/engineering/FASTAPI_ENGINEERING_GUIDE.md
from __future__ import annotations

import uuid

from fastapi import Header, HTTPException


async def require_idempotency_key(
    idempotency_key: str = Header(..., alias="Idempotency-Key"),
) -> str:
    try:
        parsed = uuid.UUID(idempotency_key)
    except ValueError as exc:
        raise HTTPException(status_code=400, detail="Invalid Idempotency-Key UUID header") from exc
    return str(parsed)
