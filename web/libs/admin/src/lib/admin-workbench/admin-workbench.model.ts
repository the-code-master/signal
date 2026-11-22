// Module: web/libs/admin/src/lib/admin-workbench/admin-workbench.model.ts — Access Review & vcs-auditor Posture Models
// Invariants: Contractor expires_at timestamps stay in ES2020 bigint (_ms); posture score bounded 0..100.
// Spec: architecture/08-identity-enterprise-and-project-model.md, architecture/13-*.md
import type { SigGridColumn, SigGridRow, SigStatusTone } from "@signal/ui-kit";
import type { WorkbenchDescriptor } from "../../../../canvas-core/src/workbench-stage/workbench-stage.model";

export interface RepoAccessAuditRow {
  readonly userId: string;
  readonly role: string;
  readonly repoScope: string;
  readonly expiresAtUtcMs: bigint;
}

export interface AdminWorkbenchVm {
  readonly descriptor: WorkbenchDescriptor;
  readonly engineBadge: string;
  readonly postureBadge: string;
  readonly postureTone: SigStatusTone;
  readonly breakGlassBadge: string;
  readonly accessColumns: readonly SigGridColumn[];
  readonly accessRows: readonly SigGridRow[];
}
