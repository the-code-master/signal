// Module: web/libs/admin/src/lib/avl-where-used-console/avl-where-used-console.model.ts — FEAT-075 Cross-Project AVL Models
// Invariants: Pure deterministic Where-Used blast radius and 1-click ECO draft models.
// Spec: architecture/18-universal-component-and-datasheet-repository.md, architecture/17-*.md
import type { SigGridColumn, SigGridRow, SigStatusTone } from "@signal/ui-kit";

export interface AvlBlastRadiusEntry {
  readonly targetMpn: string;
  readonly lifecycleStatus: "NRND" | "OBSOLETE" | "PREFERRED";
  readonly dropInMpn: string;
  readonly affectedProjects: readonly {
    readonly projectSlug: string;
    readonly instanceCount: number;
    readonly stageGate: string;
  }[];
}

export interface AvlWhereUsedVm {
  readonly mpns: readonly string[];
  readonly selectedEntry: AvlBlastRadiusEntry;
  readonly blastBadge: string;
  readonly blastTone: SigStatusTone;
  readonly ecoDraftBadge: string;
  readonly columns: readonly SigGridColumn[];
  readonly rows: readonly SigGridRow[];
}
