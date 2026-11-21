---
name: signal-universal-parts-registry
description: Builds and maintains Signal's Universal Component, Datasheet, Master Pin Table, Dual-Standard Symbol, Parametric Padstack, IPC-7351 Footprint, 3D STEP Alignment, and 3-Tier AVL Overlay architecture (signal-parts, services/parts-registry, web/libs/library-studio, and contracts/proto/signal/v1/universal_parts.proto). Use whenever working on component libraries, datasheets, symbols, pads, footprints, 3D models, or parts.lock.json pinning.
---

# Signal Universal Component, Datasheet, Symbol & Padstack Architecture

Governing spec: `architecture/18-universal-component-and-datasheet-repository.md`.

## 1. Core Architectural Invariants
1. **3-Tier Federated Resolution (Zero Contamination, Zero Silent Drift):**
   - **Tier 1 (Global Universal):** Sharded Git repos (`parts-padstacks`, `parts-packages`, `parts-symbols-std`, `parts-footprints-std`, `parts-mfr-*`) + BLAKE3 CAS (`cas.signal.parts`) for heavy PDFs/STEP/glTF/AOT `.wasm` blobs.
   - **Tier 2 (Org-Private BYO-Git):** Customer's encrypted `<org>/signal-org-parts` (`ENCRYPTED_VAULT` Padmé `AES-256-GCM-SIV`) hosting private NDA parts AND non-invasive `*.avl.toml` AVL/ERP overlays. NEVER leak Tier 2 into Tier 1!
   - **Tier 3 (Project Lockfile):** Pin exact `(part_uid, semver, manifest_blake3, padstack_blake3, model_blake3)` in `<project>-meta/.signal/parts.lock.json` so global updates never silently mutate shipped boards.
2. **Single Master Pin Table (`pins.json`):** Every schematic symbol pin, PCB padstack instance, 3D STEP lead, SPICE/IBIS port, and MCU/FPGA pinmux entry MUST bind to the exact same immutable `pin_uid`.
3. **First-Class Parametric Padstacks (`PadstackDef`):** NEVER hardcode raw pad polygon soup inside footprints. Reference reusable `padstack_uid` primitives (`i64` nm shapes, thermal spokes, anti-pads, NSMD/SMD mask, IPC-7525 windowpane paste `>= 0.66`, IPC-4761 via fill, dynamic `*.fabprofile.toml` adaptation).
4. **Dual-Standard Symbols & IPC-7351 Density Tiers:** Support IEEE/ANSI + IEC 60617 + De Morgan views from one semantic symbol, and `MOST`/`NOMINAL`/`LEAST` IPC-7351 footprint densities with Zero-Orientation.

## 2. 5-Gate Verification Pipeline (`BRONZE` → `SILVER` → `GOLD`)
- **Gate 1:** Zero-network Dual-VLM + `GBNF` Quarantine extraction (with exact `(doc_blake3, page, bbox_norm)` citations).
- **Gate 2:** Rust `MasterPinBijectionChecker` proving `Datasheet == Symbol == Footprint == SPICE/IBIS`.
- **Gate 3:** Rust `ThreeWayStepPadVerifier` (`i128` geometry) proving 100% 3D STEP metallic lead-to-pad intersection & IPC-7525 paste ratio.
- **Gate 4:** Rust `signal-sim` running `testbench.recipe.json` (`< 2%` RMS curve error → `SILVER_SIM_MATCHED`).
- **Gate 5:** Vendor cryptographic signature or Org Librarian Step-Up 2FA `ML-DSA-65 + Ed25519` attestation (`GOLD_ATTESTED`).

## 3. Multi-Stack Boundaries
- **Rust `1.100+` (`engine/crates/signal-parts`):** `#![forbid(unsafe_code)]`, `ThreeTierResolver`, `PadstackFabAdapter`, `ThreeWayStepPadVerifier`, `PartLockfileManager` (`i64`/`i128` nm, `0.5-ULP`).
- **Python `3.15+` FastAPI (`services/parts-registry`):** Stateless RLS (`PostgreSQL 19+`) parametric search, Org AVL overlay APIs (`Depends(require_org_admin)` + Step-Up 2FA), Cross-Project "Where-Used", and `Campus Edge` read-only global mirror sync.
- **Angular `v22+` (`web/libs/library-studio`):** Zoneless Signals `UniversalPartBrowser`, `MasterPinTableEditor`, `PadstackSculptor`, `ThreeWayAlignmentViewer`, and Zero-Leak streamed `DatasheetCitationViewer` (zero PDF/STEP browser downloads for NDA parts!).

## Dev Response & Anti-Hallucination
- Never recall component pinouts, pad dimensions, or AbsMax specs from memory—ground every value in verified `pins.json`/datasheet citations and `signal-parts` schema checks.
- Reply in **2–3 sentences max** stating ONLY the issue, exact fix applied, and observed verification result.
