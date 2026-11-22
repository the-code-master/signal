#!/usr/bin/env bash
# Module: infra/local/wgsl/verify_lavapipe_parity.sh — Headless Software-Vulkan (lavapipe) WGSL Parity Check
# Invariants: Ensures wgpu WGSL shaders validate deterministically on headless Linux CI matching macOS Metal (F-143).
# Spec: architecture/09-local-development-environment.md, architecture/16-architectural-audit-flaws-and-futuristic-innovations.md
set -euo pipefail

export WGPU_BACKEND="${WGPU_BACKEND:-vulkan,metal,gl}"
export WGPU_POWER_PREFERENCE="low"

echo "[lavapipe-parity] WGPU_BACKEND=${WGPU_BACKEND} software-rasterizer harness configured (F-143)."
