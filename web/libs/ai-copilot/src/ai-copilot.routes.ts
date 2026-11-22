// Module: web/libs/ai-copilot/src/ai-copilot.routes.ts — Lazy Angular v22+ Routes for Neuro-Symbolic AI Copilot & Quarantined RAG
// Invariants: Zero eager bundle cost; feeds immutable WorkbenchDescriptor into centralized WorkbenchStageComponent.
// Spec: architecture/06-frontend-angular-and-fastapi-control-plane.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import type { Routes } from "@angular/router";
import { WorkbenchStageComponent, type WorkbenchDescriptor } from "@signal/canvas-core";

export const WORKBENCH_DESCRIPTOR: WorkbenchDescriptor = {
  slug: "ai-copilot",
  title: "Neuro-Symbolic AI Copilot & Quarantined RAG",
  subtitle: "Zero-Tool VLM Datasheet Quarantine + GBNF + Rust ProofBundles",
  rustCrate: "signal-pod",
  subprojectRepo: "aurora-ev-inverter-meta",
  kpis: [
    { id: "kpi-1", label: "Primary Gate", value: "Quarantine: ZERO_TOOLS", tone: "ok" },
    { id: "kpi-2", label: "Domain Telemetry", value: "Proofs: Re-Verified", tone: "neutral" },
    { id: "kpi-3", label: "Isolation Cell", value: "1 Project = 1 Cell", tone: "ok" },
  ],
  hudSignals: ["LLM: Auto Ollama/Mock", "Citations: BBox-Pinned"],
  gridTitle: "Verified Agent Proposals",
  columns: [
    { key: "id", header: "ID", mono: true },
    { key: "item", header: "Engineering Artifact / Entity" },
    { key: "metric", header: "Verified SI / Status", mono: true },
  ],
  rows: [
    { id: "AG-1", item: "Optimize snubber C=2.2nF via Saltation gradient", metric: "Proof Verified" },
    { id: "AG-2", item: "Datasheet p.14 Creepage Citation Check", metric: "BLAKE3 Pinned" },
  ],
};

export const WORKBENCH_ROUTES: Routes = [
  {
    path: "",
    component: WorkbenchStageComponent,
    data: { workbench: WORKBENCH_DESCRIPTOR },
  },
];
