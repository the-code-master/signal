# Module: services/common/errors.py — Constant-Shape Uniform 404 BOLA/IDOR Anti-Enumeration Handler
# Invariants: Never leaks 403 vs 404 across unauthorized orgs, projects, or scoped sub-repositories (F-084).
# Spec: architecture/15-application-security-and-project-isolation.md
from __future__ import annotations

import uuid
from typing import NoReturn

from fastapi import FastAPI, Request
from fastapi.responses import JSONResponse

from services.common.schemas import UniformError


class UniformNotFoundError(Exception):
    """Raised on any missing OR unauthorized tenant/project/repo lookup (F-084)."""


def raise_not_found() -> NoReturn:
    raise UniformNotFoundError()


def register_uniform_error_handlers(app: FastAPI) -> None:
    @app.exception_handler(UniformNotFoundError)
    async def _handle_uniform_404(request: Request, _exc: UniformNotFoundError) -> JSONResponse:
        req_id = request.headers.get("x-request-id") or str(uuid.uuid4())
        body = UniformError(request_id=req_id)
        return JSONResponse(status_code=404, content=body.model_dump(mode="json"))
