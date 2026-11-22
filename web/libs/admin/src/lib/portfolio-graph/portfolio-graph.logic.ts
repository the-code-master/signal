// Module: web/libs/admin/src/lib/portfolio-graph/portfolio-graph.logic.ts — Transitive Portfolio DAG Logic (FEAT-118)
// Invariants: Pure deterministic functions <=250 LOC; computes multi-hop blast radius & bigint revenue at risk.
// Spec: architecture/14-universal-scale-accessibility-and-expandability.md
import type { SigGridColumn, SigGridRow, SigStatusTone } from "@signal/ui-kit";
import { formatMicrosUsd } from "../finops-dashboard/finops-dashboard.logic";
import type {
  BlastTriggerArtifact,
  PortfolioBlastSummary,
  PortfolioDependencyEdge,
  PortfolioGraphVm,
} from "./portfolio-graph.model";

export const BLAST_TRIGGER_ARTIFACTS: readonly BlastTriggerArtifact[] = [
  "C3M0016120K_SIC_EOL",
  "STM32H755_CVE_2026",
  "SHARED_GATE_DRIVER_IP_V2",
];

export const SEEDED_PORTFOLIO_EDGES: readonly PortfolioDependencyEdge[] = [
  {
    sourceArtifact: "C3M0016120K_SIC_EOL",
    sharedSubBlock: "ip-halfbridge-800v@2.1",
    downstreamProjectSlug: "aurora-ev-inverter",
    transitiveHop: 1,
    quarterlyRevenueMicrosUsd: 18_500_000_000_000n,
  },
  {
    sourceArtifact: "C3M0016120K_SIC_EOL",
    sharedSubBlock: "ip-halfbridge-800v@2.1 -> dc-fast-charger-powerstage",
    downstreamProjectSlug: "hyperion-350kw-charger",
    transitiveHop: 2,
    quarterlyRevenueMicrosUsd: 11_200_000_000_000n,
  },
  {
    sourceArtifact: "STM32H755_CVE_2026",
    sharedSubBlock: "ip-bms-safety-mcu@1.4",
    downstreamProjectSlug: "atlas-grid-bess-controller",
    transitiveHop: 1,
    quarterlyRevenueMicrosUsd: 6_400_000_000_000n,
  },
  {
    sourceArtifact: "SHARED_GATE_DRIVER_IP_V2",
    sharedSubBlock: "ip-isolated-gatedrv@2.0",
    downstreamProjectSlug: "aurora-ev-inverter",
    transitiveHop: 1,
    quarterlyRevenueMicrosUsd: 18_500_000_000_000n,
  },
];

export const PORTFOLIO_EDGE_COLUMNS: readonly SigGridColumn[] = [
  { key: "project", header: "Impacted Project", mono: true },
  { key: "subBlock", header: "Shared IP / Lockfile Path", mono: true },
  { key: "hop", header: "Transitive Hop", mono: true },
  { key: "revenue", header: "Quarterly Revenue at Risk", mono: true },
];

export function analyzePortfolioBlastRadius(
  artifact: BlastTriggerArtifact,
  edges: readonly PortfolioDependencyEdge[] = SEEDED_PORTFOLIO_EDGES,
): PortfolioBlastSummary {
  const matching = edges.filter((e) => e.sourceArtifact === artifact);
  const impactedProjectSlugs = [...new Set(matching.map((e) => e.downstreamProjectSlug))];
  const transitiveDepth = matching.reduce((max, e) => Math.max(max, e.transitiveHop), 0);
  const estimatedRevenueAtRiskMicrosUsd = matching.reduce(
    (sum, e) => sum + e.quarterlyRevenueMicrosUsd,
    0n,
  );
  return {
    selectedArtifact: artifact,
    impactedProjectSlugs,
    transitiveDepth,
    estimatedRevenueAtRiskMicrosUsd,
  };
}

export function buildPortfolioGraphVm(artifact: BlastTriggerArtifact): PortfolioGraphVm {
  const summary = analyzePortfolioBlastRadius(artifact);
  const matching = SEEDED_PORTFOLIO_EDGES.filter((e) => e.sourceArtifact === artifact);
  const riskTone: SigStatusTone = summary.transitiveDepth >= 2 ? "danger" : "warn";

  const edgeRows: SigGridRow[] = matching.map((e, i) => ({
    id: `${e.downstreamProjectSlug}-${i}`,
    project: e.downstreamProjectSlug,
    subBlock: e.sharedSubBlock,
    hop: `Hop ${e.transitiveHop}`,
    revenue: formatMicrosUsd(e.quarterlyRevenueMicrosUsd),
  }));

  return {
    artifacts: BLAST_TRIGGER_ARTIFACTS,
    summary,
    riskTone,
    riskBadge: `${summary.impactedProjectSlugs.length} Projects • Depth ${summary.transitiveDepth} • ${formatMicrosUsd(summary.estimatedRevenueAtRiskMicrosUsd)} Risk`,
    edgeColumns: PORTFOLIO_EDGE_COLUMNS,
    edgeRows,
  };
}
