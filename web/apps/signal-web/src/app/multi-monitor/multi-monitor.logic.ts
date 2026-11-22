// Module: web/apps/signal-web/src/app/multi-monitor/multi-monitor.logic.ts — BroadcastChannel Multi-Monitor Sync & SESSION_WIPE Logic
// Invariants: Enforces 500ms stream heartbeat blackout and zero-persistence SESSION_WIPE across secondary pop-outs (FEAT-092).
// Spec: architecture/06-frontend-angular-and-fastapi-control-plane.md, architecture/07-security-encryption-dlp-and-sftp.md
import type { SigStatusTone } from "@signal/ui-kit";
import type {
  BroadcastSyncMessage,
  MultiMonitorVm,
  SecondaryMonitorPane,
} from "./multi-monitor.model";

export const SIGNAL_MULTIMONITOR_CHANNEL = "signal-zero-leak-multimonitor";

export function isPaneBlackedOut(
  pane: SecondaryMonitorPane,
  nowUtcMs: bigint,
  blackoutThresholdMs: bigint = 500n,
): boolean {
  return pane.focusPrivacyCurtain || nowUtcMs - pane.lastHeartbeatUtcMs > blackoutThresholdMs;
}

export function createSessionWipeBroadcast(nowUtcMs: bigint): BroadcastSyncMessage {
  return {
    kind: "SESSION_WIPE",
    paneId: "*",
    issuedAtUtcMs: nowUtcMs,
  };
}

export function applyMultiMonitorMessage(
  panes: readonly SecondaryMonitorPane[],
  msg: BroadcastSyncMessage,
): readonly SecondaryMonitorPane[] {
  if (msg.kind === "SESSION_WIPE") {
    return [];
  }
  if (msg.kind === "PANE_ATTACH") {
    const exists = panes.some((p) => p.paneId === msg.paneId);
    if (exists) return panes;
    return [
      ...panes,
      {
        paneId: msg.paneId,
        workbenchSlug: "sim-viewer",
        lastHeartbeatUtcMs: msg.issuedAtUtcMs,
        focusPrivacyCurtain: false,
      },
    ];
  }
  if (msg.kind === "HEARTBEAT_SYNC") {
    return panes.map((p) =>
      p.paneId === msg.paneId ? { ...p, lastHeartbeatUtcMs: msg.issuedAtUtcMs } : p,
    );
  }
  return panes.map((p) =>
    p.paneId === msg.paneId ? { ...p, focusPrivacyCurtain: true } : p,
  );
}

export function buildMultiMonitorVm(
  panes: readonly SecondaryMonitorPane[],
  nowUtcMs: bigint,
  wiped: boolean,
): MultiMonitorVm {
  const blackoutCount = panes.filter((p) => isPaneBlackedOut(p, nowUtcMs)).length;
  const tone: SigStatusTone = wiped ? "warn" : blackoutCount > 0 ? "danger" : "ok";
  const summaryBadge = wiped
    ? "SESSION_WIPE Broadcast Sent (0 Secondary Panes)"
    : `Multi-Monitor: ${1 + panes.length}x GPU Atlas (${panes.length} Pop-Out)`;
  const paneLabels = panes.map((p) => {
    const state = isPaneBlackedOut(p, nowUtcMs) ? "BLACKOUT_CURTAIN" : "LIVE_SYNC";
    return `${p.paneId} [/${p.workbenchSlug}] • ${state}`;
  });
  return {
    channelName: SIGNAL_MULTIMONITOR_CHANNEL,
    activePaneCount: panes.length,
    blackoutCount,
    wiped,
    summaryBadge,
    tone,
    paneLabels,
  };
}
