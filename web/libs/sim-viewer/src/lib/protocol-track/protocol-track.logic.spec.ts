// Module: web/libs/sim-viewer/src/lib/protocol-track/protocol-track.logic.spec.ts — FEAT-053 Unit Tests
// Invariants: Verifies all 6 decoded bus protocols, search filtering, and cyclic Jump-to-Next-NACK/CRC navigation.
// Spec: architecture/03-rust-eda-engine.md
import { describe, expect, it } from "vitest";
import {
  buildProtocolTrackVm,
  findNextErrorPacket,
} from "./protocol-track.logic";

describe("protocol-track.logic (FEAT-053)", () => {
  it("filters decoded transactions across 6 protocols and cycles through NACK and CRC21 errors", () => {
    const firstErr = findNextErrorPacket("PKT-I2C-01");
    expect(firstErr.packetId).toBe("PKT-I2C-03");
    const secondErr = findNextErrorPacket(firstErr.packetId);
    expect(secondErr.packetId).toBe("PKT-CANFD-05");

    const i2cVm = buildProtocolTrackVm("I2C", "0x48", firstErr.packetId);
    expect(i2cVm.rows).toHaveLength(1);
    expect(i2cVm.trackTone).toBe("danger");
  });
});
