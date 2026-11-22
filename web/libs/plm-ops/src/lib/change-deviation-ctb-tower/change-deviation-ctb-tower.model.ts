// Module: web/libs/plm-ops/src/lib/change-deviation-ctb-tower/change-deviation-ctb-tower.model.ts — Change & CTB Tower Models
// Invariants: Deviation quantities, timestamps (_ms), and work-instruction torques (_milli_nm) stay in ES2020 bigint.
// Spec: architecture/19-cad-cam-3d-printing-and-plm-operations.md, contracts/proto/signal/v1/plm_ops.proto
import type { SigGridColumn, SigGridRow, SigStatusTone } from "@signal/ui-kit";

export interface AssemblyInstructionStep {
  readonly stepNumber: number;
  readonly workstationId: string;
  readonly instructionTitle: string;
  readonly fastenerTorqueMilliNm: bigint;
}

export interface ChangeDeviationCtbTowerVm {
  readonly part11Signed: boolean;
  readonly deviationConsumedQty: bigint;
  readonly deviationLimitQty: bigint;
  readonly sftpDeviationPermitted: boolean;
  readonly ctbMaxBuildableUnits: number;
  readonly activeWorkStep: AssemblyInstructionStep;
  readonly workSteps: readonly AssemblyInstructionStep[];
  readonly deviationGateBadge: string;
  readonly deviationGateTone: SigStatusTone;
  readonly workInstructionBadge: string;
  readonly columns: readonly SigGridColumn[];
  readonly rows: readonly SigGridRow[];
}
