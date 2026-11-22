// Module: web/libs/cam-print/src/lib/cnc-toolpath-studio/cnc-toolpath-studio.component.ts — Component
// Invariants: <=120 LOC; Zoneless OnPush; templateUrl ONLY; zero local CSS.
// Spec: architecture/19-cad-cam-3d-printing-and-plm-operations.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { ChangeDetectionStrategy, Component, computed, signal } from "@angular/core";
import { SigDataGridComponent, SigPanelComponent, SigStatusPillComponent } from "@signal/ui-kit";
import { buildCncToolpathStudioVm } from "./cnc-toolpath-studio.logic";
import type { CncPostProfile } from "./cnc-toolpath-studio.model";

@Component({
  selector: "sig-cnc-toolpath-studio",
  standalone: true,
  imports: [SigPanelComponent, SigStatusPillComponent, SigDataGridComponent],
  templateUrl: "./cnc-toolpath-studio.component.html",
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class CncToolpathStudioComponent {
  readonly postProfile = signal<CncPostProfile>("SIEMENS_840D");
  readonly scrubPct = signal<number>(100);

  readonly vm = computed(() =>
    buildCncToolpathStudioVm(this.postProfile(), this.scrubPct()),
  );

  selectPost(post: CncPostProfile): void {
    this.postProfile.set(post);
  }

  selectScrub(pct: number): void {
    this.scrubPct.set(pct);
  }
}
