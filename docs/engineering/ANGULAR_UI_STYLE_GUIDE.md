# Signal — Angular UI Style Guide & Standardized Component Ecosystem (`web/`)

> **Applies To:** Every component, template, stylesheet, and workbench under `web/apps/signal-web/` and `web/libs/*`.  
> **Companion Guides:** [`ANGULAR_ENGINEERING_GUIDE.md`](./ANGULAR_ENGINEERING_GUIDE.md) | [`TOKEN_EFFICIENCY_AND_CLEAN_CODE.md`](./TOKEN_EFFICIENCY_AND_CLEAN_CODE.md) | [`architecture/06-frontend-angular-and-nodejs-bff.md`](../../architecture/06-frontend-angular-and-nodejs-bff.md).  
> **Core Principle:** **Separation of Concerns + Zero-Spaghetti Templates + One Platform-Wide Style Ecosystem.** Every Angular component MUST keep HTML markup, TypeScript wiring, pure presentation logic, and models in dedicated files, eliminate `@if` duplication in templates, and consume Signal's centralized `@signal/design-system` + `@signal/ui-kit` styles with **zero per-component stylesheets** unless explicitly exempted.

---

## 1. Mandatory Multi-File Component Anatomy (Strict Separation of Concerns)

**HARD RULE:** Inline `template: \`...\`` and inline `styles: [...]` are **STRICTLY FORBIDDEN** across `web/`. Every Angular component lives in its own folder with cleanly separated files:

```text
web/libs/<domain>/src/feature/<component-name>/
├── <component-name>.component.ts      # Thin Angular glue ONLY (<= 120 LOC): inputs, outputs, inject(), vm signal
├── <component-name>.component.html    # Pure flat declarative markup ONLY (<= 150 LOC): zero logic, zero duplicated blocks
├── <component-name>.logic.ts          # Pure TypeScript UI logic (<= 250 LOC): branching, formatting, permissions, config arrays
├── <component-name>.model.ts          # Strongly-typed view-models, config types & bigint SI DTO interfaces
├── <component-name>.component.spec.ts # Template rendering & user-event tests
└── <component-name>.logic.spec.ts     # Ultra-fast pure-TS unit tests (zero Angular TestBed overhead!)
# NO <component-name>.component.css / .scss BY DEFAULT! (See Section 3.4)
```

| File | Owns (ALWAYS) | Forbidden (NEVER) | Target Size |
|---|---|---|---|
| **`.component.ts`** | `selector`, `templateUrl`, `changeDetection: OnPush`, `input()`/`output()`, `inject()`, `vm = computed(() => buildViewModel(...))`, thin event forwarders | Inline `template:`, inline `styles:`, `styleUrl` (without exception), nested `if/else` trees, raw formatting/mapping logic | `40–120 LOC` |
| **`.component.html`** | Clean semantic HTML consuming `@signal/ui-kit` components and `.sig-*` platform classes, bound ONLY to pre-computed `vm()` properties | Compound boolean expressions (`&&`/`\|\|`/`?:`), nested `@if` depth `> 2`, copy-pasted `@if`/`@else` HTML sections, inline `style="..."` | `30–120 LOC` (`<= 150` hard cap) |
| **`.logic.ts`** | Pure functions (`buildViewModel`, `resolveStatusBadge`, `filterVisibleActions`, `formatSiBigInt`, `validateFormState`), static config lookup maps | Angular decorators, DOM/Browser APIs, `inject()`, mutable shared globals, side effects | `40–220 LOC` |
| **`.model.ts`** | `readonly` interfaces, discriminated unions, `bigint` SI types, action/column/tab descriptors | Executable logic, classes with behavior, `any` | `20–120 LOC` |

---

## 2. Anti-Spaghetti HTML Laws (Zero `@if` Soup & Zero Copy-Pasted Sections)

Unmaintainable Angular templates happen when engineers pile `@if` chains into HTML and copy-paste entire sections across `@if` / `@else` branches. Signal forbids this structurally via **5 Hard Template Laws**:

