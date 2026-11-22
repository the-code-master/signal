// Module: web/libs/academy/src/lib/academy-workbench/academy-workbench.logic.spec.ts — Academy Workbench Tests
// Invariants: Verifies Academy workbench VM construction without Angular TestBed.
// Spec: docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { describe, expect, it } from "vitest";
import { ACADEMY_WORKBENCH_DESCRIPTOR, buildAcademyWorkbenchVm } from "./academy-workbench.logic";

describe("buildAcademyWorkbenchVm", () => {
  it("formats engine badge and latest calculator/auto-fix banner", () => {
    const vm = buildAcademyWorkbenchVm(ACADEMY_WORKBENCH_DESCRIPTOR, "Inserted 10 kΩ SMD");
    expect(vm.engineBadge).toContain("signal-core");
    expect(vm.activityBanner).toBe("Inserted 10 kΩ SMD");
  });
});
