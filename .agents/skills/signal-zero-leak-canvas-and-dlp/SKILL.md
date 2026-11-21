---
name: signal-zero-leak-canvas-and-dlp
description: Implements Signal's Enterprise Zero-Leak Dual-Plane viewport (Zstd-QOI/WebP dirty tiles + hardware video), HudOverlayPrimitive 120 FPS interactive HUDs, Multi-Monitor BroadcastChannel pop-outs, Tardos + Constellation watermarks, and browser DLP guardrails. Use when working on canvas rendering, viewport streaming, overlays, print blocking, or clipboard isolation.
---

# Signal Zero-Leak Canvas, Dual-Plane Streaming & DLP

Specs: `architecture/03-*` (Sec 3.4), `architecture/06-*` (Sec 1.2–1.4), `architecture/07-*`, `architecture/17-*`.

## 1. Capability-Probed Dual-Plane Viewport (`signal-render` ↔ `web/`)
- **Plane 1 (Precision Static & `80 ms` Settling):** Stream lossless `RGBA8` **`Zstd + QOI` (`> 400 MB/s` GPU shader decode on 100% of browsers) or Lossless `WebP` dirty-rect tiles** for schematics, 2D PCB/IC layout, and `80 ms` post-motion settling — zero chroma blur, zero software-AV1 CPU drain!
- **Plane 2 (60–120 FPS Continuous 3D Orbit/Pan):** Probe `WebCodecs` hardware decode capabilities (`HEVC 4:4:4` on Apple Silicon → **Dual-Luma Hardware `AV1/H.264 4:2:0` Plane Split** reconstructing exact `4:4:4` on NVDEC/Intel GPUs) and snap back to Plane 1 within `80 ms` of motion stop.
- **Single Multi-Pane GPU Atlas:** Composite all open panes (including undocked multi-monitor panes) into one server-side GPU atlas per session so GPU encoder sessions never exhaust.
- **Transport Fallback:** HTTP/3 WebTransport → `turns:443` → HTTP/2 WSS binary tile fallback; buffer uncommitted `GraphOp`s up to `60 s` strictly in a **volatile JS RAM closure** (zero disk persistence!).

## 2. Zero-Leak `HudOverlayPrimitive` & Multi-Monitor Rules
- Under Mode A (Zero-Leak), **never** send raw netlists, polygons, or GDSII geometry to browser JS RAM.
- Render interactive HUD chrome (crosshairs, drag ghosts, potentiometers, meander gauges, KaTeX cards, laser pointers, TDR reticles) in Angular aligned to the GPU atlas via `bigint` nanometer screen-to-world transform matrices; stream throttled `16 ms` intent deltas back to Rust.
- Multi-Monitor pop-outs (`FEAT-092`) coordinate via same-origin volatile `BroadcastChannel` + `SharedWorker`, inheriting identical focus-loss privacy curtains, `COOP`/`COEP` headers, `60 s` rolling stream key heartbeats (`500 ms` blackout on loss), and watermarks.

## 3. Forensic Watermarking & Client DLP Checklist
- Combine dynamic translucent overlays, frequency-domain **Tardos codes**, and **Topological/Geometric Constellation Micro-Perturbations** (`±1–2 px` wire-jog knees, resistor zig-zag phase, label kerning, via-stitch phase) inside `signal-render`.
- Enforce server-side pan/tile-harvesting rate anomaly freeze (`signal-render`), print-blocking CSS (`@media print { display: none !important; }`), focus-loss privacy curtains, and server-brokered Encrypted Cloud Clipboard (`*.sigclip`) with per-repo RBAC checks.
- Verify **ZERO** calls to `localStorage`/`sessionStorage`/`IndexedDB`/Cache API for design IP and **ZERO** Blob download URLs or `window.print()`.
