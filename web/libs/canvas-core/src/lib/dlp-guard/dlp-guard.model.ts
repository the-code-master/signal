// Module: web/libs/canvas-core/src/lib/dlp-guard/dlp-guard.model.ts — CU-P4-06 Zero-Leak Browser DLP Guard Models
// Invariants: Models shortcut interception, privacy curtain, >3x tile-sweep freeze (F-129) & capture-lock attestation.
// Spec: architecture/07-security-encryption-dlp-and-sftp.md, architecture/16-architectural-audit-flaws-and-futuristic-innovations.md
import type { SigStatusTone } from "@signal/ui-kit";

export type DlpSecurityEventCode =
  | "NONE"
  | "BLOCKED_SHORTCUT_CAPTURE"
  | "BLOCKED_CONTEXT_MENU_SAVE"
  | "BLOCKED_RAW_OS_CLIPBOARD_COPY"
  | "PRIVACY_CURTAIN_ENGAGED"
  | "TILE_SWEEP_RATE_ANOMALY_FREEZE";

export interface TileSweepEvaluation {
  readonly observedTilesPerSec: number;
  readonly thresholdTilesPerSec: number;
  readonly ratioToBaseline: number;
  readonly streamFrozen: boolean;
  readonly eventCode: DlpSecurityEventCode;
}

export interface BrowserCaptureAttestation {
  readonly osScreenCaptureLocked: boolean;
  readonly policyProfile: string;
  readonly badgeText: string;
  readonly tone: SigStatusTone;
}

export interface DlpGuardVm {
  readonly privacyCurtainActive: boolean;
  readonly streamFrozen: boolean;
  readonly lastEventCode: DlpSecurityEventCode;
  readonly blockedAttemptCount: number;
  readonly sweep: TileSweepEvaluation;
  readonly attestation: BrowserCaptureAttestation;
  readonly guardStatusLabel: string;
  readonly guardTone: SigStatusTone;
}
