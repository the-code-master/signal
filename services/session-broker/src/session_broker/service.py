# Module: services/session-broker/src/session_broker/service.py — Sticky Cell Ticket Issuer & In-Jurisdiction DR Gate
# Invariants: Issues short-lived signed WebTransport cell tickets; blocks cross-border & Sovereign DR replication (F-142).
# Spec: architecture/04-kubernetes-runtime-and-collaboration.md, architecture/16-*.md
from __future__ import annotations

import hashlib
import hmac
import os
import time

from fastapi import HTTPException

from session_broker.repository import acquire_cell_lease, save_dr_policy
from session_broker.schemas import (
    PairedRegionDrPolicyRequest,
    PairedRegionDrPolicyResponse,
    SessionLeaseResponse,
)

TICKET_TTL_MS = 60_000

REGION_JURISDICTIONS: dict[str, str] = {
    "us-east-1": "US_COMMERCIAL",
    "us-west-2": "US_COMMERCIAL",
    "eu-central-1": "EU_GDPR",
    "eu-west-1": "EU_GDPR",
    "us-gov-west-1": "US_ITAR_GOVCLOUD",
    "us-gov-east-1": "US_ITAR_GOVCLOUD",
    "ap-south-1": "IN_DPDP",
    "ap-south-2": "IN_DPDP",
}

ALLOWED_DR_PAIRS: frozenset[tuple[str, str]] = frozenset(
    {
        ("us-east-1", "us-west-2"),
        ("us-west-2", "us-east-1"),
        ("eu-central-1", "eu-west-1"),
        ("eu-west-1", "eu-central-1"),
        ("us-gov-west-1", "us-gov-east-1"),
        ("us-gov-east-1", "us-gov-west-1"),
        ("ap-south-1", "ap-south-2"),
        ("ap-south-2", "ap-south-1"),
    }
)


def mint_sticky_webtransport_ticket(
    cell_id: str,
    fencing_epoch: int,
    actor_hmac: str,
    expires_at_ms: int,
) -> str:
    secret = os.environ.get("SIGNAL_BRIDGE_TICKET_SECRET", "signal-dev-bridge-ticket-key").encode("utf-8")
    claims = f"{cell_id}:{fencing_epoch}:{actor_hmac}:{expires_at_ms}"
    sig = hmac.new(secret, claims.encode("utf-8"), hashlib.sha256).hexdigest()[:32]
    return f"wtt_v1.{cell_id}.{expires_at_ms}.{sig}"


async def broker_project_session(
    proj_slug: str,
    actor_hmac: str = "actor_hmac_guest",
) -> SessionLeaseResponse:
    base = await acquire_cell_lease(proj_slug)
    expires_ms = int(time.time() * 1000) + TICKET_TTL_MS
    ticket = mint_sticky_webtransport_ticket(
        base.cell_id,
        base.active_fencing_epoch,
        actor_hmac,
        expires_ms,
    )
    return SessionLeaseResponse(
        cell_id=base.cell_id,
        runtime_tier=base.runtime_tier,
        active_fencing_epoch=base.active_fencing_epoch,
        engine_version=base.engine_version,
        negotiated_codec=base.negotiated_codec,
        sticky_webtransport_ticket=ticket,
        ticket_expires_at_utc_ms=expires_ms,
    )


async def configure_paired_region_dr_policy(
    org_slug: str,
    proj_slug: str,
    req: PairedRegionDrPolicyRequest,
) -> PairedRegionDrPolicyResponse:
    primary = req.primary_region.strip().lower()
    primary_jur = REGION_JURISDICTIONS.get(primary)
    if primary_jur is None:
        raise HTTPException(status_code=400, detail=f"Unsupported primary region `{primary}`")

    if req.residency_policy == "SOVEREIGN_SINGLE_REGION_STRICT":
        if req.dr_standby_region is not None and req.dr_standby_region.strip().lower() != primary:
            raise HTTPException(
                status_code=409,
                detail="SOVEREIGN_SINGLE_REGION_STRICT forbids cross-region DR standby replication (F-142)",
            )
        policy = PairedRegionDrPolicyResponse(
            org_slug=org_slug,
            project_slug=proj_slug,
            residency_policy=req.residency_policy,
            primary_region=primary,
            dr_standby_region=None,
            jurisdiction_code=primary_jur,
            verified_in_jurisdiction=True,
        )
        return await save_dr_policy(policy)

    if req.dr_standby_region is None:
        raise HTTPException(status_code=400, detail="dr_standby_region required for IN_JURISDICTION_PAIRED_DR")
    standby = req.dr_standby_region.strip().lower()
    if (primary, standby) not in ALLOWED_DR_PAIRS:
        raise HTTPException(
            status_code=409,
            detail=(
                f"Cross-jurisdiction or unapproved DR pair `{primary} <-> {standby}` "
                "strictly rejected (F-142)"
            ),
        )
    policy = PairedRegionDrPolicyResponse(
        org_slug=org_slug,
        project_slug=proj_slug,
        residency_policy=req.residency_policy,
        primary_region=primary,
        dr_standby_region=standby,
        jurisdiction_code=primary_jur,
        verified_in_jurisdiction=True,
    )
    return await save_dr_policy(policy)
