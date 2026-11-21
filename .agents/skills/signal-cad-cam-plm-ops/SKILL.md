---
name: signal-cad-cam-plm-ops
description: Builds Signal's integrated Mechanical CAD (CAD_MCAD), subtractive CNC CAM (CAM_CNC), additive 3D Printing (PRINT_3D), unified 3D visualization, and native PLM/Operations change-control & supply-chain suite (eBOM/mBOM/sBOM, ECN/ECO, MCN, PCN, Deviation/Waiver, NCR/CAPA, CTB) across signal-mcad, signal-cam, signal-plm, services/plm-ops, and web/libs/{mcad-studio,cam-print,plm-ops}. Use whenever working on mechanical CAD, CNC/3D printing, BOMs, change notices, deviations, or factory supply-chain workflows.
---

# Signal Integrated CAD/CAM, 3D Printing & Native PLM Operations

Governing spec: `architecture/19-cad-cam-3d-printing-and-plm-operations.md`.

## 1. First-Class Project Types & Atomic `Super-Commit`
- Support `CAD_MCAD` (`<project>-cad`), `CAM_CNC` (`<project>-cam`), `PRINT_3D` (`<project>-3dprint`), and `PLM_OPS` (`<project>-ops`) as native Sub-Project types synchronized via Dual-Fenced Two-Phase `Super-Commit` (`repos.lock.json`) under customer BYO-Git (`ENCRYPTED_VAULT` Padmé `AES-256-GCM-SIV`).
- Keep live parametric anchors between `signal-pcb` (mounting holes, connectors, board outline, thermal zones) and `signal-mcad` (enclosures, heatsinks, brackets) so ECAD↔MCAD edits never require manual STEP round-trips.

## 2. Pure-Rust CAD/CAM, Slicing & PLM Engine (`engine/crates/`)
- **`signal-mcad` (`#![forbid(unsafe_code)]`):** Exact `i64`/`i128` nm 2D constraint sketcher, parametric B-Rep solids, sheet-metal K-factor unfolding, assembly kinematic mates, and watertight `STEP AP242` / `wgpu` LOD generation in RAM.
- **`signal-cam` (`#![forbid(unsafe_code)]`):** 2.5D/3/5-axis adaptive CNC toolpaths, voxel/dexel gouge & fixture BVH collision verification, signed post-processors, AND native direct-B-Rep 3D slicing (FDM/SLA/SLS/DMLS, auto-orientation, tree/metal supports, gyroid infill, embedded-PCB pause-insert layers).
- **`signal-plm` (`#![forbid(unsafe_code)]`):** Deterministic multi-view `eBOM`/`mBOM`/`sBOM`/`Costed BOM` compiler, cross-domain blast-radius analyzer, and cryptographic CCB state-machine verifier (`ML-DSA-65 + Ed25519`).
- **Zero-Leak Egress:** Generate all `STEP AP242`, `DXF`, `G-code`, `3MF`, and turnkey CM packages strictly in volatile Pod RAM and deliver ONLY via Step-Up 2FA write-only `russh` SFTP.

## 3. Native PLM Workflows, Control Plane & Angular UI
- **`services/plm-ops`:** Enforce `@Require*` uniform-`404`, PostgreSQL RLS (`NOBYPASSRLS`), Step-Up 2FA CCB sign-offs, and **Fail-Closed 3x Quorum Tier-0 Audit Outbox** on every `ECR`/`ECO`/`ECN`, `MCN`, `PCN`/`PDN`, `Deviation`/`Waiver` (enforcing hard `max_quantity` & `expires_at_utc` release gates!), and `NCR`/`CAPA` transition; route ERP/MES webhooks strictly via `signal-egress-proxy`.
- **`web/libs/{mcad-studio,cam-print,plm-ops}`:** Zoneless Signals standalone components, `bigint` nm dimensions, `HudOverlayPrimitive` sketch/toolpath/slicer overlays, `cdk-virtual-scroll` BOM/CCB/CTB grids, interactive 3D factory work instructions, and zero browser disk IP storage or downloads.

## Dev Response & Anti-Hallucination
- Verify exact `mcad_cam_print.proto`, `plm_ops.proto`, schema fields, and `Cargo.toml`/`package.json` deps via `rg`/`view_file` before editing; never claim builds/tests pass without observed exit code `0`.
- Reply in **2–3 sentences max** stating ONLY the issue, exact fix applied, and verification result.
