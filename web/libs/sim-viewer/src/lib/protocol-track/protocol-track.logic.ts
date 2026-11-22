// Module: web/libs/sim-viewer/src/lib/protocol-track/protocol-track.logic.ts — FEAT-053 Decoded Bus Transaction Logic
// Invariants: Filters AXI4/SPI/I2C/CAN-FD/USB-PD/PCIe pills in bigint _ps and jumps deterministically to NACK/CRC errors.
// Spec: architecture/03-rust-eda-engine.md, contracts/proto/signal/v1/ic_fpga_cosim_twin.proto
import type { SigGridColumn, SigGridRow, SigStatusTone } from "@signal/ui-kit";
import type {
  DecodedBusProtocol,
  DecodedProtocolPacket,
  ProtocolTrackVm,
} from "./protocol-track.model";

export const DEFAULT_PROTOCOL_PACKETS: readonly DecodedProtocolPacket[] = [
  {
    packetId: "PKT-I2C-01",
    busProtocol: "I2C",
    startTimePs: 12_400_000n,
    endTimePs: 98_200_000n,
    decodedSummary: "START | ADDR 0x48 (TMP117) | REG 0x00 | DATA 0x1980 | ACK",
    crcOrAckValid: true,
  },
  {
    packetId: "PKT-SPI-02",
    busProtocol: "SPI",
    startTimePs: 110_000_000n,
    endTimePs: 118_500_000n,
    decodedSummary: "CS_N | CMD 0xEB (QSPI Fast Read) | ADDR 0x001000 | DATA 0xDEADBEEF | CRC OK",
    crcOrAckValid: true,
  },
  {
    packetId: "PKT-I2C-03",
    busProtocol: "I2C",
    startTimePs: 145_000_000n,
    endTimePs: 192_000_000n,
    decodedSummary: "START | ADDR 0x60 (PMIC) | DATA 0xFF | NACK (Fault Lockout)",
    crcOrAckValid: false,
  },
  {
    packetId: "PKT-AXI4-04",
    busProtocol: "AXI4",
    startTimePs: 205_000_000n,
    endTimePs: 205_850_000n,
    decodedSummary: "AWADDR 0x40010000 | LEN=4 | WDATA 0x00001F40 | BRESP OKAY",
    crcOrAckValid: true,
  },
  {
    packetId: "PKT-CANFD-05",
    busProtocol: "CAN-FD",
    startTimePs: 240_000_000n,
    endTimePs: 312_000_000n,
    decodedSummary: "SOF | ID 0x18FF50E5 (BRS=1) | DLC=16 | CRC21_MISMATCH",
    crcOrAckValid: false,
  },
  {
    packetId: "PKT-USBPD-06",
    busProtocol: "USB-PD",
    startTimePs: 350_000_000n,
    endTimePs: 490_000_000n,
    decodedSummary: "SOP | EPR_REQUEST 28V @ 5A | GOODCRC | PS_RDY",
    crcOrAckValid: true,
  },
  {
    packetId: "PKT-PCIE-07",
    busProtocol: "PCIe",
    startTimePs: 510_000_000n,
    endTimePs: 510_420_000n,
    decodedSummary: "TLP MRd64 @ 0x0000_8000_1000 | Tag=0x07 | LCRC OK",
    crcOrAckValid: true,
  },
];

const PACKET_COLUMNS: readonly SigGridColumn[] = [
  { key: "proto", header: "Bus (FEAT-053)", mono: true },
  { key: "spanPs", header: "Window [ps]", mono: true },
  { key: "pill", header: "Decoded Transaction Pill", mono: true },
  { key: "integrity", header: "ACK / CRC", mono: true },
];

export function findNextErrorPacket(
  currentPacketId: string,
  packets: readonly DecodedProtocolPacket[] = DEFAULT_PROTOCOL_PACKETS,
): DecodedProtocolPacket {
  const errors = packets.filter((p) => !p.crcOrAckValid);
  if (errors.length === 0) return packets[0];
  const curIdx = errors.findIndex((p) => p.packetId === currentPacketId);
  const nextIdx = (curIdx + 1) % errors.length;
  return errors[nextIdx];
}

export function buildProtocolTrackVm(
  activeProtocol: DecodedBusProtocol,
  searchQuery: string,
  focusedPacketId: string,
  packets: readonly DecodedProtocolPacket[] = DEFAULT_PROTOCOL_PACKETS,
): ProtocolTrackVm {
  const needle = searchQuery.trim().toLowerCase();
  const visible = packets.filter((p) => {
    const protoMatch = activeProtocol === "ALL" || p.busProtocol === activeProtocol;
    const queryMatch =
      needle.length === 0 ||
      p.decodedSummary.toLowerCase().includes(needle) ||
      p.busProtocol.toLowerCase().includes(needle);
    return protoMatch && queryMatch;
  });

  const errorPacketCount = packets.filter((p) => !p.crcOrAckValid).length;
  const focusedPacket =
    packets.find((p) => p.packetId === focusedPacketId) ?? packets[0];

  const trackTone: SigStatusTone = focusedPacket.crcOrAckValid ? "ok" : "danger";
  const focusBadge = `Cursor -> ${focusedPacket.packetId} (${focusedPacket.busProtocol} @ ${focusedPacket.startTimePs.toString(10)} ps) • ${errorPacketCount} NACK/CRC Errors`;

  const rows: SigGridRow[] = visible.map((p) => ({
    id: p.packetId,
    proto: p.busProtocol,
    spanPs: `${p.startTimePs.toString(10)}..${p.endTimePs.toString(10)}`,
    pill: p.decodedSummary,
    integrity: p.crcOrAckValid ? "VALID_ACK_CRC" : "NACK_OR_CRC_ERR",
  }));

  return {
    activeProtocol,
    protocols: ["ALL", "AXI4", "SPI", "I2C", "CAN-FD", "USB-PD", "PCIe"],
    searchQuery,
    focusedPacket,
    errorPacketCount,
    focusBadge,
    trackTone,
    columns: PACKET_COLUMNS,
    rows,
  };
}
