# Module: services/api-gateway/src/api_gateway/service.py — Platform Release & Health Orchestration
# Invariants: Pure async orchestration; zero physics or file-export logic.
# Spec: architecture/20-versioning-release-monorepo-build-and-k8s-topology.md
from __future__ import annotations

from api_gateway.repository import REPO_ROOT, load_platform_releases
from api_gateway.schemas import HealthzResponse, PlatformReleaseDto


async def get_health_status() -> HealthzResponse:
    ver = (REPO_ROOT / "VERSION").read_text(encoding="utf-8").strip()
    return HealthzResponse(status="ok", version=ver, runtime="python-3.15-fastapi-granian")


async def list_releases() -> list[PlatformReleaseDto]:
    return await load_platform_releases()
