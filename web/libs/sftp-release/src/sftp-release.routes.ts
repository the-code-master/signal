// Module: web/libs/sftp-release/src/sftp-release.routes.ts — Lazy Angular v22+ Routes for Step-Up 2FA Write-Only SFTP Factory Egress
// Invariants: Zero eager bundle cost; feeds immutable WorkbenchDescriptor into centralized WorkbenchStageComponent.
// Spec: architecture/06-frontend-angular-and-fastapi-control-plane.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import type { Routes } from "@angular/router";
import { WorkbenchStageComponent, type WorkbenchDescriptor } from "@signal/canvas-core";

export const WORKBENCH_DESCRIPTOR: WorkbenchDescriptor = {
  slug: "sftp-release",
  title: "Step-Up 2FA Write-Only SFTP Factory Egress",
  subtitle: "WebAuthn/Burned-TOTP + 3x Quorum Tier-0 Audit + russh SFTP",
  rustCrate: "signal-sftp-egress",
  subprojectRepo: "aurora-ev-inverter-ops",
  kpis: [
    { id: "kpi-1", label: "Primary Gate", value: "Browser Downloads: ZERO", tone: "ok" },
    { id: "kpi-2", label: "Domain Telemetry", value: "Tier-0 Quorum: 3/3", tone: "neutral" },
    { id: "kpi-3", label: "Isolation Cell", value: "1 Project = 1 Cell", tone: "ok" },
  ],
  hudSignals: ["Watermark: Constellation", "Deviation Gate: PASS"],
  gridTitle: "Release Bundles Ready for SFTP",
  columns: [
    { key: "id", header: "ID", mono: true },
    { key: "item", header: "Engineering Artifact / Entity" },
    { key: "metric", header: "Verified SI / Status", mono: true },
  ],
  rows: [
    { id: "REL-1", item: "aurora_pcb_rev_a.odb.tgz", metric: "SHA-256 Pinned" },
    { id: "REL-2", item: "aurora_housing_rev_a.step", metric: "AP242 Signed" },
  ],
};

export const WORKBENCH_ROUTES: Routes = [
  {
    path: "",
    component: WorkbenchStageComponent,
    data: { workbench: WORKBENCH_DESCRIPTOR },
  },
];
