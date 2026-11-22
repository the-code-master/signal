// Module: web/libs/library-studio/src/lib/curve-sdk-snippets/curve-sdk-snippets.model.ts — FEAT-073, FEAT-074 & FEAT-079 Models
// Invariants: Wasmtime fuel budgets and snippet anchor coordinates stay in ES2020 bigint.
// Spec: architecture/18-universal-component-and-datasheet-repository.md, architecture/17-extended-feature-specifications-ui-and-rust.md
import type { SigGridColumn, SigGridRow, SigStatusTone } from "@signal/ui-kit";

export type CurveModelFamily = "SIC_RDS_ON_TJ" | "MLCC_DC_BIAS" | "FOSTER_RTH_CTH";

export interface GoldenSnippetDescriptor {
  readonly snippetId: string;
  readonly title: string;
  readonly entityTemplates: readonly string[];
  readonly routedTraceCount: number;
}

export interface StampedSnippetPreview {
  readonly snippetId: string;
  readonly anchorXNm: bigint;
  readonly anchorYNm: bigint;
  readonly remappedRefdes: readonly string[];
}

export interface CurveSdkSnippetsVm {
  readonly activeFamily: CurveModelFamily;
  readonly rmsErrorPct: number;
  readonly curveFitBadge: string;
  readonly curveFitTone: SigStatusTone;
  readonly svgCurvePoints: string;
  readonly wasmSdkBadge: string;
  readonly snippets: readonly GoldenSnippetDescriptor[];
  readonly stampedPreview: StampedSnippetPreview;
  readonly snippetColumns: readonly SigGridColumn[];
  readonly snippetRows: readonly SigGridRow[];
}
