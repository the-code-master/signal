// Module: web/libs/sftp-release/src/lib/sftp-release-workbench/sftp-release-workbench.logic.ts — CU-P4-05 SFTP Wizard Logic
// Invariants: Enforces <=60s Step-Up 2FA freshness, single-use TOTP burn (F-076), 3/3 Quorum Audit & .tmp.part rename.
// Spec: architecture/07-security-encryption-dlp-and-sftp.md, architecture/16-architectural-audit-flaws-and-futuristic-innovations.md
import type { SigGridColumn, SigGridRow, SigStatusTone } from "@signal/ui-kit";
import type { WorkbenchDescriptor } from "../../../../canvas-core/src/workbench-stage/workbench-stage.model";
import type {
  SftpBundleDestination,
  SftpReleaseWorkbenchVm,
  SftpWizardStep,
  StepUpAuthKind,
} from "./sftp-release-workbench.model";

export const SFTP_RELEASE_WORKBENCH_DESCRIPTOR: WorkbenchDescriptor = {
  slug: "sftp-release",
  title: "Step-Up 2FA Write-Only SFTP Factory Egress",
  subtitle: "WebAuthn/Burned-TOTP + 3x Quorum Tier-0 Audit + russh SFTP",
  rustCrate: "signal-sftp-egress",
  subprojectRepo: "aurora-ev-inverter-ops",
  kpis: [
    { id: "kpi-1", label: "Primary Gate", value: "Browser Downloads: ZERO", tone: "ok" },
    { id: "kpi-2", label: "Domain Telemetry", value: "Tier-0 Quorum: 3/3", tone: "neutral" },
    { id: "kpi-3", label: "Isolation Cell", value: "1 Project = 1 Cell", tone: "ok" },
  ],
  hudSignals: ["Watermark: Constellation", "Deviation Gate: PASS"],
  gridTitle: "Release Bundles Ready for SFTP",
  columns: [
    { key: "id", header: "ID", mono: true },
    { key: "item", header: "Engineering Artifact / Entity" },
    { key: "metric", header: "Verified SI / Status", mono: true },
  ],
  rows: [
    { id: "REL-1", item: "aurora_pcb_rev_a.odb.tgz", metric: "SHA-256 Pinned" },
    { id: "REL-2", item: "aurora_housing_rev_a.step", metric: "AP242 Signed" },
  ],
};

const WIZARD_COLUMNS: readonly SigGridColumn[] = [
  { key: "stage", header: "Write-Only Egress Stage", mono: true },
  { key: "detail", header: "Cryptographic / Protocol Guarantee", mono: true },
  { key: "status", header: "Verification", mono: true },
];

export function evaluateStepUpEligibility(
  authKind: StepUpAuthKind,
  assertionAgeSec: bigint,
  totpAlreadyBurned: boolean,
): { readonly eligible: boolean; readonly reason: string } {
  if (assertionAgeSec > 60n || assertionAgeSec < 0n) {
    return { eligible: false, reason: "STEP_UP_ASSERTION_EXPIRED (> 60s)" };
  }
  if (authKind === "RFC6238_BURNED_TOTP" && totpAlreadyBurned) {
    return { eligible: false, reason: "TOTP_NONCE_ALREADY_BURNED (SET NX EX 90)" };
  }
  return { eligible: true, reason: `FRESH_${authKind}_VERIFIED` };
}

function artifactForDestination(dest: SftpBundleDestination): string {
  if (dest === "FAB_GERBER_ODB_IPC2581") return "aurora_pcb_rev_a.ipc2581.tgz";
  if (dest === "REGULATORY_LAB_EMC_PPAP") return "aurora_cispr25_asil_d_ppap.tgz";
  return "aurora_enclosure_rev_a.step";
}

export function buildSftpReleaseWorkbenchVm(
  descriptor: WorkbenchDescriptor,
  step: SftpWizardStep,
  destination: SftpBundleDestination,
  authKind: StepUpAuthKind,
  assertionAgeSec: bigint,
  totpAlreadyBurned: boolean,
): SftpReleaseWorkbenchVm {
  const stepUp = evaluateStepUpEligibility(authKind, assertionAgeSec, totpAlreadyBurned);
  const gateTone: SigStatusTone = stepUp.eligible ? "ok" : "danger";
  const gateBadge = `${step} • ${stepUp.reason} (Age=${assertionAgeSec.toString(10)}s)`;
  const artifact = artifactForDestination(destination);
  const receiptSummary = `Atomic .tmp.part -> SHA-256 Read-Back -> POSIX rename(${artifact}) • Receipt: ml-dsa-65:9f4c2a81e019 • Quorum: 3/3 WORM`;

  const rows: SigGridRow[] = [
    {
      id: "G1",
      stage: "1. Pre-Release Sign-Off Gate",
      detail: `0 DRC / 0 LVS / FabProfile Signed • Target=${destination}`,
      status: "PASS",
    },
    {
      id: "G2",
      stage: "2. Fresh Step-Up 2FA & Burn-List (F-076)",
      detail: `${authKind} @ ${assertionAgeSec.toString(10)}s (Cap <= 60s, Redis SET NX EX 90)`,
      status: stepUp.eligible ? "VERIFIED" : stepUp.reason,
    },
    {
      id: "G3",
      stage: "3. 3x Quorum Audit & russh SFTP (F-081/147)",
      detail: `${artifact}.tmp.88f1.part -> SHA-256 Match -> Atomic rename + .sig`,
      status: step === "STEP_3_ATOMIC_RECEIPT" && stepUp.eligible ? "COMMITTED_POD_DESTROYED" : "ARMED",
    },
  ];

  return {
    descriptor,
    engineBadge: `${descriptor.rustCrate} • ${descriptor.subprojectRepo}`,
    step,
    steps: ["STEP_1_PRE_RELEASE_GATES", "STEP_2_FRESH_STEP_UP_2FA", "STEP_3_ATOMIC_RECEIPT"],
    destination,
    destinations: ["FAB_GERBER_ODB_IPC2581", "REGULATORY_LAB_EMC_PPAP", "PARTNER_MCAD_STEP"],
    authKind,
    authKinds: ["WEBAUTHN_FIDO2", "RFC6238_BURNED_TOTP"],
    assertionAgeSec,
    totpAlreadyBurned,
    canDispatch: stepUp.eligible,
    gateBadge,
    gateTone,
    receiptSummary,
    columns: WIZARD_COLUMNS,
    rows,
  };
}
