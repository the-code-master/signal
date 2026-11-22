# Module: services/vcs-connector/src/vcs_connector/service.py — Multi-Provider Onboarding, <=40/min Lazy Pacer & ETag Sync
# Invariants: Enforces <project>-meta first + <=40 repo creations/min token bucket (F-120); validates custom PEM CA (F-094).
# Spec: architecture/13-vcs-git-server-and-storage-analysis.md, architecture/16-*.md
from __future__ import annotations

import hashlib

from fastapi import HTTPException

from vcs_connector.repository import (
    allocate_immutable_provider_repo_id,
    fetch_vcs_connector_record,
    get_bound_repo,
    get_cached_roster,
    prune_and_get_window_timestamps,
    record_repo_creation_timestamp,
    save_bound_repo,
    save_cached_roster,
    save_vcs_connection,
)
from vcs_connector.schemas import (
    BootstrapMetaRepoRequest,
    BoundSubprojectRepo,
    LazyMaterializeSubrepoRequest,
    LazyMaterializeSubrepoResponse,
    RosterSyncRequest,
    RosterSyncResponse,
    VcsConnectionStatus,
    VcsOnboardingRequest,
)

MAX_REPO_CREATIONS_PER_MIN = 40
PACER_WINDOW_MS = 60_000


def validate_custom_ca_pem_fingerprint(pem_text: str) -> str:
    stripped = pem_text.strip()
    if not stripped.startswith("-----BEGIN CERTIFICATE-----") or not stripped.endswith("-----END CERTIFICATE-----"):
        raise HTTPException(status_code=400, detail="Invalid custom Root CA PEM envelope (F-094)")
    digest = hashlib.sha256(stripped.encode("utf-8")).hexdigest()
    return f"sha256:{digest}"


async def execute_vcs_connector(
    org_slug: str,
    onboarding: VcsOnboardingRequest | None = None,
) -> VcsConnectionStatus:
    if onboarding is None:
        return await fetch_vcs_connector_record(org_slug)
    ca_fp: str | None = None
    if onboarding.custom_ca_pem is not None:
        ca_fp = validate_custom_ca_pem_fingerprint(onboarding.custom_ca_pem)
    status = VcsConnectionStatus(
        provider=onboarding.provider,
        org_slug=org_slug,
        namespace_kind=onboarding.namespace_kind,
        namespace_handle=onboarding.namespace_handle,
        vault_mode=onboarding.vault_mode,
        custom_ca_sha256=ca_fp,
    )
    return await save_vcs_connection(status)


async def bootstrap_project_meta_repo(
    org_slug: str,
    project_slug: str,
    req: BootstrapMetaRepoRequest,
) -> BoundSubprojectRepo:
    existing = await get_bound_repo(org_slug, project_slug, "META")
    if existing is not None:
        return existing
    conn = await fetch_vcs_connector_record(org_slug)
    provider_repo_id = allocate_immutable_provider_repo_id(conn.provider, org_slug)
    await record_repo_creation_timestamp(org_slug, req.now_utc_ms)
    bound = BoundSubprojectRepo(
        org_slug=org_slug,
        project_slug=project_slug,
        subproject_type="META",
        repo_slug=f"{project_slug}-meta",
        provider_repo_id=provider_repo_id,
        created_at_utc_ms=req.now_utc_ms,
    )
    return await save_bound_repo(bound)


async def materialize_lazy_subrepo(
    org_slug: str,
    project_slug: str,
    req: LazyMaterializeSubrepoRequest,
) -> LazyMaterializeSubrepoResponse:
    subtype = req.subproject_type.strip().upper()
    repo_slug = f"{project_slug}-{subtype.lower().replace('_', '-')}"
    meta = await get_bound_repo(org_slug, project_slug, "META")
    if meta is None:
        raise HTTPException(
            status_code=409,
            detail=f"Lazy repo pacer requires `{project_slug}-meta` to be bootstrapped first (F-120)",
        )
    existing = await get_bound_repo(org_slug, project_slug, subtype)
    window = await prune_and_get_window_timestamps(org_slug, req.now_utc_ms, PACER_WINDOW_MS)
    if existing is not None:
        return LazyMaterializeSubrepoResponse(
            status="ALREADY_BOUND",
            repo_slug=existing.repo_slug,
            provider_repo_id=existing.provider_repo_id,
            tokens_used_in_window=len(window),
            max_tokens_per_minute=MAX_REPO_CREATIONS_PER_MIN,
        )
    if len(window) >= MAX_REPO_CREATIONS_PER_MIN:
        oldest_ts = window[0]
        return LazyMaterializeSubrepoResponse(
            status="QUEUED_RATE_LIMITED",
            repo_slug=repo_slug,
            provider_repo_id=None,
            tokens_used_in_window=len(window),
            max_tokens_per_minute=MAX_REPO_CREATIONS_PER_MIN,
            next_slot_utc_ms=oldest_ts + PACER_WINDOW_MS,
        )
    conn = await fetch_vcs_connector_record(org_slug)
    provider_repo_id = allocate_immutable_provider_repo_id(conn.provider, org_slug)
    used = await record_repo_creation_timestamp(org_slug, req.now_utc_ms)
    await save_bound_repo(
        BoundSubprojectRepo(
            org_slug=org_slug,
            project_slug=project_slug,
            subproject_type=subtype,
            repo_slug=repo_slug,
            provider_repo_id=provider_repo_id,
            created_at_utc_ms=req.now_utc_ms,
        )
    )
    return LazyMaterializeSubrepoResponse(
        status="MATERIALIZED",
        repo_slug=repo_slug,
        provider_repo_id=provider_repo_id,
        tokens_used_in_window=used,
        max_tokens_per_minute=MAX_REPO_CREATIONS_PER_MIN,
    )


async def sync_repo_roster_conditional_etag(
    org_slug: str,
    req: RosterSyncRequest,
) -> RosterSyncResponse:
    cached = await get_cached_roster(org_slug, req.provider_repo_id)
    if cached is not None and req.if_none_match_etag is not None and cached[0] == req.if_none_match_etag:
        return RosterSyncResponse(
            provider_repo_id=req.provider_repo_id,
            etag=cached[0],
            not_modified=True,
            synced_user_ids=cached[1],
        )
    await save_cached_roster(org_slug, req.provider_repo_id, req.upstream_etag, req.collaborator_user_ids)
    return RosterSyncResponse(
        provider_repo_id=req.provider_repo_id,
        etag=req.upstream_etag,
        not_modified=False,
        synced_user_ids=req.collaborator_user_ids,
    )