1. **Law 1 — Pre-Compute Every Condition in `.logic.ts`:**
   - **NEVER** put multi-condition boolean expressions, role comparisons, or string checks inside `.component.html`.
   - Derive named view-model flags and descriptors in `.logic.ts` (`vm().canApprove`, `vm().statusBadge`, `vm().toolbarActions`).
2. **Law 2 — Maximum Control-Flow Nesting Depth `<= 2`:**
   - Never nest `@if` inside `@if` inside `@if` (or `@if` inside `@for` inside `@if`). Extract inner blocks into a focused child `ui/` component or flatten state into a single `@switch (vm().panelState)` (`'loading' | 'empty' | 'error' | 'ready'`).
3. **Law 3 — ZERO Copy-Pasted HTML Sections Across `@if` / `@else` Branches:**
   - Never duplicate a card, header, toolbar, form group, or table across `@if (isReadOnly)` vs `@else`!
   - Render the structure **once** and drive differences from `.logic.ts` via `[disabled]="vm().isReadOnly"`, `[variant]="vm().primaryAction.variant"`, or a `@for (field of vm().fields; track field.id)` config array!
4. **Law 4 — Extract Any Branch `> 15` Lines of HTML Into a Sub-Component:**
   - Whenever a conditional branch or tab body exceeds `15` lines of HTML, extract it into a pure presentational component under `ui/` (or `@signal/ui-kit`) taking explicit `input()` signals.
5. **Law 5 — Zero Method Calls That Recompute Work in Templates:**
   - Bind strictly to memoized Angular `computed()` signals (`vm().rows`), never calling non-signal helper methods directly inside `@for` or template interpolations.

### Side-by-Side Example: Spaghetti HTML (FORBIDDEN) vs. Signal Clean Architecture (REQUIRED)

**FORBIDDEN — Logic & Copy-Paste Inside `.component.html`:**
```html
<!-- REJECTED IN CODE REVIEW: inline logic, duplicated card markup, ad-hoc inline styles -->
@if (user().role === 'ADMIN' && !ecn().locked && ecn().status === 'REVIEW') {
  <div style="padding: 16px; background: #1e293b; border: 1px solid #38bdf8;">
    <h3>{{ ecn().title }}</h3>
    <span style="color: #f59e0b">Pending Approval</span>
    <button (click)="approve()">Approve</button>
  </div>
} @else if (ecn().status === 'RELEASED') {
  <div style="padding: 16px; background: #1e293b; border: 1px solid #22c55e;">
    <h3>{{ ecn().title }}</h3>
    <span style="color: #22c55e">Released</span>
  </div>
}
```

**REQUIRED — Separate `.logic.ts` + `.component.ts` + Clean `.component.html` Using Platform Styles:**

```ts
// ecn-summary-card.logic.ts (Pure, zero-TestBed unit-testable presentation logic)
export function buildEcnSummaryVm(ecn: EcnRecord, role: SignalRole): EcnSummaryVm {
  const statusMap: Record<EcnStatus, StatusBadgeConfig> = {
    DRAFT:    { label: 'Draft',            tone: 'neutral' },
    REVIEW:   { label: 'Pending Approval', tone: 'warn' },
    RELEASED: { label: 'Released',         tone: 'ok' },
  };
  const actions: readonly ActionButtonConfig[] =
    role === 'PROJECT_ADMIN' && !ecn.locked && ecn.status === 'REVIEW'
      ? [{ id: 'approve', label: 'Approve (2FA)', variant: 'primary' }]
      : [];

  return { title: ecn.title, code: ecn.code, badge: statusMap[ecn.status], actions };
}
```

```ts
// ecn-summary-card.component.ts (Thin Angular wiring — ZERO styleUrl / ZERO inline template!)
@Component({
  selector: 'sig-ecn-summary-card',
  standalone: true,
  imports: [SigCardComponent, SigStatusPillComponent, SigButtonComponent],
  templateUrl: './ecn-summary-card.component.html',
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class EcnSummaryCardComponent {
  readonly ecn = input.required<EcnRecord>();
  readonly role = input.required<SignalRole>();
  readonly actionTriggered = output<EcnActionId>();
  protected readonly vm = computed(() => buildEcnSummaryVm(this.ecn(), this.role()));
}
```

