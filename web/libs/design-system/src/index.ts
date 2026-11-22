// Module: web/libs/design-system/src/index.ts — Centralized @signal/design-system Public Tokens & Theme Models
// Invariants: Single source of truth for themes (dark, high-contrast, okabe-ito F-070) and Progressive UI Profiles (14-*).
// Spec: docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
export type SigThemeId = "dark" | "high-contrast" | "okabe-ito";
export type SigUiProfile = "EXPLORER" | "MAKER" | "PRO";

export interface SigThemeOption {
  readonly id: SigThemeId;
  readonly label: string;
}

export const SIG_THEMES: readonly SigThemeOption[] = [
  { id: "dark", label: "Signal Obsidian" },
  { id: "high-contrast", label: "High Contrast" },
  { id: "okabe-ito", label: "Okabe-Ito CVD Safe" },
] as const;

export const SIG_PROFILES: readonly SigUiProfile[] = ["EXPLORER", "MAKER", "PRO"] as const;
