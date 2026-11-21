---
name: signal-angular-workbench
description: Scaffolds and builds clean, Zoneless, Signals-first Angular 19/20+ standalone components, signal stores, and lazy-loaded workbench libraries under web/libs/* and web/apps/signal-web/. Use whenever implementing UI workbenches, panels, spreadsheets, HUDs, or forms in Signal's frontend.
---

# Signal Angular 19/20+ Workbench Builder (`web/**`)

Reference: `docs/engineering/ANGULAR_ENGINEERING_GUIDE.md` | Specs: `architecture/06-*`, `architecture/17-*`.

## Workbench Library Structure (`web/libs/<domain>/`)
```text
web/libs/<domain>/src/
├── index.ts                  # Sole public API barrel (lazy routes + exported facades only)
├── <domain>.routes.ts        # Lazy route tree registered in apps/signal-web
├── feature/                  # Smart containers wiring *.store.ts <-> ui/ (<= 250 LOC)
├── ui/                       # Pure presentational components (input()/output() ONLY, <= 200 LOC)
├── data-access/              # Signal stores (*.store.ts) + build-time contract clients (*.client.ts)
├── overlays/                 # Zero-Leak HudOverlayPrimitive directives
└── models/                   # Typed view-models using ES2020 bigint for 64-bit SI fields
```

## Non-Negotiable Angular 19/20+ Standards
1. **Zoneless & Standalone Only:** Use standalone components, `ChangeDetectionStrategy.OnPush`, `signal()`, `computed()`, `linkedSignal()`, `effect()`, `input()`, `input.required()`, `output()`, `model()`, `viewChild()`, and function-based `inject()`. NEVER use `NgModule`, `@Input()`, `@Output()`, `*ngIf`, or `*ngFor`.
2. **Modern Templates:** Use `@if`, `@else`, `@for (item of items(); track item.id)`, `@switch`, and `@defer (on viewport/interaction)`.
3. **Strict Library Encapsulation:** Import other libs ONLY via `@signal/<lib>` (`src/index.ts`). Cross-pane selection/cross-probing uses `@signal/workspace-bus` typed signals. Keep initial shell bundle `< 250 KB` gzipped.
4. **64-Bit `bigint` & SI Safety:** Keep every `_nm`, `_ps`, `_fs`, `fencing_epoch`, and `provider_repo_id` value as `bigint` — NEVER cast via `Number(x)`. Display explicit SI confirmation pills (`m` vs `Meg`) on engineering inputs.
5. **High-Density Performance:** Wrap any BOM, FMEDA, Constraint, or Netlist view `> 50` rows in `cdk-virtual-scroll`; cap rendered waveform arrays to `<= 4,096` decimated min/max envelopes.
6. **Accessibility & Progressive Profiles:** Support `EXPLORER` / `MAKER` / `PRO` / `SILICON_SCALE` visibility gates, Okabe-Ito color-blind-safe CSS tokens + stipple patterns, keyboard-only CAD navigation, and WCAG 2.2 AA ARIA live regions.
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
