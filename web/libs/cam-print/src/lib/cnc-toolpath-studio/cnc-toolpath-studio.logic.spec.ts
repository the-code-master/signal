// Module: web/libs/cam-print/src/lib/cnc-toolpath-studio/cnc-toolpath-studio.logic.spec.ts — Unit Tests
// Invariants: Verifies 4-flute chip-load math in bigint nm and gouge-free Dexel scrub progression.
// Spec: architecture/19-cad-cam-3d-printing-and-plm-operations.md
import { describe, expect, it } from "vitest";
import {
  buildCncToolpathStudioVm,
  computeChipLoadNm,
} from "./cnc-toolpath-studio.logic";

describe("cnc-toolpath-studio.logic (CU-P5-04)", () => {
  it("computes exact 60 um/tooth chip-load and stages post-processed G-code for SFTP only", () => {
    expect(computeChipLoadNm(4_320_000_000n, 18_000n, 4n)).toBe(60_000n);
    const vm = buildCncToolpathStudioVm("SIEMENS_840D", 100);
    expect(vm.dexelRemovedPct).toBe(64.2);
    expect(vm.holderGougeCount).toBe(0);
    expect(vm.gcodeSftpDigest).toContain("siemens_840d");
  });
});
