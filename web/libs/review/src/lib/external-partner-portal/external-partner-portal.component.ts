// Module: web/libs/review/src/lib/external-partner-portal/external-partner-portal.component.ts — FEAT-119 Partner Portal Component
// Invariants: <=120 LOC; Zoneless OnPush; templateUrl ONLY; zero local CSS.
// Spec: architecture/07-security-encryption-dlp-and-sftp.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { ChangeDetectionStrategy, Component, computed, signal } from "@angular/core";
import { SigDataGridComponent, SigPanelComponent, SigStatusPillComponent } from "@signal/ui-kit";
import { buildExternalPartnerPortalVm } from "./external-partner-portal.logic";

@Component({
  selector: "sig-external-partner-portal",
  standalone: true,
  imports: [SigPanelComponent, SigStatusPillComponent, SigDataGridComponent],
  templateUrl: "./external-partner-portal.component.html",
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class ExternalPartnerPortalComponent {
  readonly revokedOrExpired = signal<boolean>(false);

  readonly vm = computed(() =>
    buildExternalPartnerPortalVm(
      this.revokedOrExpired() ? 2_000_000n : 1_000_000n,
      1_900_000n,
    ),
  );

  toggleExpireNow(): void {
    this.revokedOrExpired.update((v) => !v);
  }
}
