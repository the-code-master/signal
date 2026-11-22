// Module: engine/crates/signal-egress-proxy/src/lib.rs — Socket-Pinned Anti-SSRF & Gate-1 Linearizable Fencing Verifier
// Invariants: Rejects loopback/RFC1918/IMDS/CGNAT/IPv4-mapped-IPv6/K8s-CIDR IPs (F-089) & stale/expired fencing tokens (F-119).
// Spec: architecture/13-vcs-git-server-and-storage-analysis.md, architecture/15-application-security-and-project-isolation.md
#![forbid(unsafe_code)]

use signal_proto::v1::{VerifyEgressFencingTokenRequest, VerifyEgressFencingTokenResponse};
use std::collections::{BTreeMap, BTreeSet};
use std::net::{IpAddr, Ipv4Addr};
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum EgressError {
    #[error("SSRF blocked: resolved IP `{0}` is non-global or internal metadata")]
    DisallowedTargetIp(IpAddr),
    #[error("SSRF blocked: target host `{0}` is not in outbound BYO-Git allowlist")]
    DisallowedTargetHost(String),
    #[error("SSRF blocked: malformed resolved socket IP `{0}`")]
    MalformedSocketIp(String),
    #[error("missing or stale X-Signal-Fencing-Token ({provided} < {expected}) -> HTTP 412")]
    StaleFencingToken { provided: u64, expected: u64 },
    #[error("expired X-Signal-Fencing-Token lease ({expires_at_ms} <= {now_ms}) -> HTTP 412")]
    ExpiredFencingLease { expires_at_ms: i64, now_ms: i64 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ipv4Cidr {
    pub network: u32,
    pub prefix_len: u8,
}

impl Ipv4Cidr {
    pub const fn new(a: u8, b: u8, c: u8, d: u8, prefix_len: u8) -> Self {
        Self {
            network: u32::from_be_bytes([a, b, c, d]),
            prefix_len,
        }
    }

    pub const fn contains(&self, ip: Ipv4Addr) -> bool {
        if self.prefix_len == 0 {
            return true;
        }
        let mask = if self.prefix_len >= 32 {
            !0_u32
        } else {
            !0_u32 << (32 - self.prefix_len)
        };
        (u32::from_be_bytes(ip.octets()) & mask) == (self.network & mask)
    }
}

fn classify_ipv4_ssrf_code(v4: Ipv4Addr, extra_k8s_cidrs: &[Ipv4Cidr]) -> Option<&'static str> {
    let oct = v4.octets();
    if v4.is_link_local() || (oct[0] == 169 && oct[1] == 254) {
        return Some("SSRF_LINK_LOCAL_BLOCKED");
    }
    if v4.is_loopback() {
        return Some("SSRF_LOOPBACK_BLOCKED");
    }
    if extra_k8s_cidrs.iter().any(|c| c.contains(v4)) {
        return Some("SSRF_K8S_CLUSTER_CIDR_BLOCKED");
    }
    if v4.is_private() {
        return Some("SSRF_RFC1918_PRIVATE_BLOCKED");
    }
    // RFC 6598 CGNAT 100.64.0.0/10, multicast 224.0.0.0/4, broadcast, unspecified, benchmark 198.18/15
    if (oct[0] == 100 && (oct[1] & 0b1100_0000) == 64)
        || v4.is_multicast()
        || v4.is_broadcast()
        || v4.is_unspecified()
        || oct[0] == 0
        || (oct[0] == 198 && (oct[1] == 18 || oct[1] == 19))
        || oct[0] >= 240
    {
        return Some("SSRF_NON_GLOBAL_IPV4_BLOCKED");
    }
    None
}

fn classify_ip_ssrf_code(ip: IpAddr, k8s_cidrs: &[Ipv4Cidr]) -> Option<&'static str> {
    match ip {
        IpAddr::V4(v4) => classify_ipv4_ssrf_code(v4, k8s_cidrs),
        IpAddr::V6(v6) => {
            // Unwrap IPv4-mapped (`::ffff:a.b.c.d`) and IPv4-compatible IPv6 addresses first!
            if let Some(mapped_v4) = v6.to_ipv4_mapped().or_else(|| v6.to_ipv4()) {
                return classify_ipv4_ssrf_code(mapped_v4, k8s_cidrs);
            }
            let seg0 = v6.segments()[0];
            if v6.is_loopback() {
                Some("SSRF_LOOPBACK_BLOCKED")
            } else if (seg0 & 0xffc0) == 0xfe80 {
                Some("SSRF_LINK_LOCAL_BLOCKED")
            } else if v6.is_unspecified()
                || v6.is_multicast()
                || (seg0 & 0xfe00) == 0xfc00
                || (seg0 == 0x2001 && v6.segments()[1] == 0x0db8)
            {
                Some("SSRF_NON_GLOBAL_IPV6_BLOCKED")
            } else {
                None
            }
        }
    }
}

