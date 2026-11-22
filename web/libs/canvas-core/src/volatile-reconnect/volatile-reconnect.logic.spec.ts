// Module: web/libs/canvas-core/src/volatile-reconnect/volatile-reconnect.logic.spec.ts — Volatile Buffer & Cloud Clipboard Tests
// Invariants: Verifies 60s TTL eviction, idempotency deduplication, ack pruning, wipeAll, and *.sigclip RBAC/TTL checks.
// Spec: architecture/07-security-encryption-dlp-and-sftp.md
import { describe, expect, it } from "vitest";
import {
  createVolatileOpBuffer,
  sealCloudClipboardEnvelope,
  validateCloudClipboardPaste,
} from "./volatile-reconnect.logic";

describe("createVolatileOpBuffer & Pod-to-Pod Cloud Clipboard", () => {
  it("deduplicates by idempotencyKey, evicts >60s ops, and prunes on ack", () => {
    const buf = createVolatileOpBuffer(60_000n, 4);
    buf.enqueue(
      { idempotencyKey: "idem-1", opId: "op-1", kind: "PLACE_INSTANCE", entityId: "U1", enqueuedAtUtcMs: 1_000n },
      1_000n,
    );
    buf.enqueue(
      { idempotencyKey: "idem-1", opId: "op-1", kind: "PLACE_INSTANCE", entityId: "U1", enqueuedAtUtcMs: 1_200n },
      1_200n,
    );
    buf.enqueue(
      { idempotencyKey: "idem-2", opId: "op-2", kind: "CONNECT_NET", entityId: "HV_BUS", enqueuedAtUtcMs: 50_000n },
      50_000n,
    );
    expect(buf.pendingCount(50_000n)).toBe(2);

    // At 62_000ms, idem-1 (enqueued 1_200ms) is >60_000ms old and evicted
    const replay = buf.drainReplayBatch(62_000n);
    expect(replay.map((r) => r.opId)).toEqual(["op-2"]);

    expect(buf.acknowledgeThrough("op-2")).toBe(0);
    buf.wipeAll();
    expect(buf.pendingCount(62_000n)).toBe(0);
  });

  it("seals *.sigclip AEAD envelopes and enforces RBAC, TTL, and fencing epoch", () => {
    const env = sealCloudClipboardEnvelope("cell-aurora-01", "usr_pseudo_7f", 14n, ["U1", "C4", "R9"], 10_000n, 60_000n);
    expect(validateCloudClipboardPaste(env, 14n, 40_000n, true).allowed).toBe(true);
    expect(validateCloudClipboardPaste(env, 14n, 80_000n, true).allowed).toBe(false);
    expect(validateCloudClipboardPaste(env, 12n, 40_000n, true).allowed).toBe(false);
    expect(validateCloudClipboardPaste(env, 14n, 40_000n, false).allowed).toBe(false);
  });
});
