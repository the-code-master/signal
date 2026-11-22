// Module: web/libs/collab-hub/src/index.ts — Public Barrel for @signal/collab-hub
// Invariants: Sole public entrypoint for lazy route loading and Phase 7 Collab Space & Auto-Review exports.
// Spec: docs/engineering/ANGULAR_ENGINEERING_GUIDE.md
export { WORKBENCH_DESCRIPTOR, WORKBENCH_ROUTES } from "./collab-hub.routes";
export { CollabHubWorkbenchComponent } from "./lib/collab-hub-workbench/collab-hub-workbench.component";
export { TeamChannelsHuddleComponent } from "./lib/team-channels-huddle/team-channels-huddle.component";
export { ProjectStatusTowerComponent } from "./lib/project-status-tower/project-status-tower.component";
export { AutomatedReviewStudioComponent } from "./lib/automated-review-studio/automated-review-studio.component";
