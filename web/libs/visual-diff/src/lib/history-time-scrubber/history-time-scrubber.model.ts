// Module: web/libs/visual-diff/src/lib/history-time-scrubber/history-time-scrubber.model.ts — FEAT-097 History Scrubber Models
// Invariants: Super-Commit revisions and UTC timestamps stored strictly as ES2020 bigint.
// Spec: architecture/13-vcs-git-server-and-storage-analysis.md, architecture/17-extended-feature-specifications-ui-and-rust.md
export interface SuperCommitCheckpoint {
  readonly revision: bigint;
  readonly committedAtUtcMs: bigint;
  readonly authorHmac: string;
  readonly merkleRootShort: string;
  readonly summary: string;
  readonly deltaEntityCount: number;
}

export interface HistoryTimeScrubberVm {
  readonly checkpoints: readonly SuperCommitCheckpoint[];
  readonly selectedCheckpoint: SuperCommitCheckpoint;
  readonly onionSkinOpacityPct: number;
  readonly scrubberBadge: string;
}
