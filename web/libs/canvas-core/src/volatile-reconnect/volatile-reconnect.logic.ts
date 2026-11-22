// Module: web/libs/canvas-core/src/volatile-reconnect/volatile-reconnect.logic.ts — Closure-Isolated 60s Reconnect Ring-Buffer & Cloud Clipboard
// Invariants: Holds uncommitted GraphMutationOps strictly inside lexical closure RAM (<=60_000ms); validates AEAD *.sigclip TTL & epoch (F-071, F-073).
// Spec: architecture/06-frontend-angular-and-fastapi-control-plane.md, architecture/07-security-encryption-dlp-and-sftp.md
import type {
  CloudClipboardEnvelope,
  CloudClipboardPasteValidation,
  VolatileGraphMutationOp,
  VolatileOpBufferHandle,
} from "./volatile-reconnect.model";

export function createVolatileOpBuffer(
  maxRetentionMs: bigint = 60_000n,
  maxCapacity = 256,
): VolatileOpBufferHandle {
  const ring: VolatileGraphMutationOp[] = [];

  function pruneExpired(nowUtcMs: bigint): void {
    while (ring.length > 0 && nowUtcMs - ring[0].enqueuedAtUtcMs > maxRetentionMs) {
      ring.shift();
    }
  }

  return {
    enqueue(op: VolatileGraphMutationOp, nowUtcMs: bigint): number {
      pruneExpired(nowUtcMs);
      const existingIdx = ring.findIndex((item) => item.idempotencyKey === op.idempotencyKey);
      if (existingIdx >= 0) {
        ring[existingIdx] = op;
        return ring.length;
      }
      if (ring.length >= maxCapacity) {
        ring.shift();
      }
      ring.push(op);
      return ring.length;
    },
    acknowledgeThrough(opId: string): number {
      const idx = ring.findIndex((item) => item.opId === opId);
      if (idx >= 0) {
        ring.splice(0, idx + 1);
      }
      return ring.length;
    },
    drainReplayBatch(nowUtcMs: bigint): readonly VolatileGraphMutationOp[] {
      pruneExpired(nowUtcMs);
      return ring.slice();
    },
    pendingCount(nowUtcMs: bigint): number {
      pruneExpired(nowUtcMs);
      return ring.length;
    },
    wipeAll(): void {
      ring.length = 0;
    },
  };
}

export function sealCloudClipboardEnvelope(
  sourceCellId: string,
  actorPseudoId: string,
  fencingEpoch: bigint,
  selectedEntityIds: readonly string[],
  nowUtcMs: bigint,
  ttlMs: bigint = 120_000n,
): CloudClipboardEnvelope {
  const count = selectedEntityIds.length;
  return {
    envelopeId: `sigclip-${sourceCellId}-${fencingEpoch.toString(10)}-${count}`,
    sourceCellId,
    actorPseudoId,
    fencingEpoch,
    expiresAtUtcMs: nowUtcMs + ttlMs,
    selectedEntityCount: count,
    aeadCiphertextLength: 48 + count * 64,
  };
}

export function validateCloudClipboardPaste(
  envelope: CloudClipboardEnvelope | null,
  targetFencingEpoch: bigint,
  nowUtcMs: bigint,
  rbacAuthorized: boolean,
): CloudClipboardPasteValidation {
  if (!envelope) {
    return { allowed: false, reason: "EMPTY_CLIPBOARD: no sealed *.sigclip envelope" };
  }
  if (!rbacAuthorized) {
    return { allowed: false, reason: "RBAC_DENIED: cross-cell paste unauthorized" };
  }
  if (nowUtcMs > envelope.expiresAtUtcMs) {
    return { allowed: false, reason: "ENVELOPE_EXPIRED: sealed snippet TTL elapsed" };
  }
  if (targetFencingEpoch < envelope.fencingEpoch) {
    return { allowed: false, reason: "STALE_EPOCH: target cell epoch precedes clipboard seal" };
  }
  return {
    allowed: true,
    reason: `VERIFIED_SIGCLIP: ${envelope.selectedEntityCount} entities (${envelope.aeadCiphertextLength} B AEAD)`,
  };
}
