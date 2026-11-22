# Module: services/common/schemas.py — Immutable Pydantic v2 Base Models, Int64Str & PrincipalContext
# Invariants: extra="forbid", frozen=True; Int64Str string parity (>2^53-1, F-144); SecretStr redaction (F-088).
# Spec: architecture/10-api-and-data-contracts.md, docs/engineering/FASTAPI_ENGINEERING_GUIDE.md
from __future__ import annotations

from typing import Annotated

from pydantic import BaseModel, BeforeValidator, ConfigDict, PlainSerializer, SecretStr


def _parse_int64(value: object) -> int:
    if isinstance(value, bool):
        raise ValueError("boolean is not a valid 64-bit integer")
    if isinstance(value, int):
        parsed = value
    elif isinstance(value, str):
        parsed = int(value, 10)
    else:
        raise ValueError("expected base-10 string or integer for Int64Str")
    if not (-(1 << 63) <= parsed < (1 << 64)):
        raise ValueError("integer out of 64-bit bounds")
    return parsed


Int64Str = Annotated[
    int,
    BeforeValidator(_parse_int64),
    PlainSerializer(lambda v: str(v), return_type=str, when_used="json"),
]


class StrictSchema(BaseModel):
    model_config = ConfigDict(extra="forbid", frozen=True)


class UniformError(StrictSchema):
    code: str = "NOT_FOUND"
    message: str = "Requested resource was not found."
    request_id: str


class PrincipalContext(StrictSchema):
    org_id: str
    org_slug: str
    provider_user_id: str
    actor_hmac: str
    org_role: str
    authorized_provider_repo_ids: tuple[str, ...]
    session_token: SecretStr


class FencingAuthoritySchema(StrictSchema):
    org_id: str
    project_id: str
    fencing_epoch: Int64Str
    lease_holder_pod_id: str
    issued_at_utc_ms: Int64Str
    expires_at_utc_ms: Int64Str
