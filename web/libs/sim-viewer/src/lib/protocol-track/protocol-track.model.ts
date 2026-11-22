// Module: web/libs/sim-viewer/src/lib/protocol-track/protocol-track.model.ts — FEAT-053 Decoded Bus Protocol Track Models
// Invariants: Transaction start/end spans stay in ES2020 bigint picoseconds (_ps) across all 6 bus protocols.
// Spec: architecture/03-rust-eda-engine.md, contracts/proto/signal/v1/ic_fpga_cosim_twin.proto
import type { SigGridColumn, SigGridRow, SigStatusTone } from "@signal/ui-kit";

export type DecodedBusProtocol = "ALL" | "AXI4" | "SPI" | "I2C" | "CAN-FD" | "USB-PD" | "PCIe";

export interface DecodedProtocolPacket {
  readonly packetId: string;
  readonly busProtocol: Exclude<DecodedBusProtocol, "ALL">;
  readonly startTimePs: bigint;
  readonly endTimePs: bigint;
  readonly decodedSummary: string;
  readonly crcOrAckValid: boolean;
}

export interface ProtocolTrackVm {
  readonly activeProtocol: DecodedBusProtocol;
  readonly protocols: readonly DecodedBusProtocol[];
  readonly searchQuery: string;
  readonly focusedPacket: DecodedProtocolPacket;
  readonly errorPacketCount: number;
  readonly focusBadge: string;
  readonly trackTone: SigStatusTone;
  readonly columns: readonly SigGridColumn[];
  readonly rows: readonly SigGridRow[];
}
