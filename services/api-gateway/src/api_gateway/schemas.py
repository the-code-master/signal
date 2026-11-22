# Module: services/api-gateway/src/api_gateway/schemas.py — Platform Release & Health DTOs
# Invariants: StrictSchema; released_at_ms serialized as Int64Str string (20-*, F-144).
# Spec: architecture/20-versioning-release-monorepo-build-and-k8s-topology.md
from __future__ import annotations

from services.common.schemas import Int64Str, StrictSchema


class HealthzResponse(StrictSchema):
    status: str
    version: str
    runtime: str


class PlatformReleaseDto(StrictSchema):
    version: str
    channel: str
    released_at_ms: Int64Str
    physics_determinism_epoch: int
    wire_min_version: str
    highlights: tuple[str, ...]
