// Module: web/libs/library-studio/src/lib/curve-sdk-snippets/curve-sdk-snippets.component.ts — FEAT-073, 074, 079 Component
// Invariants: <=120 LOC; Zoneless OnPush; templateUrl ONLY; zero local CSS.
// Spec: architecture/18-universal-component-and-datasheet-repository.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { ChangeDetectionStrategy, Component, computed, output, signal } from "@angular/core";
import { SigDataGridComponent, SigPanelComponent, SigStatusPillComponent } from "@signal/ui-kit";
import { buildCurveSdkSnippetsVm } from "./curve-sdk-snippets.logic";
import type { CurveModelFamily } from "./curve-sdk-snippets.model";

@Component({
  selector: "sig-curve-sdk-snippets",
  standalone: true,
  imports: [SigPanelComponent, SigStatusPillComponent, SigDataGridComponent],
  templateUrl: "./curve-sdk-snippets.component.html",
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class CurveSdkSnippetsComponent {
  readonly families: readonly CurveModelFamily[] = ["SIC_RDS_ON_TJ", "MLCC_DC_BIAS", "FOSTER_RTH_CTH"];
  readonly offsets: readonly number[] = [200, 400, 600];

  readonly activeFamily = signal<CurveModelFamily>("SIC_RDS_ON_TJ");
  readonly selectedSnippetId = signal<string>("snip-sic-kelvin-gate");
  readonly refdesOffset = signal<number>(400);
  readonly snippetStamped = output<string>();

  readonly vm = computed(() =>
    buildCurveSdkSnippetsVm(
      this.activeFamily(),
      this.selectedSnippetId(),
      this.refdesOffset(),
    ),
  );

  selectFamily(family: CurveModelFamily): void {
    this.activeFamily.set(family);
  }

  selectOffset(offset: number): void {
    this.refdesOffset.set(offset);
  }

  stampSnippet(snippetId: string): void {
    this.selectedSnippetId.set(snippetId);
    this.snippetStamped.emit(`${snippetId} @ base ${this.refdesOffset()}`);
  }
}
