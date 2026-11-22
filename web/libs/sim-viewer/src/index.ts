// Module: web/libs/sim-viewer/src/index.ts — Public Barrel for @signal/sim-viewer
// Invariants: Sole public entrypoint for lazy route loading and CU-P1-03 + CU-P1-04 simulation studios.
// Spec: docs/engineering/ANGULAR_ENGINEERING_GUIDE.md
export { WaveformFormulaBarComponent } from "./lib/formula-bar/formula-bar.component";
export {
  buildWaveformFormulaBarVm,
  evaluateFormulaOnSampleVectors,
  suggestFormulaCompletions,
  tokenizeWaveformFormula,
} from "./lib/formula-bar/formula-bar.logic";
export type {
  FormulaToken,
  FormulaTokenizeResult,
  FormulaTokenKind,
  WaveformFormulaBarVm,
} from "./lib/formula-bar/formula-bar.model";
export { JitterStudioComponent } from "./lib/jitter-studio/jitter-studio.component";
export {
  buildJitterStudioVm,
  computeDualDiracTjAt1e15Fs,
  presetJitterInput,
} from "./lib/jitter-studio/jitter-studio.logic";
export type {
  JitterDecompositionInput,
  JitterStudioVm,
  SerDesRatePreset,
} from "./lib/jitter-studio/jitter-studio.model";
export { MonteCarloStudioComponent } from "./lib/monte-carlo/monte-carlo.component";
export {
  buildHistogramBins,
  buildMonteCarloStudioVm,
  computeSpearmanRankCorrelation,
  computeYieldAndCpk,
} from "./lib/monte-carlo/monte-carlo.logic";
export type {
  HistogramBin,
  MonteCarloStudioVm,
  MonteCarloSummary,
  TornadoSensitivityRow,
} from "./lib/monte-carlo/monte-carlo.model";
export { RegressionSuiteComponent } from "./lib/regression-suite/regression-suite.component";
export { buildRegressionSuiteVm, evaluateSpecAssertion } from "./lib/regression-suite/regression-suite.logic";
export type { EvaluatedSpecAssertion, RegressionSuiteVm } from "./lib/regression-suite/regression-suite.model";
export { SimViewerWorkbenchComponent } from "./lib/sim-viewer-workbench/sim-viewer-workbench.component";
export { buildSimViewerWorkbenchVm } from "./lib/sim-viewer-workbench/sim-viewer-workbench.logic";
export type { SimStudioTabId, SimViewerWorkbenchVm } from "./lib/sim-viewer-workbench/sim-viewer-workbench.model";
export { StimulusAuditionComponent } from "./lib/stimulus-audition/stimulus-audition.component";
export { buildStimulusAuditionVm, synthesizeVolatilePcmPair } from "./lib/stimulus-audition/stimulus-audition.logic";
export type {
  AuditionChannelAb,
  StimulusAuditionVm,
  StimulusPresetKind,
  VolatilePcmPair,
} from "./lib/stimulus-audition/stimulus-audition.model";
export { VirtualBenchDeckComponent } from "./lib/virtual-bench/virtual-bench.component";
export {
  buildVirtualBenchVm,
  computeEnobFromSinadDb,
  gammaToSmithImpedance,
} from "./lib/virtual-bench/virtual-bench.logic";
export type {
  BenchInstrumentMode,
  SmithImpedancePoint,
  VirtualBenchVm,
} from "./lib/virtual-bench/virtual-bench.model";
export { WaveformCanvasComponent } from "./lib/waveform-canvas/waveform-canvas.component";
export {
  buildWaveformCanvasVm,
  computeDualCursorReadout,
  encodeArrowWaveformFrame,
  parseDecimatedArrowFrame,
} from "./lib/waveform-canvas/waveform-canvas.logic";
export type {
  DecimatedArrowBucket,
  ParsedArrowWaveformFrame,
  WaveformCanvasVm,
  WaveformCursorReadout,
} from "./lib/waveform-canvas/waveform-canvas.model";
export { AnomalyDebuggerComponent } from "./lib/anomaly-debugger/anomaly-debugger.component";
export { DEFAULT_ANOMALY_CASES, buildAnomalyDebuggerVm } from "./lib/anomaly-debugger/anomaly-debugger.logic";
export type {
  AnomalyAttributionCase,
  AnomalyDebuggerVm,
  AnomalySymptomId,
} from "./lib/anomaly-debugger/anomaly-debugger.model";
export { BreakingPointComponent } from "./lib/breaking-point/breaking-point.component";
export { DEFAULT_STRESS_ENVELOPES, buildBreakingPointVm } from "./lib/breaking-point/breaking-point.logic";
export type { BreakingPointVm, ComponentStressEnvelope } from "./lib/breaking-point/breaking-point.model";
export { ParetoFrontierComponent } from "./lib/pareto-frontier/pareto-frontier.component";
export {
  RAW_PARETO_POPULATION,
  buildParetoFrontierVm,
  filterNonDominatedParetoCandidates,
} from "./lib/pareto-frontier/pareto-frontier.logic";
export type {
  ParetoCandidatePoint,
  ParetoFrontierVm,
  ParetoPriorityGoal,
} from "./lib/pareto-frontier/pareto-frontier.model";
export { ProtocolTrackComponent } from "./lib/protocol-track/protocol-track.component";
export {
  DEFAULT_PROTOCOL_PACKETS,
  buildProtocolTrackVm,
  findNextErrorPacket,
} from "./lib/protocol-track/protocol-track.logic";
export type {
  DecodedBusProtocol,
  DecodedProtocolPacket,
  ProtocolTrackVm,
} from "./lib/protocol-track/protocol-track.model";
export { WhatIfScrubberComponent } from "./lib/what-if-scrubber/what-if-scrubber.component";
export { buildWhatIfScrubberVm } from "./lib/what-if-scrubber/what-if-scrubber.logic";
export type { GhostFamilyCurve, WhatIfScrubberVm } from "./lib/what-if-scrubber/what-if-scrubber.model";
export { WORKBENCH_DESCRIPTOR, WORKBENCH_ROUTES } from "./sim-viewer.routes";
