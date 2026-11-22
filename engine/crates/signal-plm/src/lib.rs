// Module: engine/crates/signal-plm/src/lib.rs — Deviation/Waiver Clear-to-Build & SFTP Egress Gate
// Invariants: Blocks factory release when requested_release_qty exceeds remaining authorized_qty_limit or expiry (19-*).
// Spec: architecture/19-cad-cam-3d-printing-and-plm-operations.md
#![forbid(unsafe_code)]

use signal_parts as _;
use signal_proto::v1::{
    DeviationWaiverGate, VerifyDeviationGateRequest, VerifyDeviationGateResponse,
};

pub fn evaluate_deviation_gates(
    gates: &[DeviationWaiverGate],
    req: &VerifyDeviationGateRequest,
) -> VerifyDeviationGateResponse {
    let mut blocking = Vec::new();
    for g in gates {
        let next_consumed = g.consumed_qty.saturating_add(req.requested_release_qty);
        if req.evaluation_time_utc_ms > g.expires_at_utc_ms
            || next_consumed > g.authorized_qty_limit
        {
            blocking.push(g.deviation_id.clone());
        }
    }
    VerifyDeviationGateResponse {
        release_permitted: blocking.is_empty(),
        blocking_deviation_ids: blocking,
    }
}

#[cfg(test)]
mod tests {
    use super::evaluate_deviation_gates;
    use signal_proto::v1::{DeviationWaiverGate, VerifyDeviationGateRequest};

    #[test]
    fn blocks_expired_or_over_quota_deviations() {
        let gate = DeviationWaiverGate {
            deviation_id: "DEV-2026-004".to_string(),
            authorized_qty_limit: 500,
            consumed_qty: 450,
            expires_at_utc_ms: 1_800_000_000_000,
            ccb_approval_sig: None,
        };
        let ok_req = VerifyDeviationGateRequest {
            requested_release_qty: 50,
            evaluation_time_utc_ms: 1_795_000_000_000,
            ..Default::default()
        };
        assert!(evaluate_deviation_gates(std::slice::from_ref(&gate), &ok_req).release_permitted);

        let over_req = VerifyDeviationGateRequest {
            requested_release_qty: 51,
            evaluation_time_utc_ms: 1_795_000_000_000,
            ..Default::default()
        };
        let denied = evaluate_deviation_gates(&[gate], &over_req);
        assert!(!denied.release_permitted);
        assert_eq!(denied.blocking_deviation_ids, vec!["DEV-2026-004"]);
    }
}
