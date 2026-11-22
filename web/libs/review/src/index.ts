// Module: web/libs/review/src/index.ts — Public Barrel for @signal/review
// Invariants: Sole public entrypoint for lazy route loading and FEAT-094 / FEAT-098 / FEAT-119 review tools.
// Spec: docs/engineering/ANGULAR_ENGINEERING_GUIDE.md
export { ExternalPartnerPortalComponent } from "./lib/external-partner-portal/external-partner-portal.component";
export {
  buildExternalPartnerPortalVm,
  stripSensitiveFieldsForExternalPartner,
} from "./lib/external-partner-portal/external-partner-portal.logic";
export type {
  ExternalPartnerPortalVm,
  InternalReviewEntity,
  StrippedPartnerEntity,
} from "./lib/external-partner-portal/external-partner-portal.model";
export { LiveHuddlePresenceComponent } from "./lib/live-huddle-presence/live-huddle-presence.component";
export { buildLiveHuddlePresenceVm } from "./lib/live-huddle-presence/live-huddle-presence.logic";
export type {
  CollaboratorPresencePeer,
  LiveHuddlePresenceVm,
} from "./lib/live-huddle-presence/live-huddle-presence.model";
export { ReviewWorkbenchComponent } from "./lib/review-workbench/review-workbench.component";
export { buildReviewWorkbenchVm } from "./lib/review-workbench/review-workbench.logic";
export type { ReviewWorkbenchVm, SpatialReviewThread } from "./lib/review-workbench/review-workbench.model";
export { StageGateChecklistComponent } from "./lib/stage-gate-checklist/stage-gate-checklist.component";
export { buildStageGateChecklistVm } from "./lib/stage-gate-checklist/stage-gate-checklist.logic";
export type {
  HardwareStageGate,
  StageGateChecklistVm,
  StageGateCriterion,
} from "./lib/stage-gate-checklist/stage-gate-checklist.model";
export { LiveReviewDeckComponent } from "./lib/live-review-deck/live-review-deck.component";
export {
  DEFAULT_REVIEW_SLIDES,
  buildLiveReviewDeckVm,
  stepSlideIndex,
} from "./lib/live-review-deck/live-review-deck.logic";
export type {
  LiveReviewDeckVm,
  LiveReviewSlide,
  ReviewDeckMilestone,
} from "./lib/live-review-deck/live-review-deck.model";
export { PrincipalAuditComponent } from "./lib/principal-audit/principal-audit.component";
export { DEFAULT_REDTEAM_FINDINGS, buildPrincipalAuditVm } from "./lib/principal-audit/principal-audit.logic";
export type {
  AuditSeverityFilter,
  PrincipalAuditVm,
  RedTeamAuditItem,
} from "./lib/principal-audit/principal-audit.model";
export { WORKBENCH_DESCRIPTOR, WORKBENCH_ROUTES } from "./review.routes";
