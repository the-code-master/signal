---
name: signal-angular-ui-style-system
description: Enforces Signal's Angular UI Style Guide, strict multi-file component separation (.component.ts, .component.html, .logic.ts, .model.ts), anti-spaghetti HTML template laws (zero @if chains or duplicated markup), and centralized @signal/design-system + @signal/ui-kit styling (forbidding per-component styles unless explicitly exempted). Use whenever creating, styling, refactoring, or reviewing any Angular component, template, or UI library under web/.
---

# Signal Angular v22+ ONLY UI Style Guide & Standardized Component Ecosystem (`web/**`)

Governing guide: `docs/engineering/ANGULAR_UI_STYLE_GUIDE.md` | Rules: `.cursor/rules/angular-frontend.mdc`.

## 1. Mandatory Multi-File Component Separation (Angular v22+ ONLY — Zero Inline Templates or Logic Bloat)
Every Angular `v22+` standalone component lives in its own folder with cleanly separated files:
- `<name>.component.ts` (`<= 120 LOC`): Thin Angular glue ONLY (`templateUrl`, `OnPush`, `input()`, `output()`, `inject()`, `vm = computed(() => buildVm(...))`, event forwarders). **NEVER** inline `template:` or `styles:`.
- `<name>.component.html` (`<= 150 LOC`): Pure flat declarative markup bound ONLY to pre-computed `vm()` properties and `@signal/ui-kit` components.
- `<name>.logic.ts` (`<= 250 LOC`): Pure framework-agnostic TypeScript functions owning ALL branching, status-to-badge maps, role/profile filtering, SI `bigint` formatting, and config arrays (unit-tested in `<name>.logic.spec.ts` without TestBed!).
- `<name>.model.ts`: `readonly` interfaces, discriminated unions, and `bigint` view-model contracts.

## 2. Anti-Spaghetti HTML Template Laws (Zero `@if` Soup & Zero Copy-Paste)
1. **No Complex Expressions in HTML:** Compute booleans/states in `.logic.ts` (`vm().canApprove`); never write multi-operator `&&`/`||`/role checks in templates.
2. **Max Nesting Depth `<= 2`:** Never nest `@if` inside `@if` inside `@if`. Flatten into `@switch (vm().panelState)` or extract a child component.
3. **Zero Copy-Pasted Sections Across `@if`/`@else`:** Render shared cards/forms/toolbars ONCE; vary behavior via `.logic.ts` config arrays (`@for (action of vm().actions; track action.id)`) or bound properties (`[disabled]="vm().isReadOnly"`).
4. **Extract Branches `> 15` HTML Lines:** Move any substantial branch/tab into a pure `ui/` or `@signal/ui-kit` component with explicit `input()` signals.
5. **No Profile/Role Branching in HTML:** Filter `EXPLORER`/`MAKER`/`PRO`/`SILICON_SCALE` visibility inside `.logic.ts` config builders.

## 3. Centralized Design System & Component Ecosystem (NO Per-Component Styles!)
- **Zero Component Stylesheets by Default:** NEVER add `.component.css`/`.component.scss`, inline `styles: [...]`, or HTML inline `style="..."` on feature/workbench components!
- **Consume Centralized Styles & UI Kit Exclusively:**
  - Tokens & global `.sig-*` CSS primitives live ONLY in `web/libs/design-system/src/styles/` (`tokens/`, `primitives/`, `themes/`).
  - Reusable HTML components (`sig-button`, `sig-card`, `sig-panel`, `sig-drawer`, `sig-modal`, `sig-tabs`, `sig-si-unit-input`, `sig-data-grid`, `sig-status-pill`, `sig-trust-badge`, `sig-empty-state`, `sig-viewport-frame`, etc.) live ONLY in `web/libs/ui-kit/`.
  - If a needed UI pattern or class is missing, add it cleanly to `@signal/design-system` or `@signal/ui-kit` so the entire platform reuses it!
- **Strict Special-Case Style Exception:** Permitted ONLY for specialized visual geometry (e.g., radial pie-menu trigonometry or canvas reticle positioning) with line-1 `/* @sig-style-exception: <reason> */`, `<= 60 LOC`, 100% `var(--sig-*)` tokens (zero raw hex/px), zero `::ng-deep`, and zero `!important`.

## Dev Response & Anti-Hallucination
- Verify existing `@signal/ui-kit` components and `.sig-*` classes in `web/libs/{ui-kit,design-system}` via `rg`/`view_file` before creating anything new; never claim lint/tests pass without observed exit code `0`.
- Reply in **2–3 sentences max** stating ONLY the issue, exact fix applied, and verification result.
