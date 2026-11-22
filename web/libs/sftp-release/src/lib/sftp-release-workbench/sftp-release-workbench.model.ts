// Module: web/libs/sftp-release/src/lib/sftp-release-workbench/sftp-release-workbench.model.ts — CU-P4-05 SFTP Wizard Models
// Invariants: Step-up timestamps (_sec bigint) stay in ES2020 bigint; zero browser file-download URLs or blobs.
// Spec: architecture/07-security-encryption-dlp-and-sftp.md, architecture/16-architectural-audit-flaws-and-futuristic-innovations.md
import type { SigGridColumn, SigGridRow, SigStatusTone } from "@signal/ui-kit";
import type { WorkbenchDescriptor } from "../../../../canvas-core/src/workbench-stage/workbench-stage.model";

export type SftpWizardStep =
  | "STEP_1_PRE_RELEASE_GATES"
  | "STEP_2_FRESH_STEP_UP_2FA"
  | "STEP_3_ATOMIC_RECEIPT";

export type SftpBundleDestination =
  | "FAB_GERBER_ODB_IPC2581"
  | "REGULATORY_LAB_EMC_PPAP"
  | "PARTNER_MCAD_STEP";

export type StepUpAuthKind = "WEBAUTHN_FIDO2" | "RFC6238_BURNED_TOTP";

export interface SftpReleaseWorkbenchVm {
  readonly descriptor: WorkbenchDescriptor;
  readonly engineBadge: string;
  readonly step: SftpWizardStep;
  readonly steps: readonly SftpWizardStep[];
  readonly destination: SftpBundleDestination;
  readonly destinations: readonly SftpBundleDestination[];
  readonly authKind: StepUpAuthKind;
  readonly authKinds: readonly StepUpAuthKind[];
  readonly assertionAgeSec: bigint;
  readonly totpAlreadyBurned: boolean;
  readonly canDispatch: boolean;
  readonly gateBadge: string;
  readonly gateTone: SigStatusTone;
  readonly receiptSummary: string;
  readonly columns: readonly SigGridColumn[];
  readonly rows: readonly SigGridRow[];
}
