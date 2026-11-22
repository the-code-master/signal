// Module: web/libs/admin/src/lib/portfolio-graph/portfolio-graph.model.ts — Portfolio Blast-Radius Models (FEAT-118)
// Invariants: Pure readonly TypeScript types; 64-bit estimated_revenue_at_risk_micros_usd uses bigint ONLY.
// Spec: architecture/14-universal-scale-accessibility-and-expandability.md
import type { SigGridColumn, SigGridRow, SigStatusTone } from "@signal/ui-kit";

export type BlastTriggerArtifact = "C3M0016120K_SIC_EOL" | "STM32H755_CVE_2026" | "SHARED_GATE_DRIVER_IP_V2";

export interface PortfolioDependencyEdge {
  readonly sourceArtifact: BlastTriggerArtifact;
  readonly sharedSubBlock: string;
  readonly downstreamProjectSlug: string;
  readonly transitiveHop: number;
  readonly quarterlyRevenueMicrosUsd: bigint;
}

export interface PortfolioBlastSummary {
  readonly selectedArtifact: BlastTriggerArtifact;
  readonly impactedProjectSlugs: readonly string[];
  readonly transitiveDepth: number;
  readonly estimatedRevenueAtRiskMicrosUsd: bigint;
}

export interface PortfolioGraphVm {
  readonly artifacts: readonly BlastTriggerArtifact[];
  readonly summary: PortfolioBlastSummary;
  readonly riskTone: SigStatusTone;
  readonly riskBadge: string;
  readonly edgeColumns: readonly SigGridColumn[];
  readonly edgeRows: readonly SigGridRow[];
}