pub fn validate_resolved_egress_ip(
    ip: IpAddr,
    allow_local_dev_mock: bool,
) -> Result<(), EgressError> {
    if allow_local_dev_mock && ip.is_loopback() {
        return Ok(());
    }
    let default_k8s = [
        Ipv4Cidr::new(10, 96, 0, 0, 12),
        Ipv4Cidr::new(10, 244, 0, 0, 16),
    ];
    if classify_ip_ssrf_code(ip, &default_k8s).is_some() {
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

/// Gate 1 Socket-Pinned Anti-SSRF + Linearizable `X-Signal-Fencing-Token` Proxy Verifier (`F-089`, `F-119`).
#[derive(Debug)]
pub struct SocketPinnedEgressGate {
    allowed_hosts: BTreeSet<String>,
    k8s_cluster_cidrs: Vec<Ipv4Cidr>,
    linearizable_epochs: BTreeMap<String, u64>,
}

impl Default for SocketPinnedEgressGate {
    fn default() -> Self {
        let allowed_hosts = BTreeSet::from([
            "github.com".to_string(),
            "api.github.com".to_string(),
            "gitlab.com".to_string(),
            "bitbucket.org".to_string(),
            "api.bitbucket.org".to_string(),
        ]);
        Self {
            allowed_hosts,
            k8s_cluster_cidrs: vec![
                Ipv4Cidr::new(10, 96, 0, 0, 12),
                Ipv4Cidr::new(10, 244, 0, 0, 16),
                Ipv4Cidr::new(100, 64, 0, 0, 10),
            ],
            linearizable_epochs: BTreeMap::new(),
        }
    }
}

impl SocketPinnedEgressGate {
    pub fn allow_enterprise_git_host(&mut self, host: &str) {
        self.allowed_hosts.insert(host.trim().to_ascii_lowercase());
    }

    pub fn advance_project_epoch(&mut self, project_id: &str, new_epoch: u64) -> u64 {
        let entry = self
            .linearizable_epochs
            .entry(project_id.to_string())
            .or_insert(new_epoch);
        if new_epoch > *entry {
            *entry = new_epoch;
        }
        *entry
    }

    /// Evaluates socket-pinned IP + hostname allowlist + linearizable fencing epoch BEFORE outbound `connect()`.
    pub fn verify_outbound_connect(
        &mut self,
        req: &VerifyEgressFencingTokenRequest,
        now_utc_ms: i64,
    ) -> VerifyEgressFencingTokenResponse {
        let Some(fencing) = &req.fencing else {
            return VerifyEgressFencingTokenResponse {
                allowed: false,
                current_linearizable_epoch: 0,
                denial_reason_code: "MISSING_FENCING_TOKEN".to_string(),
            };
        };

        let cur_epoch = *self
            .linearizable_epochs
            .entry(fencing.project_id.clone())
            .or_insert(fencing.fencing_epoch);

        // 1. Validate pre-resolved socket IP FIRST (defeating DNS rebinding TOCTOU to 169.254.169.254 / ::ffff:169.254.169.254)
        let Ok(parsed_ip) = req.resolved_socket_ip.parse::<IpAddr>() else {
            return VerifyEgressFencingTokenResponse {
                allowed: false,
                current_linearizable_epoch: cur_epoch,
                denial_reason_code: "INVALID_RESOLVED_SOCKET_IP".to_string(),
            };
        };
        if let Some(ssrf_code) = classify_ip_ssrf_code(parsed_ip, &self.k8s_cluster_cidrs) {
            return VerifyEgressFencingTokenResponse {
                allowed: false,
                current_linearizable_epoch: cur_epoch,
                denial_reason_code: ssrf_code.to_string(),
            };
        }

        // 2. Validate hostname allowlist
        let host_norm = req.target_host.trim().to_ascii_lowercase();
        if !self.allowed_hosts.contains(&host_norm) {
            return VerifyEgressFencingTokenResponse {
                allowed: false,
                current_linearizable_epoch: cur_epoch,
                denial_reason_code: "HOST_NOT_IN_ALLOWLIST".to_string(),
            };
        }

        // 3. Validate linearizable fencing epoch & lease expiry
        if fencing.fencing_epoch < cur_epoch {
            return VerifyEgressFencingTokenResponse {
                allowed: false,
                current_linearizable_epoch: cur_epoch,
                denial_reason_code: "STALE_FENCING_EPOCH_412".to_string(),
            };
        }
        if fencing.expires_at_utc_ms > 0 && now_utc_ms >= fencing.expires_at_utc_ms {
            return VerifyEgressFencingTokenResponse {
                allowed: false,
                current_linearizable_epoch: cur_epoch,
                denial_reason_code: "EXPIRED_FENCING_LEASE_412".to_string(),
            };
        }

        let updated = self.advance_project_epoch(&fencing.project_id, fencing.fencing_epoch);
        VerifyEgressFencingTokenResponse {
            allowed: true,
            current_linearizable_epoch: updated,
            denial_reason_code: String::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use signal_proto::v1::FencingAuthority;
    use std::net::Ipv6Addr;

    #[test]
    fn blocks_imds_rfc1918_ipv4_mapped_ipv6_cgnat_k8s_and_zombie_epochs() {
        let imds = IpAddr::V4(Ipv4Addr::new(169, 254, 169, 254));
        assert_eq!(
            validate_resolved_egress_ip(imds, false),
            Err(EgressError::DisallowedTargetIp(imds))
        );
        // IPv4-mapped IPv6 IMDS `::ffff:169.254.169.254` MUST also be blocked!
        let mapped_imds = IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0xffff, 0xa9fe, 0xa9fe));
        assert_eq!(
            validate_resolved_egress_ip(mapped_imds, false),
            Err(EgressError::DisallowedTargetIp(mapped_imds))
        );
        assert!(
            validate_resolved_egress_ip(IpAddr::V4(Ipv4Addr::new(140, 82, 112, 6)), false).is_ok()
        );
        assert_eq!(
            verify_egress_fencing_header(Some(41), 42),
            Err(EgressError::StaleFencingToken {
                provided: 41,
                expected: 42
            })
        );

        let mut gate = SocketPinnedEgressGate::default();
        let fence = FencingAuthority {
            org_id: "org_1".into(),
            project_id: "proj_1".into(),
            fencing_epoch: 9_007_199_254_740_993,
            lease_holder_pod_id: "pod_1".into(),
            issued_at_utc_ms: 1_000,
            expires_at_utc_ms: 15_000,
        };
        let ok = gate.verify_outbound_connect(
            &VerifyEgressFencingTokenRequest {
                fencing: Some(fence.clone()),
                target_host: "api.github.com".into(),
                resolved_socket_ip: "140.82.112.6".into(),
                provider_repo_id: "gh_1".into(),
            },
            5_000,
        );
        assert!(ok.allowed);

        let ssrf = gate.verify_outbound_connect(
            &VerifyEgressFencingTokenRequest {
                fencing: Some(fence.clone()),
                target_host: "api.github.com".into(),
                resolved_socket_ip: "169.254.169.254".into(),
                provider_repo_id: "gh_1".into(),
            },
            5_000,
        );
        assert!(!ssrf.allowed);
        assert_eq!(ssrf.denial_reason_code, "SSRF_LINK_LOCAL_BLOCKED");

        // Advance epoch to 9_007_199_254_740_994 -> zombie pod at ...993 rejected!
        gate.advance_project_epoch("proj_1", 9_007_199_254_740_994);
        let zombie = gate.verify_outbound_connect(
            &VerifyEgressFencingTokenRequest {
                fencing: Some(fence),
                target_host: "api.github.com".into(),
                resolved_socket_ip: "140.82.112.6".into(),
                provider_repo_id: "gh_1".into(),
            },
            5_000,
        );
        assert!(!zombie.allowed);
        assert_eq!(zombie.denial_reason_code, "STALE_FENCING_EPOCH_412");
    }
}
