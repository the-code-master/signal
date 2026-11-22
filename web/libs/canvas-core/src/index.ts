// Module: web/libs/canvas-core/src/index.ts — Public @signal/canvas-core Barrel
// Invariants: Exports DualPlaneViewportComponent, Dual-Plane QOI client, 120 FPS HudOverlay math, and Volatile Reconnect buffer.
// Spec: architecture/06-frontend-angular-and-fastapi-control-plane.md
export {
  advanceSettlingStateMachine,
  createInitialSettlingState,
  decodeQoiRgbaTile,
  decodeQoiTileHeader,
  negotiateDualPlaneStream,
  onMotionInputSample,
  selectViewportTransport,
} from "./dual-plane-client/dual-plane-client.logic";
export type {
  ActiveRenderPlane,
  DecodedRgbaTile,
  MediaCapabilitiesProbeInput,
  NegotiatedDualPlaneSession,
  NegotiatedMotionCodec,
  NegotiatedTileCodec,
  PlaneSettlingState,
  QoiTileHeader,
  TransportCandidateInput,
  ViewportTransportKind,
} from "./dual-plane-client/dual-plane-client.model";
export { projectHudPrimitives, screenPxToWorldNm, worldNmToScreenPx } from "./hud-overlay/hud-overlay.logic";
export type {
  HudOverlayKind,
  HudWorldPrimitive,
  ProjectedHudPrimitive,
  ViewportTransformNm,
} from "./hud-overlay/hud-overlay.model";
export { DualPlaneViewportComponent } from "./viewport/dual-plane-viewport.component";
export { buildViewportReticleVm, type ViewportStateSnapshot } from "./viewport/dual-plane-viewport.logic";
export type { ViewportReticleVm } from "./viewport/dual-plane-viewport.model";
export {
  createVolatileOpBuffer,
  sealCloudClipboardEnvelope,
  validateCloudClipboardPaste,
} from "./volatile-reconnect/volatile-reconnect.logic";
export type {
  CloudClipboardEnvelope,
  CloudClipboardPasteValidation,
  VolatileGraphMutationOp,
  VolatileOpBufferHandle,
} from "./volatile-reconnect/volatile-reconnect.model";
export { WorkbenchStageComponent } from "./workbench-stage/workbench-stage.component";
export type { WorkbenchDescriptor, WorkbenchKpiCard } from "./workbench-stage/workbench-stage.model";
