// Module: web/libs/ui-kit/src/panel/sig-panel.model.ts — SigPanel View-Model Contract
// Invariants: Immutable readonly interface; zero Angular runtime imports.
// Spec: docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
export interface SigPanelVm {
  readonly heading: string;
  readonly subheading: string;
  readonly hasBadge: boolean;
  readonly badgeText: string;
}
