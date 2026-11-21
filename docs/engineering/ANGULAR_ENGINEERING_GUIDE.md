# Signal — Angular 19/20+ Frontend & Zero-Leak Engineering Guide (`web/`)

> **Applies To:** `web/apps/signal-web/` and all lazy workbench libraries under `web/libs/*`.  
> **Governing Specs:** [`architecture/06-frontend-angular-and-nodejs-bff.md`](../../architecture/06-frontend-angular-and-nodejs-bff.md), [`architecture/15-application-security-and-project-isolation.md`](../../architecture/15-application-security-and-project-isolation.md), [`architecture/17-extended-feature-specifications-ui-and-rust.md`](../../architecture/17-extended-feature-specifications-ui-and-rust.md).

---

## 1. Modern Angular Core (Zoneless + Signals-First, Zero Legacy Patterns)

Signal is a greenfield Angular 19/20+ application. Legacy NgModule / Zone.js / decorator boilerplate is **strictly prohibited**:

| Concern | ALWAYS USE (Signal Standard) | NEVER USE (Rejected in Review) |
|---|---|---|
| **Component Architecture** | Standalone Components (`standalone: true` default in v19+), `ChangeDetectionStrategy.OnPush`, **Zoneless** change detection | `NgModule`, `Zone.js` dirty-check loops, default change detection |
| **Inputs / Outputs / Queries** | Signal APIs: `input()`, `input.required()`, `output()`, `model()`, `viewChild()`, `contentChild()` | Legacy `@Input()`, `@Output()`, `@ViewChild()`, `EventEmitter` |
| **Dependency Injection** | Function-based `inject(Service)` (`readonly` class fields) | Verbose constructor parameter lists |
| **Template Control Flow** | Built-in `@if`, `@else`, `@for (...; track item.id)`, `@switch`, `@defer (on viewport/interaction)` | `*ngIf`, `*ngFor`, `*ngSwitch`, missing `track` keys |
| **Reactive State** | Angular Signals (`signal`, `computed`, `linkedSignal`, `effect`), RxJS ONLY at async transport/stream boundaries (`toSignal` / `takeUntilDestroyed`) | Manual `BehaviorSubject` sprawl, unsubscribed `.subscribe()`, mutating arrays/objects in place |
| **Styling & Design Tokens** | Scoped component styles (`styles`/`styleUrl`) using CSS custom properties (`--sig-*`) + Okabe-Ito color-blind-safe & stipple tokens | Hardcoded hex colors, `::ng-deep`, global CSS leakage |

---

## 2. Clean Library Structure & Strict Dependency Boundaries (`web/libs/*`)

Every workbench library under `web/libs/<domain>/` follows one identical, human-scannable layout:

```text
web/libs/<domain>/
├── src/
│   ├── index.ts                  # SOLE public API barrel (routes + exported facades/components)
│   ├── <domain>.routes.ts        # Lazy route tree loaded via loadChildren()
│   ├── feature/                  # Smart container components (wire stores <-> presentational UI)
│   ├── ui/                       # Pure presentational components (input()/output() ONLY, zero injected services!)
│   ├── data-access/              # Signal stores (*.store.ts) + typed build-time contract clients (*.client.ts)
│   ├── overlays/                 # Zero-Leak HudOverlayPrimitive directives & canvas HUD widgets
│   └── models/                   # View-models using ES2020 bigint for all 64-bit contract fields
└── project.json / package.json
```

**Hard Boundary Rules (Enforced by ESLint `boundaries` / Nx module tags):**
1. Cross-library imports MUST target `@signal/<lib>` (`src/index.ts`) — **never** reach into another library's `feature/`, `ui/`, or `data-access/`.
2. `ui/` dumb components MUST stay pure (`input()` + `output()` only, `<= 200 LOC`) — trivial to read, reuse, and test!
3. Workbench libraries NEVER import another workbench's internal store directly; cross-pane coordination (e.g., clicking a Schematic net highlights PCB + Waveform + Firmware) flows through `@signal/workspace-bus` typed Signals!
4. Initial shell bundle (`apps/signal-web`) MUST stay **`< 250 KB` gzipped**; all 19 core + extended workbenches load strictly via lazy `loadChildren` + `@defer`.

