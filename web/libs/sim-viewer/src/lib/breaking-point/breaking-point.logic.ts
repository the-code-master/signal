// Module: web/libs/sim-viewer/src/lib/breaking-point/breaking-point.logic.ts — FEAT-102 First-Failure Stress Solver
// Invariants: Evaluates SOA voltage & thermal headroom in bigint (_uv / _mkelvin) to pinpoint the weakest link first.
// Spec: architecture/05-ai-agent-and-datasheet-rag.md, architecture/17-extended-feature-specifications-ui-and-rust.md
import type { SigGridColumn, SigGridRow, SigStatusTone } from "@signal/ui-kit";
import type { BreakingPointVm, ComponentStressEnvelope } from "./breaking-point.model";

export const DEFAULT_STRESS_ENVELOPES: readonly ComponentStressEnvelope[] = [
  { entityId: "C_DC_LINK_1", failureMechanism: "DIELECTRIC_OVERVOLTAGE_BREAKDOWN", ratedSurgeUv: 920_000_000n, maxTjMkelvin: 398_150n, thermalRisePerVoltMk: 25n },
  { entityId: "Q1_SIC_HS", failureMechanism: "JUNCTION_THERMAL_RUNAWAY", ratedSurgeUv: 1_200_000_000n, maxTjMkelvin: 448_150n, thermalRisePerVoltMk: 60n },
  { entityId: "D_TVS_CLAMP", failureMechanism: "PEAK_PULSE_AVALANCHE_SATURATION", ratedSurgeUv: 880_000_000n, maxTjMkelvin: 423_150n, thermalRisePerVoltMk: 50n },
];

const STRESS_COLUMNS: readonly SigGridColumn[] = [
  { key: "entityId", header: "Entity", mono: true },
  { key: "mechanism", header: "Limiting SOA / Timing Mode", mono: true },
  { key: "headroom", header: "Worst-Case Headroom", mono: true },
  { key: "rank", header: "Failure Order", mono: true },
];

export function buildBreakingPointVm(
  ambientTempMkelvin: bigint,
  supplySurgeUv: bigint,
  envelopes: readonly ComponentStressEnvelope[] = DEFAULT_STRESS_ENVELOPES,
): BreakingPointVm {
  const surgeVolts = supplySurgeUv / 1_000_000n;
  const scored = envelopes
    .map((env) => {
      const estTjMk = ambientTempMkelvin + surgeVolts * env.thermalRisePerVoltMk;
      const vHeadroomPct = Number(((env.ratedSurgeUv - supplySurgeUv) * 1000n) / env.ratedSurgeUv) / 10;
      const tHeadroomPct = Number(((env.maxTjMkelvin - estTjMk) * 1000n) / env.maxTjMkelvin) / 10;
      const minHeadroom = Number(Math.min(vHeadroomPct, tHeadroomPct).toFixed(1));
      return { env, minHeadroom };
    })
    .sort((a, b) => a.minHeadroom - b.minHeadroom);

  const weakest = scored[0];
  const minHeadroomPct = weakest.minHeadroom;
  const summaryTone: SigStatusTone = minHeadroomPct <= 0 ? "danger" : minHeadroomPct <= 12 ? "warn" : "ok";
  const summaryBadge =
    minHeadroomPct <= 0
      ? `BREAKING POINT HIT: ${weakest.env.entityId} fails first (${weakest.env.failureMechanism} @ ${minHeadroomPct}%)`
      : `Weakest Link: ${weakest.env.entityId} (${minHeadroomPct}% Headroom before ${weakest.env.failureMechanism})`;

  const rows: SigGridRow[] = scored.map((item, idx) => ({
    id: item.env.entityId,
    entityId: item.env.entityId,
    mechanism: item.env.failureMechanism,
    headroom: `${item.minHeadroom >= 0 ? "+" : ""}${item.minHeadroom}%`,
    rank: idx === 0 ? "#1 FIRST_TO_FAIL" : `#${idx + 1}`,
  }));

  return {
    ambientTempMkelvin,
    supplySurgeUv,
    firstFailingEntityId: weakest.env.entityId,
    failureMechanism: weakest.env.failureMechanism,
    minHeadroomPct,
    summaryBadge,
    summaryTone,
    columns: STRESS_COLUMNS,
    rows,
  };
}
