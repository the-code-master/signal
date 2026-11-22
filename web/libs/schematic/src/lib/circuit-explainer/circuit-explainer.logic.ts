// Module: web/libs/schematic/src/lib/circuit-explainer/circuit-explainer.logic.ts — FEAT-101 VF2 Stage & H(s) Logic
// Invariants: Deterministic VF2 stage catalog and symbolic H(s) transfer cards with bigint nm bounds.
// Spec: architecture/05-ai-agent-and-datasheet-rag.md, architecture/17-extended-feature-specifications-ui-and-rust.md
import type { SigGridColumn, SigGridRow } from "@signal/ui-kit";
import type { CircuitExplainerVm, FunctionalStageBlock } from "./circuit-explainer.model";

export const DEFAULT_CIRCUIT_STAGES: readonly FunctionalStageBlock[] = [
  {
    stageId: "STG-EMI",
    topologyName: "2-Stage Common-Mode + Differential Pi EMI Filter",
    symbolicTransferFn: "H_emi(s) = 1 / (1 + s*(R_d*C_x) + s^2*(L_dm*C_x))",
    cornerFreqHz: 145_000n,
    minXNm: 0n,
    minYNm: 0n,
    maxXNm: 12_000_000n,
    maxYNm: 18_000_000n,
  },
  {
    stageId: "STG-SIC-HB",
    topologyName: "800V SiC Synchronous Half-Bridge + Kelvin Gate Drive",
    symbolicTransferFn: "V_sw(D) = D * V_dc - I_load * R_ds_on(T_j)",
    cornerFreqHz: 2_000_000n,
    minXNm: 12_500_000n,
    minYNm: 0n,
    maxXNm: 28_000_000n,
    maxYNm: 18_000_000n,
  },
  {
    stageId: "STG-COMP",
    topologyName: "Type-III Isolated Error Amplifier Loop Compensator",
    symbolicTransferFn: "G_c(s) = K * (1 + s/ω_z1)*(1 + s/ω_z2) / (s * (1 + s/ω_p1)*(1 + s/ω_p2))",
    cornerFreqHz: 85_000n,
    minXNm: 28_500_000n,
    minYNm: 0n,
    maxXNm: 42_000_000n,
    maxYNm: 18_000_000n,
  },
];

const STAGE_COLUMNS: readonly SigGridColumn[] = [
  { key: "stageId", header: "Stage", mono: true },
  { key: "topology", header: "Recognized VF2 Subgraph Topology" },
  { key: "transfer", header: "Symbolic Transfer H(s)", mono: true },
  { key: "f0", header: "Char Freq", mono: true },
];

export function buildCircuitExplainerVm(
  selectedStageId: string,
  stages: readonly FunctionalStageBlock[] = DEFAULT_CIRCUIT_STAGES,
): CircuitExplainerVm {
  const selectedStage = stages.find((s) => s.stageId === selectedStageId) ?? stages[0];
  const signalFlowSummary = stages.map((s) => s.stageId).join(" -> ");
  const bboxBadge = `BBox: (${selectedStage.minXNm.toString(10)}, ${selectedStage.minYNm.toString(10)})..(${selectedStage.maxXNm.toString(10)}, ${selectedStage.maxYNm.toString(10)}) nm`;

  const rows: SigGridRow[] = stages.map((s) => ({
    id: s.stageId,
    stageId: s.stageId,
    topology: s.topologyName,
    transfer: s.symbolicTransferFn,
    f0: `${(s.cornerFreqHz / 1_000n).toString(10)} kHz`,
  }));

  return {
    stages,
    selectedStage,
    signalFlowSummary,
    bboxBadge,
    columns: STAGE_COLUMNS,
    rows,
  };
}
