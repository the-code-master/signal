// Module: web/libs/canvas-core/src/lib/dlp-guard/dlp-guard.component.ts — CU-P4-06 Zero-Leak DLP Guardrail Component
// Invariants: <=120 LOC; Zoneless OnPush; templateUrl ONLY; zero local CSS; enforces runtime DLP event interception.
// Spec: architecture/07-security-encryption-dlp-and-sftp.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { ChangeDetectionStrategy, Component, computed, input, output, signal } from "@angular/core";
import { SigStatusPillComponent } from "@signal/ui-kit";
import {
  buildDlpGuardVm,
  shouldAllowClipboardCopy,
  shouldInterceptDlpKey,
} from "./dlp-guard.logic";
import type { DlpSecurityEventCode } from "./dlp-guard.model";

@Component({
  selector: "sig-dlp-guard",
  standalone: true,
  imports: [SigStatusPillComponent],
  templateUrl: "./dlp-guard.component.html",
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class DlpGuardComponent {
  readonly hasSealedClipboard = input<boolean>(false);
  readonly securityAuditEmitted = output<DlpSecurityEventCode>();

  readonly windowFocused = signal<boolean>(true);
  readonly visibilityState = signal<"visible" | "hidden">("visible");
  readonly tilesInLastSec = signal<number>(36);
  readonly blockedCount = signal<number>(0);
  readonly lastEvent = signal<DlpSecurityEventCode>("NONE");
  readonly attestationHeader = signal<string>("sig-attested-os-capture-lock:v1:enterprise-managed");

  readonly vm = computed(() =>
    buildDlpGuardVm({
      windowFocused: this.windowFocused(),
      visibilityState: this.visibilityState(),
      tilesInLastSec: this.tilesInLastSec(),
      blockedAttemptCount: this.blockedCount(),
      lastEventCode: this.lastEvent(),
      attestationHeader: this.attestationHeader(),
    }),
  );

  onKeydown(event: KeyboardEvent): void {
    if (shouldInterceptDlpKey(event.ctrlKey || event.metaKey, event.key)) {
      event.preventDefault();
      this.recordViolation("BLOCKED_SHORTCUT_CAPTURE");
    }
  }

  onContextMenu(event: MouseEvent): void {
    event.preventDefault();
    this.recordViolation("BLOCKED_CONTEXT_MENU_SAVE");
  }

  onCopy(event: ClipboardEvent): void {
    if (!shouldAllowClipboardCopy(this.hasSealedClipboard())) {
      event.preventDefault();
      this.recordViolation("BLOCKED_RAW_OS_CLIPBOARD_COPY");
    }
  }

  onWindowBlur(): void {
    this.windowFocused.set(false);
    this.securityAuditEmitted.emit("PRIVACY_CURTAIN_ENGAGED");
  }

  onWindowFocus(): void {
    this.windowFocused.set(true);
  }

  simulateSweepBurst(tilesPerSec: number): void {
    this.tilesInLastSec.set(tilesPerSec);
    if (tilesPerSec > 120) {
      this.securityAuditEmitted.emit("TILE_SWEEP_RATE_ANOMALY_FREEZE");
    }
  }

  private recordViolation(code: DlpSecurityEventCode): void {
    this.blockedCount.update((c) => c + 1);
    this.lastEvent.set(code);
    this.securityAuditEmitted.emit(code);
  }
}
