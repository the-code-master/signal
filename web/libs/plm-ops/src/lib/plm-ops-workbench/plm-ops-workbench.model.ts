// Module: web/libs/plm-ops/src/lib/plm-ops-workbench/plm-ops-workbench.model.ts — PLM & Supply-Chain Host Models
// Invariants: Organizes Multi-BOM/DPP and ECN/MCN/PCN/Deviation/8D/CTB/3D-Work-Instructions studios cleanly.
// Spec: architecture/19-cad-cam-3d-printing-and-plm-operations.md
import type { WorkbenchDescriptor } from "../../../../canvas-core/src/workbench-stage/workbench-stage.model";

export type ErpConnectorPreset = "SAP_S4HANA" | "ORACLE_NETSUITE" | "ARENA_PLM" | "FACTORY_MES";

export interface PlmOpsWorkbenchVm {
  readonly descriptor: WorkbenchDescriptor;
  readonly engineBadge: string;
  readonly erpTarget: ErpConnectorPreset;
  readonly erpTargets: readonly ErpConnectorPreset[];
  readonly erpSyncBadge: string;
}