```html
<!-- ecn-summary-card.component.html (Single flat template, 100% shared ui-kit + .sig-* styles!) -->
<sig-card [title]="vm().title" [subtitle]="vm().code">
  <div class="sig-row sig-row--between sig-gap-3">
    <sig-status-pill [label]="vm().badge.label" [tone]="vm().badge.tone" />
    <div class="sig-row sig-gap-2">
      @for (action of vm().actions; track action.id) {
        <sig-button [variant]="action.variant" (pressed)="actionTriggered.emit(action.id)">
          {{ action.label }}
        </sig-button>
      }
    </div>
  </div>
</sig-card>
```

---

## 3. Centralized Platform Style & HTML Component Ecosystem (Zero Component-Local Styles!)

**HARD RULE:** Feature & workbench components (`web/apps/signal-web/` and `web/libs/<domain>/`) **MUST NOT** create `.component.css` / `.component.scss` files or write inline `styles: [...]`! All visual styling lives in **two shared platform libraries**:

```text
web/libs/
├── design-system/                 # SOLE home of CSS design tokens, themes & platform .sig-* style primitives
│   └── src/styles/
│       ├── index.css              # Single entry imported once by apps/signal-web/src/styles.css
│       ├── tokens/
│       │   ├── colors.css         # Semantic dark/light/high-contrast + Okabe-Ito color-blind-safe CAD tokens
│       │   ├── typography.css     # UI sans + tabular-nums engineering monospace + strict type scale
│       │   ├── spacing.css        # Strict 4px grid (--sig-space-1..12), radii, shadows, z-index layers
│       │   └── motion.css         # Standardized durations/easings (respecting prefers-reduced-motion)
│       ├── primitives/
│       │   ├── layout.css         # .sig-shell, .sig-dock, .sig-pane, .sig-split-{h,v}, .sig-row, .sig-stack, .sig-grid
│       │   ├── surfaces.css       # .sig-panel, .sig-card, .sig-drawer, .sig-modal, .sig-toolbar, .sig-statusbar
│       │   ├── typography.css     # .sig-heading-{1..4}, .sig-label, .sig-muted, .sig-mono, .sig-si-pill
│       │   ├── controls.css       # .sig-btn(--primary/secondary/ghost/danger/sm/md), .sig-input, .sig-select, .sig-tabs
│       │   ├── data-display.css   # .sig-table, .sig-tree, .sig-kv-list, .sig-badge--*, .sig-empty-state, .sig-skeleton
│       │   └── viewport-dlp.css   # .sig-hud-layer, .sig-hud-chip, .sig-focus-curtain, .sig-watermark-plane, .sig-no-print
│       └── themes/
│           ├── dark-workbench.css # Default precision dark CAD workbench theme
│           ├── light-studio.css   # High-legibility daylight engineering theme
│           └── high-contrast.css  # WCAG 2.2 AAA + Okabe-Ito stipple/hatch accessibility theme
└── ui-kit/                        # Complete reusable HTML Component Ecosystem consumed by ALL 23 workbenches
    └── src/components/            # Each component uses separate .ts + .html + .logic.ts + .model.ts + platform CSS!
```

### 3.1 Standardized Platform CSS Primitives (`.sig-*` Namespace)

