// Module: engine/crates/signal-git-sync/src/history_scrubber.rs — Commit-Graph Delta Walker (FEAT-097)
// Invariants: Deterministic reverse-chronological Super-Commit delta counting for visual history scrubbing.
// Spec: architecture/13-vcs-git-server-and-storage-analysis.md, architecture/17-extended-feature-specifications-ui-and-rust.md
use signal_proto::v1::{
    HistoryTimelineEntry, ScrubHistoryTimelineRequest, ScrubHistoryTimelineResponse,
};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone)]
pub struct CommitGraphSnapshot {
    pub super_commit_id: String,
    pub summary: String,
    pub committed_at_utc_ms: i64,
    pub entity_hashes: BTreeMap<String, String>,
}

/// Walks a project's `Super-Commit` history graph and computes exact entity delta counts per commit (`FEAT-097`).
pub fn scrub_history_timeline(
    req: &ScrubHistoryTimelineRequest,
    chronological_commits: &[CommitGraphSnapshot],
) -> ScrubHistoryTimelineResponse {
    let limit = if req.max_commits == 0 {
        50
    } else {
        req.max_commits.min(500) as usize
    };
    let mut entries = Vec::with_capacity(chronological_commits.len().min(limit));

    for (idx, commit) in chronological_commits.iter().enumerate() {
        let changed = if idx == 0 {
            commit.entity_hashes.len() as u32
        } else {
            let prev = &chronological_commits[idx - 1].entity_hashes;
            let mut all_keys = BTreeSet::new();
            all_keys.extend(prev.keys());
            all_keys.extend(commit.entity_hashes.keys());
            all_keys
                .into_iter()
                .filter(|k| prev.get(*k) != commit.entity_hashes.get(*k))
                .count() as u32
        };

        entries.push(HistoryTimelineEntry {
            super_commit_id: commit.super_commit_id.clone(),
            summary: commit.summary.clone(),
            committed_at_utc_ms: commit.committed_at_utc_ms,
            changed_entity_count: changed,
        });
    }

    entries.reverse();
    entries.truncate(limit);
    ScrubHistoryTimelineResponse { entries }
}
