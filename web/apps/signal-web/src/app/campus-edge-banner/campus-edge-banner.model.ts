// Module: web/apps/signal-web/src/app/campus-edge-banner/campus-edge-banner.model.ts — Campus Edge & Skew Models (FEAT-120)
// Invariants: Pure readonly TypeScript types; 64-bit _ms durations and _bytes deltas use bigint ONLY.
// Spec: architecture/14-universal-scale-accessibility-and-expandability.md, architecture/20-versioning-release-monorepo-build-and-k8s-topology.md
import type { SigStatusTone } from "@signal/ui-kit";

export type VersionSkewState = "CURRENT" | "SOFT_REFRESH_AVAILABLE" | "HARD_RELOAD_REQUIRED";
export type CampusConnectivityMode = "CLOUD_CONNECTED" | "CAMPUS_LAN_AIRGAPPED";

export interface CampusEdgeTelemetry {
  readonly applianceId: string;
  readonly connectivity: CampusConnectivityMode;
  readonly offlineDatasheetCount: number;
  readonly cachedOpenPdks: readonly string[];
  readonly queuedSuperCommitCount: number;
  readonly offlineDurationMs: bigint;
  readonly compressedDeltaBytes: bigint;
  readonly clientWebVersion: string;
  readonly gatewayPlatformVersion: string;
  readonly physicsDeterminismEpoch: number;
  readonly skewState: VersionSkewState;
}

export interface CampusEdgeBannerVm {
  readonly telemetry: CampusEdgeTelemetry;
  readonly connectivityTone: SigStatusTone;
  readonly connectivityBadge: string;
  readonly cacheBadge: string;
  readonly queueBadge: string;
  readonly skewTone: SigStatusTone;
  readonly skewBadge: string;
}
