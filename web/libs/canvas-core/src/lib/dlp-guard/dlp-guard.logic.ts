// Module: web/libs/canvas-core/src/lib/dlp-guard/dlp-guard.logic.ts — CU-P4-06 Zero-Leak Browser DLP Guard Logic
// Invariants: Blocks Ctrl/Cmd+P/S, raw clipboard & context-save; freezes stream on >3x tile sweep (F-129).
// Spec: architecture/07-security-encryption-dlp-and-sftp.md, architecture/16-architectural-audit-flaws-and-futuristic-innovations.md
import type { SigStatusTone } from "@signal/ui-kit";
import type {
  BrowserCaptureAttestation,
  DlpGuardVm,
  DlpSecurityEventCode,
  TileSweepEvaluation,
} from "./dlp-guard.model";

export const TILE_SWEEP_ANOMALY_MULTIPLIER = 3;

export function shouldInterceptDlpKey(ctrlOrMeta: boolean, key: string): boolean {
  if (!ctrlOrMeta) return false;
  const lower = key.toLowerCase();
  return lower === "p" || lower === "s";
}

export function shouldAllowClipboardCopy(hasSealedCloudEnvelope: boolean): boolean {
  return hasSealedCloudEnvelope;
}

export function evaluatePrivacyCurtainActive(
  windowFocused: boolean,
  visibilityState: "visible" | "hidden",
): boolean {
  return !windowFocused || visibilityState === "hidden";
}

export function evaluateTileSweepRate(
  tilesRequestedInWindow: number,
  windowDurationSec: number,
  baselinePanTilesPerSec = 40,
): TileSweepEvaluation {
  const safeWindow = Math.max(0.1, windowDurationSec);
  const observed = Number((tilesRequestedInWindow / safeWindow).toFixed(1));
  const threshold = baselinePanTilesPerSec * TILE_SWEEP_ANOMALY_MULTIPLIER;
  const ratio = Number((observed / Math.max(1, baselinePanTilesPerSec)).toFixed(2));
  const streamFrozen = observed > threshold;
  return {
    observedTilesPerSec: observed,
    thresholdTilesPerSec: threshold,
    ratioToBaseline: ratio,
    streamFrozen,
    eventCode: streamFrozen ? "TILE_SWEEP_RATE_ANOMALY_FREEZE" : "NONE",
  };
}

export function verifyManagedBrowserCaptureLock(
  attestationHeader: string,
): BrowserCaptureAttestation {
  const valid = attestationHeader.startsWith("sig-attested-os-capture-lock:v1:");
  return {
    osScreenCaptureLocked: valid,
    policyProfile: valid ? "CHROME_ENTERPRISE_SCREEN_LOCK" : "UNATTESTED_BYOD_WATERMARK_MAX",
    badgeText: valid
      ? "OS Capture Lock: Attested"
      : "OS Capture Unattested (Max Watermark)",
    tone: valid ? "ok" : "warn",
  };
}

export interface DlpGuardStateInput {
  readonly windowFocused: boolean;
  readonly visibilityState: "visible" | "hidden";
  readonly tilesInLastSec: number;
  readonly blockedAttemptCount: number;
  readonly lastEventCode: DlpSecurityEventCode;
  readonly attestationHeader: string;
}

export function buildDlpGuardVm(input: DlpGuardStateInput): DlpGuardVm {
  const privacyCurtainActive = evaluatePrivacyCurtainActive(
    input.windowFocused,
    input.visibilityState,
  );
  const sweep = evaluateTileSweepRate(input.tilesInLastSec, 1, 40);
  const attestation = verifyManagedBrowserCaptureLock(input.attestationHeader);
  const effectiveEvent: DlpSecurityEventCode = sweep.streamFrozen
    ? "TILE_SWEEP_RATE_ANOMALY_FREEZE"
    : privacyCurtainActive
      ? "PRIVACY_CURTAIN_ENGAGED"
      : input.lastEventCode;

  const guardTone: SigStatusTone = sweep.streamFrozen
    ? "danger"
    : privacyCurtainActive || input.blockedAttemptCount > 0
      ? "warn"
      : "ok";

  const guardStatusLabel = sweep.streamFrozen
    ? `DLP STREAM FROZEN (${sweep.ratioToBaseline}x Tile-Sweep > 3x Cap)`
    : privacyCurtainActive
      ? "Privacy Curtain Engaged (Viewport Occluded)"
      : `Zero-Leak DLP Guard Active (${input.blockedAttemptCount} Blocked)`;

  return {
    privacyCurtainActive,
    streamFrozen: sweep.streamFrozen,
    lastEventCode: effectiveEvent,
    blockedAttemptCount: input.blockedAttemptCount,
    sweep,
    attestation,
    guardStatusLabel,
    guardTone,
  };
}
