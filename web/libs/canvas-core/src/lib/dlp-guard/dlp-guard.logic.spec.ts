// Module: web/libs/canvas-core/src/lib/dlp-guard/dlp-guard.logic.spec.ts — CU-P4-06 DLP Guard Unit Tests
// Invariants: Verifies shortcut block, blur privacy curtain, >3x tile-sweep anomaly freeze (F-129), and capture attestation.
// Spec: architecture/07-security-encryption-dlp-and-sftp.md
import { describe, expect, it } from "vitest";
import {
  buildDlpGuardVm,
  evaluatePrivacyCurtainActive,
  evaluateTileSweepRate,
  shouldAllowClipboardCopy,
  shouldInterceptDlpKey,
  verifyManagedBrowserCaptureLock,
} from "./dlp-guard.logic";

describe("dlp-guard.logic (CU-P4-06)", () => {
  it("intercepts Ctrl/Cmd+P and Ctrl/Cmd+S, raw clipboard copy, and window blur curtain", () => {
    expect(shouldInterceptDlpKey(true, "p")).toBe(true);
    expect(shouldInterceptDlpKey(true, "S")).toBe(true);
    expect(shouldInterceptDlpKey(false, "p")).toBe(false);
    expect(shouldAllowClipboardCopy(false)).toBe(false);
    expect(shouldAllowClipboardCopy(true)).toBe(true);
    expect(evaluatePrivacyCurtainActive(false, "visible")).toBe(true);
    expect(evaluatePrivacyCurtainActive(true, "hidden")).toBe(true);
    expect(evaluatePrivacyCurtainActive(true, "visible")).toBe(false);
  });

  it("freezes viewport stream when tile-sweep rate exceeds 3x normal pan baseline (F-129)", () => {
    const normal = evaluateTileSweepRate(80, 1, 40);
    expect(normal.streamFrozen).toBe(false);

    const scrapeAttack = evaluateTileSweepRate(140, 1, 40);
    expect(scrapeAttack.streamFrozen).toBe(true);
    expect(scrapeAttack.eventCode).toBe("TILE_SWEEP_RATE_ANOMALY_FREEZE");

    const attested = verifyManagedBrowserCaptureLock("sig-attested-os-capture-lock:v1:cr-ent-01");
    expect(attested.osScreenCaptureLocked).toBe(true);

    const vm = buildDlpGuardVm({
      windowFocused: true,
      visibilityState: "visible",
      tilesInLastSec: 150,
      blockedAttemptCount: 1,
      lastEventCode: "BLOCKED_SHORTCUT_CAPTURE",
      attestationHeader: "sig-attested-os-capture-lock:v1:cr-ent-01",
    });
    expect(vm.streamFrozen).toBe(true);
    expect(vm.guardTone).toBe("danger");
  });
});
