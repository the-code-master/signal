---
name: signal-security-isolation-audit
description: Audits Signal code, PRs, Kubernetes manifests, API routes, crypto flows, and sandboxes against OWASP ASVS L3 and the "1 Project = 1 Isolated Security Cell" architecture. Use before merging any security-sensitive change or when designing auth, encryption, GPU isolation, SFTP egress, or parser sandboxes.
---

# Signal Security & Complete Project Isolation Auditor

Governing specs: `architecture/07-*`, `architecture/15-*`, `architecture/16-*`.

## 7-Ring Audit Checklist (Every Item Must Pass)

1. **Tenant & Database Isolation:**
   - Every tenant query runs under a `NOBYPASSRLS` role inside a transaction setting `app.org_id` / `app.user_id`.
   - `pgvector` queries hit `datasheet_chunks_global` plus ONLY the caller's dedicated enterprise partition (`<= 500`/cluster) or 64-bucket hash partition (`hnsw.iterative_scan`).
   - All unauthorized UUID/resource lookups return **uniform `404 Not Found`**.

2. **Compute Cell & GPU Silicon Reality:**
   - Every active project gets a dedicated `< 8 MB` `signal-cell-wasm` isolate (`EXPLORER`) or Kata/gVisor / `CONFIDENTIAL_SILICON` (SEV-SNP/TDX) pod — NEVER shared across projects.
   - GPU isolation follows real silicon rules: **Exclusive Whole-GPU Hopper `H100 CC`** (NEVER claim simultaneous `MIG + CC` on H100!) OR **Blackwell `B200+ TEE-I/O` Confidential MIG** (verified via `NRAS` SPDM) OR standard Hardware MIG (never time-sliced multi-tenant GPU sharing).
   - **Cilium eBPF** default-deny drops 100% of Project Pod ↔ Project Pod lateral traffic; NATS uses per-project JWT/NKey subjects.

3. **Cryptography & Key Lifecycle:**
   - Hybrid Post-Quantum envelopes (`ML-KEM-768 + X25519`, `ML-DSA-65 + Ed25519`), unique per-project DEK + 2-level `HKDF` RAM keyring (`SecretBox<T>` + `ZeroizeOnDrop`), and per-project 60-min SPIFFE SVIDs.
   - Strictly In-Jurisdiction Paired-Region DR (`Frankfurt <-> Paris`, `GovCloud-W <-> E`, `Mumbai <-> Hyderabad`).

4. **Network, SSRF & Foreign-Binary Sandboxing:**
   - All outbound Git/SFTP/webhook egress passes through socket-pinned + `X-Signal-Fencing-Token` epoch-enforcing `signal-egress-proxy` with pinned Custom Root CAs.
   - Untrusted PDFs/CAD/GDSII/STEP parsers run in zero-network sandboxes capped at `<= 20:1` compression ratio, `0` XXE, and `<= 64` hierarchy depth.
   - Binary `IBIS-AMI`/`OSDI` `.so` models execute ONLY inside ephemeral zero-network child `Firecracker`/`gVisor` IPC sandboxes (never `dlopen` or link LLVM inside `signal-pod`).

5. **2FA, SFTP Egress & Fail-Closed Audit:**
   - Step-Up 2FA enforces WebAuthn or Redis-burned single-use TOTP; SFTP egress uses disposable write-only `russh` pods (`.tmp.part` → SHA-256 verify → `rename`, zero `SSH_FXP_READ`).
   - Tier-0 security actions commit synchronously to the 3x Quorum Audit Outbox (`503` fail-closed) using pseudonymous IDs (GDPR crypto-shreddable PII outside WORM chains).

6. **Browser & Supply Chain:**
   - Route-Partitioned `COOP`/`COEP` + `__Host-` `SameSite=Strict` cookies; zero `bypassSecurityTrust*`, `eval`, `child_process.exec`, or browser disk design storage; `cargo vet`/`cargo deny` + `pnpm audit --ignore-scripts` clean.
