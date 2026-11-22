// Module: web/libs/canvas-core/src/index.ts — Public @signal/canvas-core Barrel
// Invariants: Exports DualPlaneViewportComponent and WorkbenchStageComponent for lazy domain libraries.
// Spec: architecture/06-frontend-angular-and-fastapi-control-plane.md
export { DualPlaneViewportComponent } from "./viewport/dual-plane-viewport.component";
export type { ViewportReticleVm } from "./viewport/dual-plane-viewport.model";
export { WorkbenchStageComponent } from "./workbench-stage/workbench-stage.component";
export type { WorkbenchDescriptor, WorkbenchKpiCard } from "./workbench-stage/workbench-stage.model";
