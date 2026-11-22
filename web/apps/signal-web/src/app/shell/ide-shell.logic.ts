// Module: web/apps/signal-web/src/app/shell/ide-shell.logic.ts — Pure Profile Filtering & Shell View-Model Logic
// Invariants: Filters EXPLORER/MAKER/PRO workbenches in TypeScript (never HTML @if chains); <=250 LOC.
// Spec: architecture/14-universal-scale-accessibility-and-expandability.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import type { SigThemeId, SigUiProfile } from "@signal/design-system";
import type { IdeShellVm, PlatformReleaseDto, SessionLeaseDto, WorkbenchNavLink } from "./ide-shell.model";

const PROFILE_RANK: Readonly<Record<SigUiProfile, number>> = {
  EXPLORER: 1,
  MAKER: 2,
  PRO: 3,
};

const NAV_CATALOG: readonly Omit<WorkbenchNavLink, "routePath">[] = [
  { slug: "schematic", label: "01 • Schematic Capture", minProfile: "EXPLORER" },
  { slug: "sim-viewer", label: "02 • SPICE & Adjoint Lab", minProfile: "EXPLORER" },
  { slug: "pcb", label: "03 • 224G / HV PCB Layout", minProfile: "EXPLORER" },
  { slug: "library-studio", label: "04 • Universal Parts (18-*)", minProfile: "EXPLORER" },
  { slug: "ai-copilot", label: "05 • AI Copilot & RAG", minProfile: "EXPLORER" },
  { slug: "academy", label: "06 • Academy & Quick Calc", minProfile: "EXPLORER" },
  { slug: "mcad-studio", label: "07 • 3D Parametric MCAD", minProfile: "MAKER" },
  { slug: "cam-print", label: "08 • CNC CAM & 3D Print", minProfile: "MAKER" },
  { slug: "plm-ops", label: "09 • PLM: BOM / ECN / CTB", minProfile: "MAKER" },
  { slug: "collab-hub", label: "10 • 5-Team Collab & Reviews", minProfile: "MAKER" },
  { slug: "code-ide", label: "11 • Firmware / BIOS Co-Sim", minProfile: "MAKER" },
  { slug: "dfm-panel", label: "12 • Signed Fab DFM", minProfile: "MAKER" },
  { slug: "magnetics", label: "13 • Magnetics Studio", minProfile: "PRO" },
  { slug: "power-studio", label: "14 • Power Integrity PDN", minProfile: "PRO" },
  { slug: "emc-lab", label: "15 • Pre-Compliance EMC", minProfile: "PRO" },
  { slug: "safety-wcca", label: "16 • ISO 26262 FMEDA/WCCA", minProfile: "PRO" },
  { slug: "ic", label: "17 • Analog/Digital IC & UPF", minProfile: "PRO" },
  { slug: "fpga-studio", label: "18 • FPGA RTL & Pinout", minProfile: "PRO" },
  { slug: "bringup-lab", label: "19 • Lab Bench Digital Twin", minProfile: "PRO" },
  { slug: "harness-3d", label: "20 • 3D Wire Harness", minProfile: "PRO" },
  { slug: "specs", label: "21 • Executable PRD Specs", minProfile: "PRO" },
  { slug: "visual-diff", label: "22 • Topological Visual Diff", minProfile: "PRO" },
  { slug: "review", label: "23 • Access Review Console", minProfile: "PRO" },
  { slug: "sftp-release", label: "24 • 2FA SFTP Factory Egress", minProfile: "PRO" },
  { slug: "admin", label: "25 • BYO-Git, PQC & K8s Admin", minProfile: "PRO" },
];

export function filterNavLinksForProfile(
  orgSlug: string,
  projectSlug: string,
  profile: SigUiProfile,
): readonly WorkbenchNavLink[] {
  const rank = PROFILE_RANK[profile];
  return NAV_CATALOG.filter((item) => PROFILE_RANK[item.minProfile] <= rank).map((item) => ({
    ...item,
    routePath: `/org/${orgSlug}/project/${projectSlug}/${item.slug}`,
  }));
}

export function buildIdeShellVm(
  orgSlug: string,
  projectSlug: string,
  profile: SigUiProfile,
  theme: SigThemeId,
  releases: readonly PlatformReleaseDto[],
  lease: SessionLeaseDto | null,
): IdeShellVm {
  const latest = releases[0];
  const sessionBadge = lease
    ? `Cell: ${lease.cell_id} • Epoch #${lease.active_fencing_epoch}`
    : "Cell: Connecting Tier-3 Rust Security Cell…";
  const releaseBadge = latest
    ? `v${latest.version} (${latest.channel} • Epoch ${latest.physics_determinism_epoch})`
    : "v0.1.0 (STABLE)";
  return {
    orgSlug,
    projectSlug,
    activeProfile: profile,
    activeTheme: theme,
    visibleNavLinks: filterNavLinksForProfile(orgSlug, projectSlug, profile),
    sessionBadge,
    releaseBadge,
    latestHighlights: latest?.highlights ?? [
      "3-Tier Isolated Architecture: Angular v22.3 + FastAPI 3.15 + Pure-Rust 1.100 Security Cell.",
    ],
  };
}
