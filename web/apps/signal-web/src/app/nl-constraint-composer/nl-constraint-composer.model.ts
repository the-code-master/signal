// Module: web/apps/signal-web/src/app/nl-constraint-composer/nl-constraint-composer.model.ts — FEAT-105 NL Rule Compiler Models
// Invariants: Compiles natural-language prompts into deterministic DRC, UPF, SDC, or SVA rule ASTs (zero eval).
// Spec: architecture/05-ai-agent-and-datasheet-rag.md, architecture/17-extended-feature-specifications-ui-and-rust.md
import type { SigStatusTone } from "@signal/ui-kit";

export type ConstraintTargetLanguage = "DRC" | "UPF" | "SDC" | "SVA";

export interface CompiledConstraintAst {
  readonly valid: boolean;
  readonly targetLanguage: ConstraintTargetLanguage;
  readonly compiledRuleDsl: string;
  readonly statusBadge: string;
  readonly tone: SigStatusTone;
}

export interface NlConstraintComposerVm {
  readonly prompt: string;
  readonly targetLanguage: ConstraintTargetLanguage;
  readonly languages: readonly ConstraintTargetLanguage[];
  readonly compiled: CompiledConstraintAst;
}
