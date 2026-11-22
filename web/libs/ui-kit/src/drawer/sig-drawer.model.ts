// Module: web/libs/ui-kit/src/drawer/sig-drawer.model.ts — SigDrawer View-Model Contract
// Invariants: Immutable readonly interface; zero Angular imports.
// Spec: docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
export interface SigDrawerVm {
  readonly open: boolean;
  readonly heading: string;
}
