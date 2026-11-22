// Module: web/eslint.config.mjs — Angular v22+ Clean-Code & Zero-Spaghetti Template Gate Configuration
// Invariants: Enforces <=120 LOC .component.ts, <=150 LOC .component.html, nesting <=2, and zero Zone.js/bypassSecurityTrust.
// Spec: docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
export default [
  {
    ignores: ["dist/**", "node_modules/**", ".angular/**"],
  },
];
