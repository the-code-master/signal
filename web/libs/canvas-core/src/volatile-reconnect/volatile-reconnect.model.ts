// Module: web/libs/canvas-core/src/volatile-reconnect/volatile-reconnect.model.ts — Volatile Closure Ring-Buffer & Cloud Clipboard Models
// Invariants: Zero browser disk persistence; uncommitted ops live <=60s in JS closure memory; clipboard envelopes carry fencingEpoch (F-071, F-073).
// Spec: architecture/06-frontend-angular-and-fastapi-control-plane.md, architecture/07-security-encryption-dlp-and-sftp.md
export interface VolatileGraphMutationOp {
  readonly idempotencyKey: string;
  readonly opId: string;
  readonly kind: string;
  readonly entityId: string;
  readonly enqueuedAtUtcMs: bigint;
  readonly positionXNm?: bigint;
  readonly positionYNm?: bigint;
  readonly paramSiValue?: bigint;
}

export interface VolatileOpBufferHandle {
  enqueue(op: VolatileGraphMutationOp, nowUtcMs: bigint): number;
  acknowledgeThrough(opId: string): number;
  drainReplayBatch(nowUtcMs: bigint): readonly VolatileGraphMutationOp[];
  pendingCount(nowUtcMs: bigint): number;
  wipeAll(): void;
}

export interface CloudClipboardEnvelope {
  readonly envelopeId: string;
  readonly sourceCellId: string;
  readonly actorPseudoId: string;
  readonly fencingEpoch: bigint;
  readonly expiresAtUtcMs: bigint;
  readonly selectedEntityCount: number;
  readonly aeadCiphertextLength: number;
}

export interface CloudClipboardPasteValidation {
  readonly allowed: boolean;
  readonly reason: string;
}
