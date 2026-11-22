// Module: web/libs/ui-kit/src/si-input/sig-si-input.model.ts — SI Unit Input View-Model & Parse Contract
// Invariants: Preserves 64-bit bigint nano-scaled values; disambiguates m (milli) vs M/Meg (Mega) (F-075, F-144).
// Spec: docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
export interface SigSiParseResult {
  readonly valid: boolean;
  readonly nanoUnits: bigint;
  readonly resolvedHint: string;
}