| Category | Canonical Classes (Use Across Entire Platform) | Design Guarantee |
|---|---|---|
| **Layout & Spacing** | `.sig-shell`, `.sig-dock`, `.sig-pane`, `.sig-split-h`, `.sig-split-v`, `.sig-row`, `.sig-row--between`, `.sig-stack`, `.sig-grid-2`, `.sig-grid-3`, `.sig-gap-{1..6}`, `.sig-pad-{1..6}`, `.sig-scroll-y` | Strict `4px` spatial rhythm; zero magic margins or rogue flex wrappers. |
| **Surfaces & Chrome** | `.sig-panel`, `.sig-panel__header`, `.sig-panel__body`, `.sig-card`, `.sig-card--interactive`, `.sig-toolbar`, `.sig-statusbar`, `.sig-drawer`, `.sig-modal` | Identical elevation, border contrast, and header alignment across all 23 workbenches. |
| **Typography & Engineering Data** | `.sig-heading-{1..4}`, `.sig-label`, `.sig-muted`, `.sig-mono` (`tabular-nums`), `.sig-si-pill`, `.sig-truncate` | Decimal points, nanometer coordinates, BOM costs, and voltages align vertically in every grid/HUD! |
| **Semantic Status & Trust Badges** | `.sig-badge--{neutral,info,ok,warn,danger}`, `.sig-trust--{bronze,silver,gold}`, `.sig-diff--{added,removed,modified}` | WCAG 2.2 AA contrast + icon/text pairing (never color-only signaling!). |
| **Controls & Navigation** | `.sig-btn`, `.sig-btn--{primary,secondary,ghost,danger}`, `.sig-btn--{sm,md}`, `.sig-input`, `.sig-select`, `.sig-field`, `.sig-field--error`, `.sig-tabs`, `.sig-tab--active` | Consistent focus rings (`:focus-visible`), keyboard navigation, and hover/active states. |
| **CAD Viewport & Zero-Leak DLP** | `.sig-viewport-host`, `.sig-hud-layer`, `.sig-hud-chip`, `.sig-focus-curtain`, `.sig-watermark-plane`, `.sig-no-print` (`@media print { display: none !important; }`) | Unified Zero-Leak privacy curtain, print block, and `HudOverlayPrimitive` z-layering. |

### 3.2 Complete Reusable HTML Component Ecosystem (`@signal/ui-kit`)

Before writing ANY new UI markup in a workbench (`schematic`, `pcb`, `ic`, `mcad-studio`, `cam-print`, `plm-ops`, `collab-hub`, `library-studio`, `admin`, etc.), developers and AI agents MUST compose from `@signal/ui-kit`:

| Family | Standardized Components (`@signal/ui-kit`) | Purpose Across Signal |
|---|---|---|
| **Actions & Commands** | `sig-button`, `sig-icon-button`, `sig-split-button`, `sig-toolbar`, `sig-action-bar`, `sig-command-palette` | Uniform action bars, `Cmd+K` Omni-Palette, and 2FA-badged release buttons. |
| **Layout & Containers** | `sig-panel`, `sig-dock-pane`, `sig-card`, `sig-drawer`, `sig-modal`, `sig-tabs`, `sig-accordion`, `sig-split-view` | Consistent docking panels, drawers (including "What's New"), and modals. |
| **Forms & Engineering Inputs** | `sig-form-field`, `sig-input`, `sig-si-unit-input`, `sig-select`, `sig-search-bar`, `sig-toggle`, `sig-slider` | Built-in validation errors, `bigint` SI-prefix parsing (`m` vs `Meg` confirmation pill!). |
| **Data, Trees & Inspectors** | `sig-data-grid` (`cdk-virtual-scroll`), `sig-tree-view`, `sig-property-inspector`, `sig-kv-list`, `sig-diff-viewer` | High-density 100k-row BOM/FMEDA/Constraint/Netlist/Access-Review tables without DOM lag. |
| **Feedback & States** | `sig-status-pill`, `sig-trust-badge`, `sig-banner`, `sig-toast`, `sig-empty-state`, `sig-error-state`, `sig-skeleton` | Uniform loading, empty, error, and `BRONZE`/`SILVER`/`GOLD` part trust states. |
| **CAD / 3D / Zero-Leak** | `sig-viewport-frame`, `sig-hud-chip`, `sig-layer-swatch`, `sig-stepper-timeline` | Shared Dual-Plane viewport shell, Okabe-Ito + stipple layer swatches, and CCB/ECO approval trails. |

