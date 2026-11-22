// Module: web/libs/visual-diff/src/lib/history-time-scrubber/history-time-scrubber.logic.ts — FEAT-097 History Scrubber Logic
// Invariants: Deterministic Super-Commit checkpoint lookup and onion-skin blend clamping [0..100].
// Spec: architecture/13-vcs-git-server-and-storage-analysis.md, architecture/17-extended-feature-specifications-ui-and-rust.md
import type { HistoryTimeScrubberVm, SuperCommitCheckpoint } from "./history-time-scrubber.model";

export const DEFAULT_SUPER_COMMITS: readonly SuperCommitCheckpoint[] = [
  { revision: 99n, committedAtUtcMs: 1_800_000_000_000n, authorHmac: "actor_hmac_41a9", merkleRootShort: "b3_99f1c2", summary: "Initial 800V DC-Link schematic", deltaEntityCount: 18 },
  { revision: 100n, committedAtUtcMs: 1_800_003_600_000n, authorHmac: "actor_hmac_41a9", merkleRootShort: "b3_100a7d", summary: "Route Kelvin gate pair +85um", deltaEntityCount: 6 },
  { revision: 101n, committedAtUtcMs: 1_800_007_200_000n, authorHmac: "actor_hmac_88e2", merkleRootShort: "b3_101e4b", summary: "Update heatsink fin pitch 4.5mm", deltaEntityCount: 4 },
];

export function buildHistoryTimeScrubberVm(
  selectedIndex: number,
  onionOpacityPct: number,
  checkpoints: readonly SuperCommitCheckpoint[] = DEFAULT_SUPER_COMMITS,
): HistoryTimeScrubberVm {
  const idx = Math.max(0, Math.min(checkpoints.length - 1, Math.round(selectedIndex)));
  const selectedCheckpoint = checkpoints[idx];
  const clampedOpacity = Math.max(0, Math.min(100, Math.round(onionOpacityPct)));
  return {
    checkpoints,
    selectedCheckpoint,
    onionSkinOpacityPct: clampedOpacity,
    scrubberBadge: `Super-Commit @${selectedCheckpoint.revision.toString(10)} (${selectedCheckpoint.merkleRootShort}) • Onion-Skin ${clampedOpacity}%`,
  };
}
