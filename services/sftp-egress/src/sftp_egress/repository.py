# Module: services/sftp-egress/src/sftp_egress/repository.py — Tenant SFTP Registry & Single-Use TOTP Burn Store
# Invariants: Tenant-isolated endpoint lookups; SET NX EX 90 single-use TOTP burn-list blocks same-window replay (F-076).
# Spec: architecture/07-security-encryption-dlp-and-sftp.md
from __future__ import annotations

import uuid

from pydantic import SecretStr

from sftp_egress.schemas import (
    SftpEndpointCreateRequest,
    SftpEndpointView,
    SftpReleaseQueued,
)

TOTP_BURN_TTL_SEC = 90


class SftpEndpointRegistry:
    def __init__(self) -> None:
        self._records: dict[str, tuple[SftpEndpointView, SecretStr]] = {}
        self._seed_defaults()

    def _seed_defaults(self) -> None:
        defaults = (
            SftpEndpointView(
                endpoint_id="ep-fab-ttm-01",
                org_slug="signal-design-inc",
                display_name="TTM Technologies Write-Only Fab Drop",
                host="sftp-ingest.ttm-fab.example",
                port=22,
                remote_drop_dir="/incoming/signal-design-inc",
                pinned_ed25519_fingerprint="SHA256:k8Lp9xQ2mZvN4wR7tY1uI0oP3aS6dF8gH2jK5lZ9xC0",
                target_kind="FAB_GERBER_ODB_IPC2581",
            ),
            SftpEndpointView(
                endpoint_id="ep-lab-tuv-02",
                org_slug="signal-design-inc",
                display_name="TUV Rheinland EMC & ISO-26262 Lab Drop",
                host="sftp-submissions.tuv-lab.example",
                port=22,
                remote_drop_dir="/incoming/compliance",
                pinned_ed25519_fingerprint="SHA256:m2Qw7eR9tY4uI1oP6aS8dF3gH5jK0lZ2xC7vB4nM8qW",
                target_kind="REGULATORY_LAB_EMC_PPAP",
            ),
            SftpEndpointView(
                endpoint_id="ep-mcad-tier1-03",
                org_slug="signal-design-inc",
                display_name="Magna Enclosure Partner AP242 STEP Drop",
                host="sftp-mcad.magna-partner.example",
                port=22,
                remote_drop_dir="/incoming/step-ap242",
                pinned_ed25519_fingerprint="SHA256:p9Lm3nB6vC1xZ8lK4jH7gF2dS5aP0oI9uY6tR3eW1qA",
                target_kind="PARTNER_MCAD_STEP",
            ),
        )
        for view in defaults:
            self._records[view.endpoint_id] = (view, SecretStr("kms:ml-kem-768:wrapped-ed25519-key"))

    def list_for_org(self, org_slug: str) -> list[SftpEndpointView]:
        return [v for v, _k in self._records.values() if v.org_slug == org_slug]

    def get_for_org(self, org_slug: str, endpoint_id: str) -> SftpEndpointView | None:
        hit = self._records.get(endpoint_id)
        if hit is None or hit[0].org_slug != org_slug:
            return None
        return hit[0]

    def register(self, org_slug: str, req: SftpEndpointCreateRequest) -> SftpEndpointView:
        endpoint_id = f"ep-{uuid.uuid4().hex[:10]}"
        view = SftpEndpointView(
            endpoint_id=endpoint_id,
            org_slug=org_slug,
            display_name=req.display_name,
            host=req.host,
            port=req.port,
            remote_drop_dir=req.remote_drop_dir,
            pinned_ed25519_fingerprint=req.pinned_ed25519_fingerprint,
            target_kind=req.target_kind,
        )
        self._records[endpoint_id] = (view, req.kms_wrapped_ssh_key)
        return view


class SingleUseTotpBurnStore:
    """In-memory atomic equivalent of Redis `SET key 1 NX EX 90` (F-076)."""

    def __init__(self) -> None:
        self._burned_until_sec: dict[str, int] = {}

    def set_nx_ex(self, burn_key: str, now_epoch_sec: int, ttl_sec: int = TOTP_BURN_TTL_SEC) -> bool:
        expired = [k for k, exp in self._burned_until_sec.items() if exp <= now_epoch_sec]
        for k in expired:
            del self._burned_until_sec[k]
        if burn_key in self._burned_until_sec:
            return False
        self._burned_until_sec[burn_key] = now_epoch_sec + ttl_sec
        return True


ENDPOINT_REGISTRY = SftpEndpointRegistry()
TOTP_BURN_STORE = SingleUseTotpBurnStore()


async def fetch_sftp_egress_record(org_slug: str) -> SftpReleaseQueued:
    _ = org_slug
    return SftpReleaseQueued(
        release_id="f47ac10b-58cc-4372-a567-0e02b2c3d479",
        status="QUEUED_DISPOSABLE_RUSSH_POD",
        quorum_audit_committed=True,
    )
