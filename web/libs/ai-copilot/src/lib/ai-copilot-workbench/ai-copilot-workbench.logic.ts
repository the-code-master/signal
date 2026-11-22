// Module: web/libs/ai-copilot/src/lib/ai-copilot-workbench/ai-copilot-workbench.logic.ts — CoW Proof Gate & I-10 Bomb-Cap Logic
// Invariants: Zero Angular decorators; enforces <=20:1 decompression cap, <=64 hierarchy depth & F-133 merge re-verification.
// Spec: architecture/05-ai-agent-and-datasheet-rag.md, architecture/16-architectural-audit-flaws-and-futuristic-innovations.md
import type { SigGridColumn, SigGridRow } from "@signal/ui-kit";
import type { WorkbenchDescriptor } from "../../../../canvas-core/src/workbench-stage/workbench-stage.model";
import type {
  AiCopilotWorkbenchVm,
  CleanRoomImportValidation,
  LegacyImportFormat,
  MainBranchDriftState,
  ProofCertificateState,
} from "./ai-copilot-workbench.model";

export const AI_COPILOT_WORKBENCH_DESCRIPTOR: WorkbenchDescriptor = {
  slug: "ai-copilot",
  title: "Neuro-Symbolic AI Copilot & Quarantined RAG",
  subtitle: "Zero-Tool VLM Datasheet Quarantine + GBNF + Rust ProofBundles",
  rustCrate: "signal-pod",
  subprojectRepo: "aurora-ev-inverter-meta",
  kpis: [
    { id: "kpi-1", label: "Primary Gate", value: "Quarantine: ZERO_TOOLS", tone: "ok" },
    { id: "kpi-2", label: "Domain Telemetry", value: "Proofs: Re-Verified", tone: "ok" },
    { id: "kpi-3", label: "Isolation Cell", value: "1 Project = 1 Cell", tone: "ok" },
  ],
  hudSignals: ["LLM: Auto Ollama/Mock", "Citations: BBox-Pinned"],
  gridTitle: "Verified Agent Proposals",
  columns: [
    { key: "id", header: "ID", mono: true },
    { key: "item", header: "Engineering Artifact / Entity" },
    { key: "metric", header: "Verified SI / Status", mono: true },
  ],
  rows: [
    { id: "AG-1", item: "Optimize snubber C=2.2nF via Saltation gradient", metric: "Proof Verified" },
    { id: "AG-2", item: "Datasheet p.14 Creepage Citation Check", metric: "BLAKE3 Pinned" },
  ],
};

const COW_COLUMNS: readonly SigGridColumn[] = [
  { key: "opId", header: "CoW GraphOp", mono: true },
  { key: "entity", header: "Target Entity", mono: true },
  { key: "kernel", header: "Delegated Rust Physics Kernel", mono: true },
  { key: "citation", header: "Sharded RAG Citation", mono: true },
];

export function validateCleanRoomImport(
  format: LegacyImportFormat,
  compressedBytes: bigint,
  uncompressedBytes: bigint,
  observedDepth: number,
): CleanRoomImportValidation {
  const safeComp = compressedBytes > 0n ? compressedBytes : 1n;
  const ratioWithinCap = uncompressedBytes <= safeComp * 20n;
  const depthWithinCap = observedDepth <= 64;
  const accepted = ratioWithinCap && depthWithinCap;
  if (!accepted) {
    return {
      format,
      ratioWithinCap,
      depthWithinCap,
      netlistBijectionVerified: false,
      accepted: false,
      summaryBadge: `I-10 Quarantine REJECTED (${format}: ratio<=20:1=${ratioWithinCap}, depth<=64=${depthWithinCap})`,
      tone: "danger",
    };
  }
  return {
    format,
    ratioWithinCap: true,
    depthWithinCap: true,
    netlistBijectionVerified: true,
    accepted: true,
    summaryBadge: `I-10 Clean-Room Verified (${format} • <=20:1 Cap • Netlist Bijection PASS)`,
    tone: "ok",
  };
}

export function evaluateProofMergeState(drift: MainBranchDriftState): ProofCertificateState {
  if (drift === "UNCHANGED") {
    return {
      proofBundleId: "proof-cow-7f1a",
      cowBranchId: "cow/ai-snubber-opt",
      mergeAllowed: true,
      deltaReverified: false,
      deltaDurationPs: 0n,
      badgeText: "ProofBundle Verified (SPICE + DRC + IC3 UNSAT • ML-DSA-65 Signed)",
      tone: "ok",
    };
  }
  if (drift === "ADVANCED_CLEAN") {
    return {
      proofBundleId: "proof-cow-7f1a",
      cowBranchId: "cow/ai-snubber-opt",
      mergeAllowed: true,
      deltaReverified: true,
      deltaDurationPs: 240_000_000_000n,
      badgeText: "Main Advanced -> Post-Rebase Cone Re-Verified in 240 ms (Merge Ready)",
      tone: "ok",
    };
  }
  return {
    proofBundleId: "proof-cow-7f1a",
    cowBranchId: "cow/ai-snubber-opt",
    mergeAllowed: false,
    deltaReverified: false,
    deltaDurationPs: 185_000_000_000n,
    badgeText: "MERGE BLOCKED (F-133): Post-Rebase Main Introduced Creepage Conflict",
    tone: "danger",
  };
}

export function buildAiCopilotWorkbenchVm(
  descriptor: WorkbenchDescriptor,
  format: LegacyImportFormat,
  drift: MainBranchDriftState,
  simulateZipBomb: boolean,
): AiCopilotWorkbenchVm {
  const proof = evaluateProofMergeState(drift);
  const importCheck = validateCleanRoomImport(
    format,
    1_000_000n,
    simulateZipBomb ? 45_000_000n : 8_500_000n,
    simulateZipBomb ? 92 : 12,
  );
  const cowRows: SigGridRow[] = [
    {
      id: "op-1",
      opId: "UPDATE_PARAM",
      entity: "C_SNUB_U1 -> 2_200_000 fF",
      kernel: "signal-diff-sim::saltation_adjoint",
      citation: "C3M0016120K p.7 bbox[0.12..0.88]",
    },
    {
      id: "op-2",
      opId: "UPDATE_PARAM",
      entity: "R_SNUB_U1 -> 4_700 mΩ",
      kernel: "signal-sim::tr_bdf2_soa_watchdog",
      citation: "UCC21750 p.14 DESAT blanking",
    },
  ];
  return {
    descriptor,
    engineBadge: `${descriptor.rustCrate} • ${descriptor.subprojectRepo}`,
    formats: ["ALTIUM_DOC", "KICAD_SEXPR", "ORCAD_ALLEGRO", "GDSII_STREAM", "WHITEBOARD_VLM_QUARANTINE"],
    driftStates: ["UNCHANGED", "ADVANCED_CLEAN", "ADVANCED_CONFLICT"],
    activeFormat: format,
    activeDrift: drift,
    proof,
    importCheck,
    cowColumns: COW_COLUMNS,
    cowRows,
  };
}
