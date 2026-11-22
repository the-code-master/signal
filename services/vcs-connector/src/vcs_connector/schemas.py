# Module: services/vcs-connector/src/vcs_connector/schemas.py — BYO-Git Onboarding, <=40/min Pacer & ETag Roster Schemas
# Invariants: StrictSchema (extra="forbid", frozen=True); Int64Str on 64-bit timestamps; immutable provider_repo_id (F-014).
# Spec: architecture/13-vcs-git-server-and-storage-analysis.md, architecture/16-*.md
from __future__ import annotations

from typing import Literal

from services.common.schemas import Int64Str, StrictSchema

VcsProviderKind = Literal["GITHUB", "GITLAB", "BITBUCKET"]
VcsNamespaceKind = Literal["ORG_NAMESPACE", "USER_NAMESPACE"]
VaultEncryptionMode = Literal["ENCRYPTED_VAULT", "PLAIN_CANONICAL"]
MaterializeStatus = Literal["MATERIALIZED", "ALREADY_BOUND", "QUEUED_RATE_LIMITED"]


class VcsOnboardingRequest(StrictSchema):
    provider: VcsProviderKind = "GITHUB"
    namespace_kind: VcsNamespaceKind = "ORG_NAMESPACE"
    namespace_handle: str = "signal-design-inc"
    vault_mode: VaultEncryptionMode = "ENCRYPTED_VAULT"
    custom_ca_pem: str | None = None


class VcsConnectionStatus(StrictSchema):
    provider: VcsProviderKind
    org_slug: str
    namespace_kind: VcsNamespaceKind = "ORG_NAMESPACE"
    namespace_handle: str = "signal-design-inc"
    vault_mode: VaultEncryptionMode
    custom_ca_sha256: str | None = None


class BoundSubprojectRepo(StrictSchema):
    org_slug: str
    project_slug: str
    subproject_type: str
    repo_slug: str
    provider_repo_id: str
    created_at_utc_ms: Int64Str


class BootstrapMetaRepoRequest(StrictSchema):
    now_utc_ms: Int64Str


class LazyMaterializeSubrepoRequest(StrictSchema):
    subproject_type: str
    now_utc_ms: Int64Str


class LazyMaterializeSubrepoResponse(StrictSchema):
    status: MaterializeStatus
    repo_slug: str
    provider_repo_id: str | None
    tokens_used_in_window: int
    max_tokens_per_minute: int
    next_slot_utc_ms: Int64Str | None = None


class RosterSyncRequest(StrictSchema):
    provider_repo_id: str
    if_none_match_etag: str | None = None
    upstream_etag: str
    collaborator_user_ids: tuple[str, ...]


class RosterSyncResponse(StrictSchema):
    provider_repo_id: str
    etag: str
    not_modified: bool
    synced_user_ids: tuple[str, ...]
