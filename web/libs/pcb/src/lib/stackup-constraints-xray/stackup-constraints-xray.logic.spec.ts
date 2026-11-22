// Module: web/libs/pcb/src/lib/stackup-constraints-xray/stackup-constraints-xray.logic.spec.ts — Unit Tests
// Invariants: Verifies HDI microvia <=0.8 aspect ratio check, GPU X-Ray uniform bitmask, and teardrop pour hygiene.
// Spec: architecture/03-rust-eda-engine.md
import { describe, expect, it } from "vitest";
import {
  DEFAULT_HDI_SPANS,
  buildStackupConstraintsXrayVm,
  computeViaAspectRatio,
  computeXrayLayerBitmaskHex,
} from "./stackup-constraints-xray.logic";

describe("stackup-constraints-xray.logic (FEAT-014/018/095/096)", () => {
  it("validates HDI microvia <=0.8 aspect ratio and computes single-key GPU X-Ray bitmask", () => {
    expect(computeViaAspectRatio(DEFAULT_HDI_SPANS[0])).toBe(0.72);
    expect(computeXrayLayerBitmaskHex("PCIE6_DIFF_STRIPLINES")).toBe("0x0330");

    const cleanVm = buildStackupConstraintsXrayVm("HV_SIC_SWITCH_XRAY", true);
    expect(cleanVm.pourHdiTone).toBe("ok");
    expect(cleanVm.gpuLayerBitmaskHex).toBe("0x0003");
  });
});
