// Module: web/apps/signal-web/src/app/shell/ide-shell.model.ts — IDE Shell View-Model & Live Control-Plane DTOs
// Invariants: Preserves string-serialized 64-bit fencing epochs; zero Angular imports.
// Spec: architecture/06-frontend-angular-and-fastapi-control-plane.md, architecture/20-versioning-release-monorepo-build-and-k8s-topology.md
import type { SigThemeId, SigUiProfile } from "@signal/design-system";

export interface WorkbenchNavLink {
  readonly slug: string;
  readonly label: string;
  readonly minProfile: SigUiProfile;
  readonly routePath: string;
}

export interface PlatformReleaseDto {
  readonly version: string;
  readonly channel: string;
  readonly released_at_ms: string;
  readonly physics_determinism_epoch: number;
  readonly wire_min_version: string;
  readonly highlights: readonly string[];
}

export interface SessionLeaseDto {
  readonly cell_id: string;
  readonly runtime_tier: string;
  readonly active_fencing_epoch: string;
  readonly engine_version: string;
  readonly negotiated_codec: string;
}

export interface IdeShellVm {
  readonly orgSlug: string;
  readonly projectSlug: string;
  readonly activeProfile: SigUiProfile;
  readonly activeTheme: SigThemeId;
  readonly visibleNavLinks: readonly WorkbenchNavLink[];
  readonly sessionBadge: string;
  readonly releaseBadge: string;
  readonly latestHighlights: readonly string[];
}
