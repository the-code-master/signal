// Module: web/libs/bringup-lab/src/lib/boot-hil-boardmap/boot-hil-boardmap.model.ts — FEAT-058/059/070 Bring-Up Models
// Invariants: Boot timestamps (_ps), HIL recovery times (_ps), and board-map nominal V/Z/f (_uv/_mohm/_hz) stay in bigint.
// Spec: architecture/03-rust-eda-engine.md, contracts/proto/signal/v1/ic_fpga_cosim_twin.proto
import type { SigGridColumn, SigGridRow, SigStatusTone } from "@signal/ui-kit";

export type HilFaultScenario = "VDD_BROWNOUT_DIP" | "CANFD_CRC_STORM" | "RESOLVER_WIRE_OPEN";

export interface BoardTestpointLookup {
  readonly testpointId: string;
  readonly netName: string;
  readonly nominalVoltageUv: bigint;
  readonly nominalImpedanceMohm: bigint;
  readonly nominalFreqHz: bigint;
}

export interface BootHilBoardmapVm {
  readonly hilScenario: HilFaultScenario;
  readonly hilScenarios: readonly HilFaultScenario[];
  readonly selectedTp: BoardTestpointLookup;
  readonly testpoints: readonly BoardTestpointLookup[];
  readonly coldBootPostPassed: boolean;
  readonly hilRecoveryLatencyPs: bigint;
  readonly bootHilBadge: string;
  readonly bootHilTone: SigStatusTone;
  readonly boardMapBadge: string;
  readonly columns: readonly SigGridColumn[];
  readonly rows: readonly SigGridRow[];
}
