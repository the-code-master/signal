---
name: signal-angular-workbench
description: Scaffolds and builds clean, Zoneless, Signals-first Angular 19/20+ standalone components, signal stores, and lazy-loaded workbench libraries under web/libs/* and web/apps/signal-web/. Use whenever implementing UI workbenches, panels, spreadsheets, HUDs, or forms in Signal's frontend.
---

# Signal Angular 19/20+ Workbench Builder (`web/**`)

References: `docs/engineering/ANGULAR_ENGINEERING_GUIDE.md`, `docs/engineering/ANGULAR_UI_STYLE_GUIDE.md` | Specs: `architecture/06-*`, `17-*`, `18-*`, `19-*`, `20-*`.

## Workbench & Component File Structure (`web/libs/<domain>/`)
```text
web/libs/<domain>/src/
├── index.ts                  # Sole public API barrel (lazy routes + exported facades only)
├── <domain>.routes.ts        # Lazy route tree registered in apps/signal-web
├── feature/<name>/           # Smart container folder:
│   ├── <name>.component.ts   # Thin Angular glue ONLY (<= 120 LOC, templateUrl, NO styleUrl/inline styles)
│   ├── <name>.component.html # Pure flat declarative markup (<= 150 LOC, bound ONLY to vm() + @signal/ui-kit)
│   ├── <name>.logic.ts       # Pure framework-agnostic TS logic/branching/formatting (<= 250 LOC)
│   └── <name>.model.ts       # readonly view-models & bigint SI DTOs
├── ui/<name>/                # Pure presentational component (same .ts/.html/.logic.ts/.model.ts split, input/output ONLY)
├── data-access/              # Signal stores (*.store.ts) + build-time contract clients (*.client.ts)
└── overlays/                 # Zero-Leak HudOverlayPrimitive directives
```

## Non-Negotiable Angular 19/20+ & UI Style Standards
1. **Zoneless, Standalone & Strict File Split:** Use standalone components, `OnPush`, `templateUrl` ONLY (NEVER inline `template:` or `styles:`), `signal()`, `computed()`, `linkedSignal()`, `effect()`, `input()`, `output()`, `viewChild()`, and `inject()`. NEVER use `NgModule`, `@Input()`, `@Output()`, `*ngIf`, or `*ngFor`.
2. **Zero-Spaghetti HTML Templates:** Put ALL branching, status-to-variant maps, role/profile filters, and config arrays in `<name>.logic.ts`. Keep `.component.html` `<= 150 LOC` and nesting depth `<= 2`, forbid compound boolean expressions in HTML, NEVER copy-paste sections across `@if`/`@else` (use `@for` over config arrays), and extract any branch `> 15` lines into a `ui/` or `@signal/ui-kit` component!
3. **Centralized Platform Styles ONLY (`@signal/design-system` + `@signal/ui-kit`):** NEVER create per-component `.css`/`.scss` files or HTML `style="..."`! Compose from `@signal/ui-kit` (`sig-button`, `sig-card`, `sig-panel`, `sig-si-unit-input`, `sig-data-grid`, `sig-status-pill`, `sig-viewport-frame`, etc.) and `.sig-*` platform classes. Rare special-geometry exceptions require `/* @sig-style-exception: <reason> */`, `<= 60 LOC`, 100% `var(--sig-*)` tokens, zero `::ng-deep`, zero `!important`.
4. **Strict Library Encapsulation:** Import other libs ONLY via `@signal/<lib>` (`src/index.ts`). Cross-pane selection uses `@signal/workspace-bus` typed signals. Keep initial shell bundle `< 250 KB` gzipped.
5. **64-Bit `bigint` & SI Safety:** Keep every `_nm`, `_ps`, `_fs`, `fencing_epoch`, and `provider_repo_id` value as `bigint` — NEVER cast via `Number(x)`. Use `sig-si-unit-input` / `.sig-si-pill` (`m` vs `Meg`) and `.sig-mono` tabular numerics.
6. **High-Density Performance & Accessibility:** Wrap any BOM, FMEDA, Constraint, or Netlist view `> 50` rows in `sig-data-grid` (`cdk-virtual-scroll`); cap waveforms to `<= 4,096` decimated envelopes; honor Okabe-Ito color-blind-safe tokens + stipple patterns and WCAG 2.2 AA keyboard CAD.
7. **Zero Legacy Security Anti-Patterns:** Never call `bypassSecurityTrust*`, `innerHTML`, `eval`, or write design data to browser storage (`localStorage`/`sessionStorage`/`IndexedDB`).

## Verification Commands
```bash
pnpm --filter @signal/web lint
pnpm --filter @signal/web typecheck
pnpm --filter @signal/web test --run
```

## Dev Response & Anti-Hallucination
- Verify Angular version (`package.json`), component symbols, and contract `bigint` types via `rg`/`view_file` before editing; never claim pass without observed `pnpm` exit code `0`.
- Reply in **2–3 sentences max** stating ONLY the issue, exact fix applied, and verification result.
