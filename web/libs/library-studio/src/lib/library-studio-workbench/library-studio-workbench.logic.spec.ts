// Module: web/libs/library-studio/src/lib/library-studio-workbench/library-studio-workbench.logic.spec.ts — Workbench Tests
// Invariants: Verifies 3 Library Studio tabs and stamp banner state without Angular TestBed.
// Spec: docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { describe, expect, it } from "vitest";
import {
  buildLibraryStudioWorkbenchVm,
  LIBRARY_STUDIO_WORKBENCH_DESCRIPTOR,
} from "./library-studio-workbench.logic";

describe("buildLibraryStudioWorkbenchVm", () => {
  it("exposes 3 Library Studio tabs and reflects stamped snippet telemetry", () => {
    const vm = buildLibraryStudioWorkbenchVm(
      LIBRARY_STUDIO_WORKBENCH_DESCRIPTOR,
      "PADSTACK_STEP",
      "snip-sic-kelvin-gate @ base 400",
    );
    expect(vm.tabs.length).toBe(3);
    expect(vm.activeTab).toBe("PADSTACK_STEP");
    expect(vm.lastStampNote).toContain("base 400");
  });
});
