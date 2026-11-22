// Module: web/libs/sftp-release/src/lib/sftp-release-workbench/sftp-release-workbench.component.ts — Wizard Component
// Invariants: <=120 LOC; Zoneless OnPush; templateUrl ONLY; zero local CSS; zero browser file downloads.
// Spec: architecture/07-security-encryption-dlp-and-sftp.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { ChangeDetectionStrategy, Component, computed, input, signal } from "@angular/core";
import type { WorkbenchDescriptor } from "@signal/canvas-core";
import { SigDataGridComponent, SigPanelComponent, SigStatusPillComponent } from "@signal/ui-kit";
import { buildSftpReleaseWorkbenchVm } from "./sftp-release-workbench.logic";
import type {
  SftpBundleDestination,
  SftpWizardStep,
  StepUpAuthKind,
} from "./sftp-release-workbench.model";

@Component({
  selector: "sig-sftp-release-workbench",
  standalone: true,
  imports: [SigPanelComponent, SigStatusPillComponent, SigDataGridComponent],
  templateUrl: "./sftp-release-workbench.component.html",
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class SftpReleaseWorkbenchComponent {
  readonly workbench = input.required<WorkbenchDescriptor>();
  readonly activeStep = signal<SftpWizardStep>("STEP_1_PRE_RELEASE_GATES");
  readonly destination = signal<SftpBundleDestination>("FAB_GERBER_ODB_IPC2581");
  readonly authKind = signal<StepUpAuthKind>("WEBAUTHN_FIDO2");
  readonly assertionAgeSec = signal<bigint>(12n);
  readonly totpBurned = signal<boolean>(false);

  readonly vm = computed(() =>
    buildSftpReleaseWorkbenchVm(
      this.workbench(),
      this.activeStep(),
      this.destination(),
      this.authKind(),
      this.assertionAgeSec(),
      this.totpBurned(),
    ),
  );

  selectStep(step: SftpWizardStep): void {
    this.activeStep.set(step);
  }

  selectDestination(dest: SftpBundleDestination): void {
    this.destination.set(dest);
  }

  selectAuth(kind: StepUpAuthKind): void {
    this.authKind.set(kind);
    this.totpBurned.set(false);
    this.assertionAgeSec.set(12n);
  }

  dispatchAndBurnNonce(): void {
    if (this.vm().canDispatch) {
      this.activeStep.set("STEP_3_ATOMIC_RECEIPT");
      if (this.authKind() === "RFC6238_BURNED_TOTP") {
        this.totpBurned.set(true);
      }
    }
  }
}
