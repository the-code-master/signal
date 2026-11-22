// Module: web/libs/ui-kit/src/status-pill/sig-status-pill.logic.ts — Pure Tone-to-Class Mapper
// Invariants: Resolves CSS class in TypeScript so HTML never contains ternary/boolean class soup.
// Spec: docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import type { SigStatusPillVm, SigStatusTone } from "./sig-status-pill.model";

const TONE_CLASS_MAP: Readonly<Record<SigStatusTone, string>> = {
  ok: "sig-badge sig-badge-ok",
  warn: "sig-badge sig-badge-warn",
  danger: "sig-badge sig-badge-danger",
  neutral: "sig-badge",
};

export function buildSigStatusPillVm(label: string, tone: SigStatusTone): SigStatusPillVm {
  return {
    label: label.trim(),
    cssClass: TONE_CLASS_MAP[tone],
  };
}
