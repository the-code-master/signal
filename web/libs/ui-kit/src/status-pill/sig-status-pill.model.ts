// Module: web/libs/ui-kit/src/status-pill/sig-status-pill.model.ts — SigStatusPill View-Model Contract
// Invariants: Immutable readonly severity union; zero Angular imports.
// Spec: docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
export type SigStatusTone = "ok" | "warn" | "danger" | "neutral";

export interface SigStatusPillVm {
  readonly label: string;
  readonly cssClass: string;
}
