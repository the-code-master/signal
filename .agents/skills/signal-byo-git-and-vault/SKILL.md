---
name: signal-byo-git-and-vault
description: Implements Signal's unified Customer-Owned BYO-Git integration (GitHub, GitLab, Bitbucket), immutable provider_repo_id tracking, lazy sub-repo materialization, Padme-padded path-blinded AES-256-GCM-SIV ENCRYPTED_VAULT encryption, Dual-Fenced Two-Phase Super-Commit, and vcs-auditor checks. Use when modifying signal-git-sync, vcs-connector, or vcs-auditor.
---

# Signal BYO-Git, Padmé-Blinded Vault & Dual-Fenced `Super-Commit`

Governing spec: `architecture/13-vcs-git-server-and-storage-analysis.md`, `architecture/07-*`.

## 1. Multi-Provider Unified Adapter (`GitProviderAdapter`)
- Support **GitHub** (Cloud/GHEC/GHES), **GitLab** (Cloud/Self-Managed EE Groups), and **Bitbucket** (Cloud Workspaces / Data Center Projects) across both `ORG_NAMESPACE` and student `USER_NAMESPACE`.
- Bind every repository by **immutable `provider_repo_id`** (never rely solely on mutable repo slugs/names).
- **Zero Permanent Signal File Custody:** Never store permanent customer design files on Signal storage; never delete customer repositories on detach.
- **Lazy Sub-Repo Materialization & Pacing:** Project creation creates ONLY `<project>-meta` synchronously; child `<project>-<type>` repos materialize lazily on first commit through an adaptive token-bucket pacer (`<= 40 repo creations/min`) honoring GitHub's `80/min` secondary abuse limit.
- **Hydration & Live Push Rebase:** Clone via **blobless partial clone (`--filter=blob:none`)** with `gitoxide` + `jj-lib`; stream multi-GB LFS blobs via `64 KB` AEAD chunks (`< 16 MB` constant RAM); auto-rebase external `git push` commits on dirty-hint webhook verification.

## 2. Unified Padmé-Padded, Path-Blinded `AES-256-GCM-SIV` Vault
- Default Enterprise repos to `ENCRYPTED_VAULT` (enforcing `MUST_ENCRYPT` taint gate so NDA IP can never leak into `PLAIN_CANONICAL`).
- Derive per-entity keys locally via **2-level `HKDF-SHA256` in `SecretBox` RAM** (KMS unwraps `DEK_vN` once at Pod Hydration — zero KMS calls per commit!).
- Encrypt entity chunks using **RFC 8452 `AES-256-GCM-SIV`** with:
  1. **Padmé Length-Hiding Padding** (`<= 12%` overhead, defeating standard-cell byte-size fingerprinting).
  2. **HMAC-Blinded Git Paths (`blobs/b7/3f...sigv`)** (zero plaintext schematic/module filenames on GitHub/GitLab).
  3. **`ML-DSA-65`-Signed `.signal/vault.merkle.sig`** binding all `(path, chunk_hash, dek_version)` tuples at commit root (defeating single-chunk rollback swaps).

## 3. Dual-Fenced Two-Phase Cross-Repo `Super-Commit`
1. **Gate 1 (Active Proxy Fencing):** Attach signed `X-Signal-Fencing-Token` to every outbound Git write; `signal-egress-proxy` hard-drops (`412`) stale zombie pods before packets leave Signal.
2. **Gate 2 (Atomic Coordinator CAS):** Push member `refs/signal/txn/<id>` prepare refs → commit signed `.signal/repos.lock.json` to `<project>-meta` **FIRST via strict Git `old_oid -> new_oid` CAS** → fast-forward member branch refs.
3. Restrict Continuity WAL (`<= 72h`) to BYOK/HYOK orgs wrapped strictly under Customer BYOK with `ML-DSA`-signed Crypto-Shred Receipts on Git commit.
4. Allocate 70/30 provider API rate-limit budget (70% interactive sync, 30% `services/vcs-auditor`).
