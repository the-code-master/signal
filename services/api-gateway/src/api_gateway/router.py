# Module: services/api-gateway/src/api_gateway/router.py — Healthz & Platform Releases Gateway Router
# Invariants: /api/v1/platform/releases guarded by require_authenticated; /api/v1/healthz public liveness.
# Spec: architecture/06-frontend-angular-and-fastapi-control-plane.md, architecture/20-*.md
from __future__ import annotations

from fastapi import APIRouter, Depends

from api_gateway.schemas import HealthzResponse, PlatformReleaseDto
from api_gateway.service import get_health_status, list_releases
from services.common.schemas import PrincipalContext
from services.common.security import require_authenticated

router = APIRouter()


@router.get("/api/v1/healthz", response_model=HealthzResponse)
async def healthz() -> HealthzResponse:
    return await get_health_status()


@router.get("/api/v1/platform/releases", response_model=list[PlatformReleaseDto])
async def get_platform_releases(
    _principal: PrincipalContext = Depends(require_authenticated),
) -> list[PlatformReleaseDto]:
    return await list_releases()
