# Module: services/vcs-connector/src/vcs_connector/repository.py — BYO-Git Connection, Pacer Window & ETag Roster Store
# Invariants: Binds repos by immutable provider_repo_id (F-014); tracks per-org 60s creation timestamps for <=40/min cap.
# Spec: architecture/13-vcs-git-server-and-storage-analysis.md
from __future__ import annotations

from vcs_connector.schemas import BoundSubprojectRepo, VcsConnectionStatus

_CONNECTION_STORE: dict[str, VcsConnectionStatus] = {}
_BOUND_REPOS: dict[tuple[str, str, str], BoundSubprojectRepo] = {}
_CREATION_TIMESTAMPS_MS: dict[str, list[int]] = {}
_ROSTER_ETAG_CACHE: dict[tuple[str, str], tuple[str, tuple[str, ...]]] = {}
_REPO_ID_SEQ: dict[str, int] = {}


async def save_vcs_connection(status: VcsConnectionStatus) -> VcsConnectionStatus:
    _CONNECTION_STORE[status.org_slug] = status
    return status


async def fetch_vcs_connector_record(org_slug: str) -> VcsConnectionStatus:
    return _CONNECTION_STORE.get(
        org_slug,
        VcsConnectionStatus(
            provider="GITHUB",
            org_slug=org_slug,
            namespace_kind="ORG_NAMESPACE",
            namespace_handle=org_slug,
            vault_mode="ENCRYPTED_VAULT",
        ),
    )


def allocate_immutable_provider_repo_id(provider: str, org_slug: str) -> str:
    seq = _REPO_ID_SEQ.get(org_slug, 2000) + 1
    _REPO_ID_SEQ[org_slug] = seq
    prefix = provider.lower()[:2]
    return f"{prefix}_repo_id_{org_slug}_{seq}"


async def get_bound_repo(
    org_slug: str,
    project_slug: str,
    subproject_type: str,
) -> BoundSubprojectRepo | None:
    return _BOUND_REPOS.get((org_slug, project_slug, subproject_type))


async def save_bound_repo(repo: BoundSubprojectRepo) -> BoundSubprojectRepo:
    _BOUND_REPOS[(repo.org_slug, repo.project_slug, repo.subproject_type)] = repo
    return repo


async def prune_and_get_window_timestamps(org_slug: str, now_utc_ms: int, window_ms: int = 60_000) -> list[int]:
    current = [ts for ts in _CREATION_TIMESTAMPS_MS.get(org_slug, []) if now_utc_ms - ts < window_ms]
    _CREATION_TIMESTAMPS_MS[org_slug] = current
    return current


async def record_repo_creation_timestamp(org_slug: str, now_utc_ms: int) -> int:
    window = await prune_and_get_window_timestamps(org_slug, now_utc_ms)
    window.append(now_utc_ms)
    _CREATION_TIMESTAMPS_MS[org_slug] = window
    return len(window)


async def get_cached_roster(org_slug: str, provider_repo_id: str) -> tuple[str, tuple[str, ...]] | None:
    return _ROSTER_ETAG_CACHE.get((org_slug, provider_repo_id))


async def save_cached_roster(
    org_slug: str,
    provider_repo_id: str,
    etag: str,
    users: tuple[str, ...],
) -> None:
    _ROSTER_ETAG_CACHE[(org_slug, provider_repo_id)] = (etag, users)
