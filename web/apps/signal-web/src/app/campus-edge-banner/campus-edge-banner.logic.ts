// Module: web/apps/signal-web/src/app/campus-edge-banner/campus-edge-banner.logic.ts — Campus Edge & N-2 Skew Logic
// Invariants: Pure deterministic functions <=250 LOC; enforces N-2 minor version compatibility & store-and-forward queue.
// Spec: architecture/14-universal-scale-accessibility-and-expandability.md, architecture/20-versioning-release-monorepo-build-and-k8s-topology.md
import type { SigStatusTone } from "@signal/ui-kit";
import type {
  CampusConnectivityMode,
  CampusEdgeBannerVm,
  CampusEdgeTelemetry,
  VersionSkewState,
} from "./campus-edge-banner.model";

export function parseSemverTriplet(version: string): readonly [number, number, number] {
  const cleaned = version.trim().replace(/^v/i, "");
  const parts = cleaned.split(".");
  const major = Number.parseInt(parts[0] ?? "0", 10);
  const minor = Number.parseInt(parts[1] ?? "0", 10);
  const patch = Number.parseInt(parts[2] ?? "0", 10);
  return [
    Number.isFinite(major) ? major : 0,
    Number.isFinite(minor) ? minor : 0,
    Number.isFinite(patch) ? patch : 0,
  ];
}

export function evaluateWebVersionSkew(
  clientWebVersion: string,
  gatewayPlatformVersion: string,
): VersionSkewState {
  const [cMaj, cMin, cPat] = parseSemverTriplet(clientWebVersion);
  const [gMaj, gMin, gPat] = parseSemverTriplet(gatewayPlatformVersion);
  if (cMaj !== gMaj || gMin - cMin > 2 || cMin > gMin) {
    return "HARD_RELOAD_REQUIRED";
  }
  if (cMin < gMin || cPat < gPat) {
    return "SOFT_REFRESH_AVAILABLE";
  }
  return "CURRENT";
}

export function buildCampusEdgeBannerVm(
  connectivity: CampusConnectivityMode,
  queuedSuperCommits: number,
  clientWebVersion: string,
  gatewayPlatformVersion: string,
): CampusEdgeBannerVm {
  const skewState = evaluateWebVersionSkew(clientWebVersion, gatewayPlatformVersion);
  const offlineDurationMs = connectivity === "CAMPUS_LAN_AIRGAPPED" ? 5_400_000n : 0n;
  const compressedDeltaBytes = BigInt(queuedSuperCommits) * 40_960n;

  const telemetry: CampusEdgeTelemetry = {
    applianceId: "campus-edge-k3s-iit-01",
    connectivity,
    offlineDatasheetCount: 148_500,
    cachedOpenPdks: ["Sky130", "GF180MCU", "IHP-SG13G2"],
    queuedSuperCommitCount: queuedSuperCommits,
    offlineDurationMs,
    compressedDeltaBytes,
    clientWebVersion,
    gatewayPlatformVersion,
    physicsDeterminismEpoch: 1,
    skewState,
  };

  const connectivityTone: SigStatusTone =
    connectivity === "CLOUD_CONNECTED" ? "ok" : "warn";
  const skewTone: SigStatusTone =
    skewState === "HARD_RELOAD_REQUIRED"
      ? "danger"
      : skewState === "SOFT_REFRESH_AVAILABLE"
        ? "warn"
        : "ok";

  return {
    telemetry,
    connectivityTone,
    connectivityBadge:
      connectivity === "CLOUD_CONNECTED"
        ? "Campus Edge: Cloud Synced"
        : "Campus Edge: LAN Air-Gapped Mode",
    cacheBadge: "Offline Cache: 148.5k Datasheets + Sky130/GF180/SG13G2",
    queueBadge: `Store-Forward Queue: ${queuedSuperCommits} Super-Commits (${compressedDeltaBytes.toString()} B)`,
    skewTone,
    skewBadge: `Web v${clientWebVersion} / API v${gatewayPlatformVersion} (${skewState})`,
  };
}
