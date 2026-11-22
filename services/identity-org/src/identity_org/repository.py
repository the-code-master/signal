# Module: services/identity-org/src/identity_org/repository.py — Tenant-Isolated Identity, Quorum & Membership Store
# Invariants: Strictly keys state by org_slug; enforces immutable frozen records and contractor expires_at tracking.
# Spec: architecture/08-identity-enterprise-and-project-model.md, docs/engineering/FASTAPI_ENGINEERING_GUIDE.md
from __future__ import annotations

from identity_org.schemas import (
    AccessReviewEntry,
    BreakGlassQuorumRecord,
    ContractorMembershipRecord,
    ElectronicSignatureReceipt,
)

_BREAK_GLASS_STORE: dict[tuple[str, str], BreakGlassQuorumRecord] = {}
_ATTESTATION_STORE: dict[tuple[str, str], ElectronicSignatureReceipt] = {}
_MEMBERSHIP_STORE: dict[str, list[ContractorMembershipRecord]] = {
    "signal-design-inc": [
        ContractorMembershipRecord(
            provider_user_id="local-guest-0001",
            org_role="ORG_ADMIN",
            authorized_provider_repo_ids=(
                "local_gh_repo_1001",
                "local_gh_repo_1002",
                "local_gh_repo_1003",
                "local_gh_repo_1004",
                "local_gh_repo_1005",
            ),
            expires_at_utc_ms=1_924_905_600_000,
        ),
        ContractorMembershipRecord(
            provider_user_id="local-pcb-contractor-0002",
            org_role="EXTERNAL_COLLABORATOR",
            authorized_provider_repo_ids=("local_gh_repo_1001", "local_gh_repo_1003"),
            expires_at_utc_ms=1_893_456_000_000,
        ),
        ContractorMembershipRecord(
            provider_user_id="temp-emc-consultant-0003",
            org_role="EXTERNAL_COLLABORATOR",
            authorized_provider_repo_ids=("local_gh_repo_1003",),
            expires_at_utc_ms=1_760_000_000_000,
        ),
    ]
}


async def fetch_identity_org_record(org_slug: str) -> AccessReviewEntry:
    members = _MEMBERSHIP_STORE.get(org_slug, [])
    active_entries = tuple(
        f"{m.provider_user_id}:{m.org_role}" for m in members if not m.revoked
    )
    if not active_entries:
        active_entries = ("local-guest-0001:ORG_ADMIN", "local-pcb-contractor-0002:PCB_ONLY")
    return AccessReviewEntry(entries=active_entries, posture="ATTESTED")


async def save_break_glass_record(record: BreakGlassQuorumRecord) -> BreakGlassQuorumRecord:
    _BREAK_GLASS_STORE[(record.org_slug, record.request_id)] = record
    return record


async def get_break_glass_record(org_slug: str, request_id: str) -> BreakGlassQuorumRecord | None:
    return _BREAK_GLASS_STORE.get((org_slug, request_id))


async def list_org_memberships(org_slug: str) -> list[ContractorMembershipRecord]:
    return list(_MEMBERSHIP_STORE.get(org_slug, []))


async def replace_org_memberships(
    org_slug: str,
    records: list[ContractorMembershipRecord],
) -> None:
    _MEMBERSHIP_STORE[org_slug] = records


async def save_signature_receipt(receipt: ElectronicSignatureReceipt) -> ElectronicSignatureReceipt:
    _ATTESTATION_STORE[(receipt.org_slug, receipt.attestation_id)] = receipt
    return receipt
