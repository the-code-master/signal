// Module: web/apps/signal-web/src/app/multi-monitor/multi-monitor.model.ts — FEAT-092 BroadcastChannel Secondary Viewport Models
// Invariants: Zero-Leak secondary panes enforce 500ms heartbeat blackout and instant SESSION_WIPE on logout/expiry.
// Spec: architecture/06-frontend-angular-and-fastapi-control-plane.md, architecture/07-security-encryption-dlp-and-sftp.md
import type { SigStatusTone } from "@signal/ui-kit";

export type MultiMonitorEventKind = "PANE_ATTACH" | "HEARTBEAT_SYNC" | "PRIVACY_CURTAIN" | "SESSION_WIPE";

export interface SecondaryMonitorPane {
  readonly paneId: string;
  readonly workbenchSlug: string;
  readonly lastHeartbeatUtcMs: bigint;
  readonly focusPrivacyCurtain: boolean;
}

export interface BroadcastSyncMessage {
  readonly kind: MultiMonitorEventKind;
  readonly paneId: string;
  readonly issuedAtUtcMs: bigint;
}

export interface MultiMonitorVm {
  readonly channelName: string;
  readonly activePaneCount: number;
  readonly blackoutCount: number;
  readonly wiped: boolean;
  readonly summaryBadge: string;
  readonly tone: SigStatusTone;
  readonly paneLabels: readonly string[];
}
