# Module: services/session-broker/src/session_broker/schemas.py — Sticky Cell Ticket & Paired-Region DR Schemas
# Invariants: Serializes active_fencing_epoch & ticket expiry as Int64Str; enforces In-Jurisdiction DR pairs (F-142).
# Spec: architecture/04-kubernetes-runtime-and-collaboration.md, architecture/16-*.md
from __future__ import annotations

from typing import Literal

from services.common.schemas import Int64Str, StrictSchema

ResidencyPolicyKind = Literal["IN_JURISDICTION_PAIRED_DR", "SOVEREIGN_SINGLE_REGION_STRICT"]


class SessionLeaseResponse(StrictSchema):
    cell_id: str
    runtime_tier: str
    active_fencing_epoch: Int64Str
    engine_version: str
    negotiated_codec: str
    sticky_webtransport_ticket: str = ""
    ticket_expires_at_utc_ms: Int64Str = 0


class PairedRegionDrPolicyRequest(StrictSchema):
    residency_policy: ResidencyPolicyKind
    primary_region: str
    dr_standby_region: str | None = None


class PairedRegionDrPolicyResponse(StrictSchema):
    org_slug: str
    project_slug: str
    residency_policy: ResidencyPolicyKind
    primary_region: str
    dr_standby_region: str | None
    jurisdiction_code: str
    verified_in_jurisdiction: bool
