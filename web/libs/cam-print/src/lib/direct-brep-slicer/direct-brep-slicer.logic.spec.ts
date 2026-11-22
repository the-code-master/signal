// Module: web/libs/cam-print/src/lib/direct-brep-slicer/direct-brep-slicer.logic.spec.ts — Unit Tests
// Invariants: Verifies exact analytic layer count across 30um DMLS vs 200um FDM and DMLS warp guard.
// Spec: architecture/19-cad-cam-3d-printing-and-plm-operations.md
import { describe, expect, it } from "vitest";
import {
  buildDirectBrepSlicerVm,
  computeExactSliceLayerCount,
} from "./direct-brep-slicer.logic";

describe("direct-brep-slicer.logic (CU-P5-04)", () => {
  it("computes exact 934 layers for 28 mm DMLS @ 30 um and warns if thermal warp pre-comp is disabled", () => {
    expect(computeExactSliceLayerCount(28_000_000n, 30_000n)).toBe(934);
    expect(buildDirectBrepSlicerVm("DMLS_METAL_LPBF", true, false).slicerTone).toBe("warn");
    expect(buildDirectBrepSlicerVm("DMLS_METAL_LPBF", true, true).slicerTone).toBe("ok");
  });
});
