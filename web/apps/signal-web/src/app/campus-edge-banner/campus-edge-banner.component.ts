// Module: web/apps/signal-web/src/app/campus-edge-banner/campus-edge-banner.component.ts — Campus Edge Banner UI
// Invariants: <=120 LOC; Zoneless OnPush; templateUrl ONLY; displays LAN store-forward status & What's New trigger.
// Spec: architecture/14-universal-scale-accessibility-and-expandability.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { ChangeDetectionStrategy, Component, computed, input, output, signal } from "@angular/core";
import { SigStatusPillComponent } from "@signal/ui-kit";
import { buildCampusEdgeBannerVm } from "./campus-edge-banner.logic";
import type { CampusConnectivityMode } from "./campus-edge-banner.model";

@Component({
  selector: "sig-campus-edge-banner",
  standalone: true,
  imports: [SigStatusPillComponent],
  templateUrl: "./campus-edge-banner.component.html",
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class CampusEdgeBannerComponent {
  readonly gatewayVersion = input<string>("0.1.0");
  readonly openWhatsNew = output<void>();

  readonly connectivity = signal<CampusConnectivityMode>("CLOUD_CONNECTED");
  readonly queuedSuperCommits = signal<number>(0);
  readonly clientWebVersion = signal<string>("0.1.0");

  readonly vm = computed(() =>
    buildCampusEdgeBannerVm(
      this.connectivity(),
      this.queuedSuperCommits(),
      this.clientWebVersion(),
      this.gatewayVersion(),
    ),
  );

  toggleCampusLanMode(): void {
    if (this.connectivity() === "CLOUD_CONNECTED") {
      this.connectivity.set("CAMPUS_LAN_AIRGAPPED");
      this.queuedSuperCommits.set(2);
    } else {
      this.connectivity.set("CLOUD_CONNECTED");
      this.queuedSuperCommits.set(0);
    }
  }

  triggerWhatsNew(): void {
    this.openWhatsNew.emit();
  }
}
