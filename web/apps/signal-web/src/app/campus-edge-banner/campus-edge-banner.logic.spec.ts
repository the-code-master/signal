// Module: web/apps/signal-web/src/app/campus-edge-banner/campus-edge-banner.logic.spec.ts — Unit Tests (FEAT-120)
// Invariants: Verifies CURRENT, SOFT_REFRESH_AVAILABLE (<=N-2), HARD_RELOAD_REQUIRED (>N-2) & LAN Super-Commit queue.
// Spec: architecture/14-universal-scale-accessibility-and-expandability.md, architecture/20-versioning-release-monorepo-build-and-k8s-topology.md
import { describe, expect, it } from "vitest";
import { buildCampusEdgeBannerVm, evaluateWebVersionSkew } from "./campus-edge-banner.logic";

describe("campus-edge-banner.logic", () => {
  it("evaluates N-2 minor version skew states and offline store-and-forward compressed delta bytes", () => {
    expect(evaluateWebVersionSkew("0.1.0", "0.1.0")).toBe("CURRENT");
    expect(evaluateWebVersionSkew("0.1.0", "0.3.0")).toBe("SOFT_REFRESH_AVAILABLE");
    expect(evaluateWebVersionSkew("0.1.0", "0.4.0")).toBe("HARD_RELOAD_REQUIRED");

    const lanVm = buildCampusEdgeBannerVm("CAMPUS_LAN_AIRGAPPED", 3, "0.1.0", "0.1.0");
    expect(lanVm.telemetry.compressedDeltaBytes).toBe(122_880n);
    expect(lanVm.connectivityTone).toBe("warn");
  });
});
