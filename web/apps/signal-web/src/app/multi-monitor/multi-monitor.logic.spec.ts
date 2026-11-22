// Module: web/apps/signal-web/src/app/multi-monitor/multi-monitor.logic.spec.ts — FEAT-092 Multi-Monitor Unit Tests
// Invariants: Verifies PANE_ATTACH, >500ms heartbeat blackout curtain, and instant SESSION_WIPE purge.
// Spec: architecture/07-security-encryption-dlp-and-sftp.md
import { describe, expect, it } from "vitest";
import {
  applyMultiMonitorMessage,
  buildMultiMonitorVm,
  createSessionWipeBroadcast,
  isPaneBlackedOut,
} from "./multi-monitor.logic";

describe("MultiMonitor BroadcastChannel manager (FEAT-092)", () => {
  it("attaches secondary panes, triggers 500ms blackout on heartbeat loss, and wipes on SESSION_WIPE", () => {
    const attached = applyMultiMonitorMessage([], {
      kind: "PANE_ATTACH",
      paneId: "mon-2",
      issuedAtUtcMs: 1_000n,
    });
    expect(attached.length).toBe(1);
    expect(isPaneBlackedOut(attached[0], 1_400n)).toBe(false);
    expect(isPaneBlackedOut(attached[0], 1_600n)).toBe(true);

    const wipeMsg = createSessionWipeBroadcast(2_000n);
    const wiped = applyMultiMonitorMessage(attached, wipeMsg);
    expect(wiped.length).toBe(0);
    expect(buildMultiMonitorVm(wiped, 2_000n, true).summaryBadge).toContain("SESSION_WIPE");
  });
});
