# Module: services/sftp-egress/src/sftp_egress/service.py — Pure Async Domain Orchestration
# Invariants: Zero raw SQL, zero physics/DRC kernels.
# Spec: docs/engineering/FASTAPI_ENGINEERING_GUIDE.md
from __future__ import annotations

from sftp_egress.repository import fetch_sftp_egress_record
from sftp_egress.schemas import SftpReleaseQueued


async def execute_sftp_egress(org_slug: str) -> SftpReleaseQueued:
    return await fetch_sftp_egress_record(org_slug)
