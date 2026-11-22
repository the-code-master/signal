// Module: web/libs/collab-hub/src/lib/project-status-tower/project-status-tower.logic.spec.ts — Unit Tests
// Invariants: Verifies AT_RISK / BLOCKED dependency escalation, consumer team RED RAG propagation & rationale override.
// Spec: architecture/21-collaboration-space-project-status-and-automated-reviews.md
import { describe, expect, it } from "vitest";
import {
  SEEDED_DECISIONS,
  SEEDED_DEPENDENCIES,
  buildProjectStatusTowerVm,
} from "./project-status-tower.logic";

describe("project-status-tower.logic", () => {
  it("evaluates AT_RISK within 48h window and escalates overdue dependencies to BLOCKED + RED RAG", () => {
    const atRiskVm = buildProjectStatusTowerVm(SEEDED_DEPENDENCIES, SEEDED_DECISIONS, 1_795_324_800_000n);
    expect(atRiskVm.overallHealth).toBe("AMBER");
    expect(atRiskVm.dependencyRows[1]?.["status"]).toBe("AT_RISK");

    const overdueDeps = [
      ...SEEDED_DEPENDENCIES,
      {
        dependencyId: "dep-03",
        producerTeam: "DESIGN" as const,
        consumerTeam: "SOFTWARE_FIRMWARE" as const,
        deliverableTitle: "Overdue ADC Calibration LUT",
        dueAtUtcMs: 1_795_000_000_000n,
        handshakeStatus: "PROPOSED" as const,
      },
    ];
    const blockedVm = buildProjectStatusTowerVm(overdueDeps, SEEDED_DECISIONS, 1_795_324_800_000n);
    expect(blockedVm.overallHealth).toBe("RED");
    const fw = blockedVm.teamCards.find((c) => c.team === "SOFTWARE_FIRMWARE")!;
    expect(fw.autoHealth).toBe("RED");
    expect(fw.openBlockers).toBe(1);

    const overriddenVm = buildProjectStatusTowerVm(
      overdueDeps,
      SEEDED_DECISIONS,
      1_795_324_800_000n,
      "Mock LUT active on HIL rig until rev E lands",
    );
    expect(overriddenVm.overallHealth).toBe("AMBER");
  });
});
