// Module: web/libs/sftp-release/src/index.ts — Public Barrel for @signal/sftp-release
// Invariants: Sole public entrypoint for lazy route loading and CU-P4-05 3-Step SFTP Release Wizard.
// Spec: docs/engineering/ANGULAR_ENGINEERING_GUIDE.md
export { SftpReleaseWorkbenchComponent } from "./lib/sftp-release-workbench/sftp-release-workbench.component";
export {
  SFTP_RELEASE_WORKBENCH_DESCRIPTOR,
  buildSftpReleaseWorkbenchVm,
  evaluateStepUpEligibility,
} from "./lib/sftp-release-workbench/sftp-release-workbench.logic";
export type {
  SftpBundleDestination,
  SftpReleaseWorkbenchVm,
  SftpWizardStep,
  StepUpAuthKind,
} from "./lib/sftp-release-workbench/sftp-release-workbench.model";
export { WORKBENCH_DESCRIPTOR, WORKBENCH_ROUTES } from "./sftp-release.routes";