### 3.3 Strict Exception Protocol for Special Component-Local Styles

Creating a component-local stylesheet (`<name>.component.css`) outside `web/libs/design-system/` or `web/libs/ui-kit/` is **DISALLOWED by default** and permitted **ONLY** for genuinely specialized visual geometry (e.g., radial pie-menu trigonometry, custom 2D/3D canvas overlay crosshair alignment, or WebGPU/SVG schematic cursor reticles):

1. **Mandatory Exception Header (Line 1):**
   ```css
   /* @sig-style-exception: Radial 8-slice trigonometric HUD positioning (cannot be expressed via .sig-* primitives) */
   ```
2. **100% Design-Token Discipline:** Even inside an approved exception file, **ZERO hardcoded `#hex`/`rgb()` colors, pixel font sizes, or arbitrary spacing** are allowed—consume ONLY `var(--sig-*)` tokens!
3. **Hard Size & Encapsulation Limits:** Maximum **`60 LOC`**, **ZERO `::ng-deep`**, **ZERO `!important`**, and **ZERO overrides of `.sig-*` global classes** (if a shared pattern is missing, add it cleanly to `@signal/design-system` or `@signal/ui-kit` instead!).

---

## 4. Accessibility, Progressive Profiles & Zero-Leak Visual Standards

- **Color-Blind & High-Contrast Parity:** Every CAD/PCB/IC/MCAD layer, trace, and status badge pairs Okabe-Ito tokens with **procedural stipple/hatch patterns** and explicit text/icon labels (WCAG 2.2 AA `>= 4.5:1` text, `>= 3:1` UI chrome).
- **Tabular Engineering Numerics:** Every coordinate, net length, impedance, voltage, current, cost, and timestamp uses `.sig-mono` (`font-variant-numeric: tabular-nums`) + ES2020 `bigint` formatting from `.logic.ts`.
- **Progressive UI Profiles (`EXPLORER` / `MAKER` / `PRO` / `SILICON_SCALE`):** Filter visible panels, toolbar buttons, and inspector fields inside `.logic.ts` config builders—never via scattered `@if (profile() === ...)` checks in HTML!
- **Zero-Leak DLP Styling:** `.sig-no-print`, `.sig-focus-curtain`, and `.sig-watermark-plane` apply globally via `@signal/design-system` to both the main IDE window and Multi-Monitor pop-outs.

---

## 5. Automated CI / ESLint / Stylelint Enforcement Rules

Every rule in this Style Guide is mechanically enforced in `pnpm --filter @signal/web lint` so neither humans nor AI agents can introduce drift:

| Rule / Check | Automated Failure Trigger |
|---|---|
| **`no-inline-template` & `no-inline-styles`** | Fails if any `@Component` uses inline `template:` or `styles:` instead of `templateUrl` + centralized design system. |
| **`sig-no-local-stylesheet`** | Fails if any component outside `web/libs/{design-system,ui-kit}/` declares `styleUrl`/`styleUrls` without a documented `/* @sig-style-exception: ... */` header and `<= 60 LOC` cap. |
| **`stylelint-no-raw-colors-or-deep`** | Fails on hardcoded `#hex`/`rgb()`/`hsl()` colors outside `design-system/src/styles/tokens/`, `::ng-deep`, `!important` (except `.sig-no-print`), or inline `style="..."` in `.html`. |
| **`sig-html-complexity-gate`** | Fails if any `.component.html` exceeds `150 LOC`, nests `@if`/`@for`/`@switch` deeper than `2` levels, or contains compound boolean expressions (`&&`/`\|\|` with `> 1` operator) in template directives. |
| **`sig-clean-file-separation`** | Enforces `.component.ts <= 120 LOC` (target) / `<= 250 LOC` (hard cap), pure `ui/` dumb components (`input()`/`output()` only), and zero `NgModule`/`@Input`/`@Output`/`*ngIf`/`*ngFor`/`Number(bigint)`/`bypassSecurityTrust*`. |
