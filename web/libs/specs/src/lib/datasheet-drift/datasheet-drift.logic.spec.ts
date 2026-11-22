// Module: web/libs/specs/src/lib/datasheet-drift/datasheet-drift.logic.spec.ts — FEAT-078 Unit Tests
// Invariants: Verifies Rev A -> Rev B pin/AbsMax regression detection and lockfile acknowledgement state.
// Spec: architecture/05-ai-agent-and-datasheet-rag.md
import { describe, expect, it } from "vitest";
import {
  DEFAULT_DATASHEET_DRIFT_NOTICES,
  buildDatasheetDriftVm,
  computeAbsMaxRegressionUv,
} from "./datasheet-drift.logic";

describe("datasheet-drift.logic (FEAT-078)", () => {
  it("flags danger tone when unacknowledged revision drift mutates pinouts or tightens AbsMax", () => {
    const vm = buildDatasheetDriftVm("DRIFT-UCC21750-REVD", new Set());
    expect(vm.summaryTone).toBe("danger");
    expect(vm.unacknowledgedCount).toBe(2);
    expect(computeAbsMaxRegressionUv(DEFAULT_DATASHEET_DRIFT_NOTICES[0])).toBe(3_000_000n);
  });

  it("transitions to ok tone once all revision notices are re-verified and pinned", () => {
    const ack = new Set(["DRIFT-UCC21750-REVD", "DRIFT-C3M0016120K-REV5"]);
    const vm = buildDatasheetDriftVm("DRIFT-C3M0016120K-REV5", ack);
    expect(vm.summaryTone).toBe("ok");
    expect(vm.unacknowledgedCount).toBe(0);
    expect(vm.rows[0]?.["status"]).toBe("LOCKED_REV_NEW");
  });
});
