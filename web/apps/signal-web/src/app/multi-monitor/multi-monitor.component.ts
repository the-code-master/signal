// Module: web/apps/signal-web/src/app/multi-monitor/multi-monitor.component.ts — FEAT-092 Multi-Monitor Manager Component
// Invariants: <=120 LOC; Zoneless OnPush; templateUrl ONLY; broadcasts SESSION_WIPE over volatile BroadcastChannel.
// Spec: architecture/06-frontend-angular-and-fastapi-control-plane.md, architecture/07-security-encryption-dlp-and-sftp.md
import { ChangeDetectionStrategy, Component, computed, type OnDestroy, output, signal } from "@angular/core";
import { SigPanelComponent, SigStatusPillComponent } from "@signal/ui-kit";
import {
  applyMultiMonitorMessage,
  buildMultiMonitorVm,
  createSessionWipeBroadcast,
  SIGNAL_MULTIMONITOR_CHANNEL,
} from "./multi-monitor.logic";
import type { BroadcastSyncMessage, SecondaryMonitorPane } from "./multi-monitor.model";

@Component({
  selector: "sig-multi-monitor-manager",
  standalone: true,
  imports: [SigPanelComponent, SigStatusPillComponent],
  templateUrl: "./multi-monitor.component.html",
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class MultiMonitorManagerComponent implements OnDestroy {
  private readonly channel =
    typeof BroadcastChannel === "function" ? new BroadcastChannel(SIGNAL_MULTIMONITOR_CHANNEL) : null;
  private paneCounter = 1;

  readonly panes = signal<readonly SecondaryMonitorPane[]>([]);
  readonly wiped = signal<boolean>(false);
  readonly nowUtcMs = signal<bigint>(10_000n);
  readonly paneCountChanged = output<number>();

  readonly vm = computed(() => buildMultiMonitorVm(this.panes(), this.nowUtcMs(), this.wiped()));

  constructor() {
    if (this.channel) {
      this.channel.onmessage = (ev: MessageEvent<BroadcastSyncMessage>) => {
        if (ev.data?.kind) {
          this.panes.update((cur) => applyMultiMonitorMessage(cur, ev.data));
          if (ev.data.kind === "SESSION_WIPE") this.wiped.set(true);
          this.paneCountChanged.emit(this.panes().length);
        }
      };
    }
  }

  spawnSecondaryPane(): void {
    this.paneCounter += 1;
    const now = BigInt(Date.now());
    this.nowUtcMs.set(now);
    this.wiped.set(false);
    const msg: BroadcastSyncMessage = {
      kind: "PANE_ATTACH",
      paneId: `mon-${this.paneCounter}`,
      issuedAtUtcMs: now,
    };
    this.panes.update((cur) => applyMultiMonitorMessage(cur, msg));
    this.channel?.postMessage(msg);
    this.paneCountChanged.emit(this.panes().length);
  }

  broadcastSessionWipe(): void {
    const now = BigInt(Date.now());
    const msg = createSessionWipeBroadcast(now);
    this.panes.set([]);
    this.wiped.set(true);
    this.channel?.postMessage(msg);
    this.paneCountChanged.emit(0);
  }

  ngOnDestroy(): void {
    this.channel?.close();
  }
}
