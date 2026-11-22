// Module: web/apps/signal-web/src/app/shell/ide-shell.component.ts — Signal Angular v22+ Zoneless IDE Shell
// Invariants: <=120 LOC; Zoneless OnPush; templateUrl ONLY; integrates CU-P1-05 Power UX & WCAG CAD cleanly.
// Spec: architecture/06-frontend-angular-and-fastapi-control-plane.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { DOCUMENT } from "@angular/common";
import { HttpClient, HttpHeaders } from "@angular/common/http";
import { ChangeDetectionStrategy, Component, computed, inject, type OnInit, signal } from "@angular/core";
import { Router, RouterLink, RouterLinkActive, RouterOutlet } from "@angular/router";
import { SIG_PROFILES, SIG_THEMES, type SigThemeId, type SigUiProfile } from "@signal/design-system";
import { SigDrawerComponent, SigPanelComponent, SigStatusPillComponent } from "@signal/ui-kit";
import { AccessibilityCadComponent } from "../accessibility/accessibility.component";
import { MultiMonitorManagerComponent } from "../multi-monitor/multi-monitor.component";
import { OmniPaletteComponent } from "../omni-palette/omni-palette.component";
import { SelectionQueryToolbarComponent } from "../selection-query/selection-query.component";
import { SpacemouseControlComponent } from "../spacemouse/spacemouse.component";
import { buildIdeShellVm } from "./ide-shell.logic";
import type { CadKeymapPreset, PlatformReleaseDto, SessionLeaseDto } from "./ide-shell.model";

@Component({
  selector: "sig-ide-shell",
  standalone: true,
  imports: [
    RouterOutlet,
    RouterLink,
    RouterLinkActive,
    SigDrawerComponent,
    SigPanelComponent,
    SigStatusPillComponent,
    OmniPaletteComponent,
    SelectionQueryToolbarComponent,
    MultiMonitorManagerComponent,
    SpacemouseControlComponent,
    AccessibilityCadComponent,
  ],
  templateUrl: "./ide-shell.component.html",
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class IdeShellComponent implements OnInit {
  private readonly http = inject(HttpClient);
  private readonly doc = inject(DOCUMENT);
  private readonly router = inject(Router);

  readonly profiles = SIG_PROFILES;
  readonly themes = SIG_THEMES;
  readonly activeProfile = signal<SigUiProfile>("PRO");
  readonly activeTheme = signal<SigThemeId>("dark");
  readonly whatsNewOpen = signal<boolean>(false);
  readonly omniPaletteOpen = signal<boolean>(false);
  readonly queryBarOpen = signal<boolean>(false);
  readonly powerDrawerOpen = signal<boolean>(false);
  readonly secondaryPaneCount = signal<number>(0);
  readonly keymapPreset = signal<CadKeymapPreset>("KiCad");
  readonly lastCliSummary = signal<string>("Cmd+K Ready");
  readonly releases = signal<readonly PlatformReleaseDto[]>([]);
  readonly sessionLease = signal<SessionLeaseDto | null>(null);
  readonly reviewSummary = signal<string>("DVT Gate GREEN • 0 DRC • 98.5% CTB");

  readonly vm = computed(() =>
    buildIdeShellVm({
      orgSlug: "signal-design-inc",
      projectSlug: "aurora-ev-inverter",
      profile: this.activeProfile(),
      theme: this.activeTheme(),
      releases: this.releases(),
      lease: this.sessionLease(),
      secondaryPaneCount: this.secondaryPaneCount(),
      keymapPreset: this.keymapPreset(),
    }),
  );

  ngOnInit(): void {
    this.http.get<readonly PlatformReleaseDto[]>("/api/v1/platform/releases").subscribe({
      next: (items) => this.releases.set(items),
      error: () => undefined,
    });
    const headers = new HttpHeaders({ "Idempotency-Key": "00000000-0000-4000-8000-000000000001" });
    this.http
      .post<SessionLeaseDto>("/api/v1/orgs/signal-design-inc/projects/aurora-ev-inverter/sessions", {}, { headers })
      .subscribe({ next: (lease) => this.sessionLease.set(lease), error: () => undefined });
  }

  onShellKeydown(event: KeyboardEvent): void {
    if ((event.metaKey || event.ctrlKey) && event.key.toLowerCase() === "k") {
      event.preventDefault();
      this.omniPaletteOpen.update((v) => !v);
    } else if (event.key === "Escape" && this.omniPaletteOpen()) {
      this.omniPaletteOpen.set(false);
    }
  }

  selectProfile(profile: SigUiProfile): void {
    this.activeProfile.set(profile);
  }

  selectTheme(theme: SigThemeId): void {
    this.activeTheme.set(theme);
    this.doc.documentElement.setAttribute("data-sig-theme", theme);
  }

  navigateWorkbench(slug: string): void {
    void this.router.navigateByUrl(`/org/signal-design-inc/project/aurora-ev-inverter/${slug}`);
  }
}
