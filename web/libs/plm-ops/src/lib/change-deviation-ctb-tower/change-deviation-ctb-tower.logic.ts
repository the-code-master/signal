// Module: web/libs/plm-ops/src/lib/change-deviation-ctb-tower/change-deviation-ctb-tower.logic.ts — Change, Deviation, 8D & 3D WI Logic
// Invariants: Blocks SFTP release when Deviation consumedQty >= authorizedLimitQty; formats 3D work instructions in bigint mN·m.
// Spec: architecture/19-cad-cam-3d-printing-and-plm-operations.md, contracts/proto/signal/v1/plm_ops.proto
import type { SigGridColumn, SigGridRow, SigStatusTone } from "@signal/ui-kit";
import type {
  AssemblyInstructionStep,
  ChangeDeviationCtbTowerVm,
} from "./change-deviation-ctb-tower.model";

export const DEFAULT_ASSEMBLY_WORK_STEPS: readonly AssemblyInstructionStep[] = [
  {
    stepNumber: 1,
    workstationId: "WS-10-SMT-TOP",
    instructionTitle: "Reflow Top-Side SiC Power Stage & AOI Inspect Kelvin Source Pads",
    fastenerTorqueMilliNm: 0n,
  },
  {
    stepNumber: 2,
    workstationId: "WS-40-CNC-MECH",
    instructionTitle: "Dispense TIM Gap-Filler & Mate AlSi10Mg Cold-Plate on 12x M4 Bosses",
    fastenerTorqueMilliNm: 2_800n,
  },
  {
    stepNumber: 3,
    workstationId: "WS-50-FINAL-TORQUE",
    instructionTitle: "Seat EPDM IP67 Gasket & Torque HV HVP800 Busbar Lugs (Star Pattern)",
    fastenerTorqueMilliNm: 8_500n,
  },
];

const TOWER_COLUMNS: readonly SigGridColumn[] = [
  { key: "recordId", header: "Change / Quality / CTB Record", mono: true },
  { key: "category", header: "Workflow Stage", mono: true },
  { key: "summary", header: "Redline / Disposition / Waterfall Detail" },
  { key: "gate", header: "21 CFR Part 11 / SFTP Gate", mono: true },
];

export function isDeviationReleasePermitted(
  consumedQty: bigint,
  requestedLotQty: bigint,
  limitQty: bigint,
): boolean {
  return consumedQty + requestedLotQty <= limitQty;
}

export function buildChangeDeviationCtbTowerVm(
  part11Signed: boolean,
  deviationConsumedQty: bigint,
  activeStepNum: number,
  steps: readonly AssemblyInstructionStep[] = DEFAULT_ASSEMBLY_WORK_STEPS,
): ChangeDeviationCtbTowerVm {
  const deviationLimitQty = 500n;
  const sftpDeviationPermitted = isDeviationReleasePermitted(
    deviationConsumedQty,
    50n,
    deviationLimitQty,
  );
  const ctbMaxBuildableUnits = 400;
  const activeWorkStep =
    steps.find((s) => s.stepNumber === activeStepNum) ?? steps[0];

  const deviationGateTone: SigStatusTone = sftpDeviationPermitted ? "ok" : "danger";
  const remaining = deviationLimitQty - deviationConsumedQty;
  const deviationGateBadge = sftpDeviationPermitted
    ? `DEV-2026-004 Active: ${deviationConsumedQty.toString(10)}/${deviationLimitQty.toString(10)} Used (${remaining.toString(10)} Left) • SFTP Permitted`
    : `DEV-2026-004 EXHAUSTED (${deviationConsumedQty.toString(10)}/${deviationLimitQty.toString(10)}) -> SFTP RELEASE HARD-BLOCKED (409)`;

  const torqueNm = (Number(activeWorkStep.fastenerTorqueMilliNm) / 1_000).toFixed(2);
  const workInstructionBadge = `3D Work Instruction Step ${activeWorkStep.stepNumber}/${steps.length} (${activeWorkStep.workstationId}): ${activeWorkStep.instructionTitle} [Torque=${torqueNm} N·m]`;

  const rows: SigGridRow[] = [
    {
      id: "ECN-104",
      recordId: "ECN-2026-104",
      category: part11Signed ? "ECN • RELEASED" : "ECO • CCB_REVIEW",
      summary: "Upgrade DC-link film cap ripple rating • WIP=REWORK, Stock=USE_AS_IS",
      gate: part11Signed ? "ml-dsa-65:CCB_QUORUM_SIGNED" : "AWAITING_PART11_2FA",
    },
    {
      id: "MCN-019",
      recordId: "MCN-2026-019",
      category: "MCN • WS-40-CNC-MECH",
      summary: "Update cold-plate M4 torque 2_500 -> 2_800 mN·m & 5-axis recipe hash",
      gate: "ACTIVE_ON_MES",
    },
    {
      id: "PCN-088",
      recordId: "PCN-TI-202611",
      category: "PCN -> Auto-ECR",
      summary: "UCC21750QDWR fab die shrink notice -> Spawned ECR-8A12F901",
      gate: "LTB_TRACKED",
    },
    {
      id: "NCR-042",
      recordId: "NCR-2026-042",
      category: "8D Step D5_CORRECTIVE",
      summary: "SN-AUR-000419 F2_GASKET_GROOVE corner compression -> CAPA groove +120 um",
      gate: "CONTAINED",
    },
    {
      id: "CTB-LOT",
      recordId: "CTB-500-UNIT-LOT",
      category: `Max Buildable: ${ctbMaxBuildableUnits}`,
      summary: "Gating Shortage: UCC21750QDWR (-600 pcs for 500-unit lot, ETA +21d)",
      gate: "SHORTAGE_WATERFALL",
    },
  ];

  return {
    part11Signed,
    deviationConsumedQty,
    deviationLimitQty,
    sftpDeviationPermitted,
    ctbMaxBuildableUnits,
    activeWorkStep,
    workSteps: steps,
    deviationGateBadge,
    deviationGateTone,
    workInstructionBadge,
    columns: TOWER_COLUMNS,
    rows,
  };
}