---

## 3. 64-Bit `bigint` Precision, SI Units & Zero Physics in the Browser

1. **Zero `2^53` Precision Corruption:** Every 64-bit integer contract field (`_nm`, `_ps`, `_fs`, `fencing_epoch`, `provider_repo_id`) is generated as **ES2020 `bigint`** (`longAsBigint: true`). **NEVER** cast 64-bit coordinates or epochs via `Number(x)` — ESLint forbids `Number()` on branded `BigInt` SI types (`NanometersBigInt`, `PicosecondsBigInt`).
2. **Unambiguous SI Unit Pills:** Every numeric engineering input parses SI prefixes deterministically and displays an explicit unit confirmation pill distinguishing `m` (`milli`, `1e-3`) from `Meg` (`mega`, `1e6`) before dispatching mutations to Rust.
3. **Zero Physics in TypeScript:** Angular NEVER calculates SPICE matrices, trace impedance, creepage, DRC/LVS, or Gerber/GDSII data. All authoritative math runs in the Rust Project Cell; Angular renders 120 FPS client-side predictive guides (`HudOverlayPrimitive`) and streams `16 ms`-coalesced intent commands over WebTransport.

---

## 4. Zero-Leak DLP, Dual-Plane Viewport & Security Guardrails

1. **Zero Browser Disk Persistence of Design IP:**
   - **FORBIDDEN** for any design/schematic/PCB/IC/waveform/spec/code/AI data: `localStorage`, `sessionStorage`, `IndexedDB`, Cache API, Service Workers, `URL.createObjectURL` downloads, `window.print()`.
   - Only non-sensitive UI preferences (theme, dock layout, keymap) may persist locally.
   - Reconnect buffering (`<= 60 s`) lives strictly inside a **volatile in-memory JS ring-buffer closure**.
2. **Enterprise Zero-Leak `HudOverlayPrimitive` Discipline:**
   - Under Mode A (Zero-Leak), `signal-render` draws all design geometry into the `Zstd-QOI`/WebP GPU Atlas + hardware video plane; Angular renders only lightweight HUD overlays projected via `bigint` camera transforms—never buffering raw netlists or polygons in browser JS memory.
   - Multi-Monitor pop-outs (`FEAT-092`) sync via same-origin volatile `BroadcastChannel` + `SharedWorker`, inheriting identical focus-loss privacy curtains, print-block CSS, and Tardos + Constellation watermark streams.
3. **XSS, Trusted Types & Route-Partitioned Isolation:**
   - Strictly forbid `bypassSecurityTrustHtml`, `bypassSecurityTrustScript`, `innerHTML`, `eval`, and `new Function`.
   - Workspace routes (`/org/:org_slug/project/:proj_slug/*`) run under `COOP: same-origin` + `COEP: require-corp` (`crossOriginIsolated === true`); auth flows use top-level redirects to `/auth/*` (`SameSite=Lax -> __Host-signal_session SameSite=Strict`).
4. **Performance & Accessibility:**
   - Use `cdk-virtual-scroll` for every list/tree/spreadsheet `> 50` rows (BOM, FMEDA, Constraints, Netlist); cap waveform arrays to `<= 4,096` decimated min/max envelopes per pane.
   - Comply with WCAG 2.2 AA: keyboard-only wiring/navigation, ARIA live announcements for ERC/connectivity, Okabe-Ito color-blind-safe palettes + procedural stipple patterns.

---

## 5. Mandatory Angular / Frontend Verification Gate

```bash
pnpm --filter @signal/web lint
pnpm --filter @signal/web typecheck
pnpm --filter @signal/web test --run
pnpm audit --ignore-scripts
```
*(Verify zero `Number()` casts on `bigint` contract fields, zero forbidden storage/export/trust-bypass APIs, and bundle budget pass before opening a PR.)*
