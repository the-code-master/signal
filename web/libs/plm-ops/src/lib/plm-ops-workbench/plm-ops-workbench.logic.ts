// Module: web/libs/plm-ops/src/lib/plm-ops-workbench/plm-ops-workbench.logic.ts — PLM & Supply-Chain Host Logic
// Invariants: Zero Angular decorators; owns PLM_OPS_WORKBENCH_DESCRIPTOR and signed ERP/MES webhook state.
// Spec: architecture/19-cad-cam-3d-printing-and-plm-operations.md
import type { WorkbenchDescriptor } from "../../../../canvas-core/src/workbench-stage/workbench-stage.model";
import type { ErpConnectorPreset, PlmOpsWorkbenchVm } from "./plm-ops-workbench.model";

export const PLM_OPS_WORKBENCH_DESCRIPTOR: WorkbenchDescriptor = {
  slug: "plm-ops",
  title: "Native PLM: eBOM/mBOM, ECN/MCN & Clear-to-Build",
  subtitle: "Unified change control, qty-capped Deviations, 8D CAPA & CTB readiness (19-*)",
  rustCrate: "signal-plm",
  subprojectRepo: "aurora-ev-inverter-ops",
  kpis: [
    { id: "kpi-1", label: "Primary Gate", value: "CTB Readiness: 98.5%", tone: "ok" },
    { id: "kpi-2", label: "Domain Telemetry", value: "Open ECNs: 1", tone: "neutral" },
    { id: "kpi-3", label: "Isolation Cell", value: "1 Project = 1 Cell", tone: "ok" },
  ],
  hudSignals: ["Unit Cost: $157.50", "Deviations: 1 Active (450/500)"],
  gridTitle: "BOM & Change Orders",
  columns: [
    { key: "id", header: "ID", mono: true },
    { key: "item", header: "Engineering Artifact / Entity" },
    { key: "metric", header: "Verified SI / Status", mono: true },
  ],
  rows: [
    { id: "ECN-104", item: "Upgrade DC-link film cap ripple rating", metric: "CCB_APPROVED" },
    { id: "DEV-004", item: "Alternate NTC thermistor lot (50 units left)", metric: "QTY_BOUNDED" },
  ],
};

export function buildPlmOpsWorkbenchVm(
  descriptor: WorkbenchDescriptor,
  erpTarget: ErpConnectorPreset,
): PlmOpsWorkbenchVm {
  return {
    descriptor,
    engineBadge: `${descriptor.rustCrate} • ${descriptor.subprojectRepo}`,
    erpTarget,
    erpTargets: ["SAP_S4HANA", "ORACLE_NETSUITE", "ARENA_PLM", "FACTORY_MES"],
    erpSyncBadge: `HMAC-SHA256 Outbound ERP/MES Sync Armed -> ${erpTarget} (via signal-egress-proxy)`,
  };
}
