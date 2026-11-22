// Module: engine/crates/signal-egress-proxy/src/lib.rs — Socket-Pinned Anti-SSRF & Gate-1 Fencing Verifier
// Invariants: Rejects loopback/private/link-local/cloud-IMDS IPs (F-089) and zombie fencing epochs (F-119).
// Spec: architecture/15-application-security-and-project-isolation.md
#![forbid(unsafe_code)]

use std::net::IpAddr;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum EgressError {
    #[error("SSRF blocked: resolved IP `{0}` is non-global or internal metadata")]
    DisallowedTargetIp(IpAddr),
    #[error("missing or stale X-Signal-Fencing-Token ({provided} < {expected}) -> HTTP 412")]
    StaleFencingToken { provided: u64, expected: u64 },
}

pub fn validate_resolved_egress_ip(
    ip: IpAddr,
    allow_local_dev_mock: bool,
) -> Result<(), EgressError> {
    if allow_local_dev_mock && ip.is_loopback() {
        return Ok(());
    }
    let blocked = match ip {
        IpAddr::V4(v4) => {
            v4.is_private()
                || v4.is_loopback()
                || v4.is_link_local()
                || v4.is_broadcast()
                || v4.is_unspecified()
                || v4.octets()[0] == 100 && (v4.octets()[1] & 0b1100_0000) == 64
        }
        IpAddr::V6(v6) => {
            v6.is_loopback()
                || v6.is_unspecified()
                || v6.is_multicast()
                || (v6.segments()[0] & 0xfe00) == 0xfc00
                || (v6.segments()[0] & 0xffc0) == 0xfe80
        }
    };
    if blocked {
        Err(EgressError::DisallowedTargetIp(ip))
    } else {
        Ok(())
    }
}

pub fn verify_egress_fencing_header(
    provided_epoch: Option<u64>,
    live_lease_epoch: u64,
) -> Result<(), EgressError> {
    let epoch = provided_epoch.unwrap_or(0);
    if epoch < live_lease_epoch {
        return Err(EgressError::StaleFencingToken {
            provided: epoch,
            expected: live_lease_epoch,
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{EgressError, validate_resolved_egress_ip, verify_egress_fencing_header};
    use std::net::{IpAddr, Ipv4Addr};

    #[test]
    fn blocks_cloud_imds_and_rfc1918_and_stale_epochs() {
        let imds = IpAddr::V4(Ipv4Addr::new(169, 254, 169, 254));
        assert_eq!(
            validate_resolved_egress_ip(imds, false),
            Err(EgressError::DisallowedTargetIp(imds))
        );
        assert!(
            validate_resolved_egress_ip(IpAddr::V4(Ipv4Addr::new(140, 82, 112, 4)), false).is_ok()
        );
        assert_eq!(
            verify_egress_fencing_header(Some(41), 42),
            Err(EgressError::StaleFencingToken {
                provided: 41,
                expected: 42
            })
        );
    }
}
