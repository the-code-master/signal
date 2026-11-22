// Module: engine/crates/signal-git-sync/src/portfolio_dag.rs — Cross-Project Portfolio Blast-Radius Dependency Graph (FEAT-118)
// Invariants: Propagates MPN EOL / shared-IP CVE impact transitively across .signal/repos.lock.json & parts.lock.json DAGs.
// Spec: architecture/14-universal-scale-accessibility-and-expandability.md, architecture/17-extended-feature-specifications-ui-and-rust.md
use signal_proto::v1::{AnalyzePortfolioBlastRadiusRequest, AnalyzePortfolioBlastRadiusResponse};
use std::collections::{BTreeMap, BTreeSet, VecDeque};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PortfolioProjectNode {
    pub org_id: String,
    pub project_id: String,
    pub annual_revenue_micros_usd: i64,
    pub locked_mpns_and_ips: Vec<String>,
    pub upstream_project_dependencies: Vec<String>,
}

/// Propagates transitive blast-radius & revenue-at-risk across an organization's multi-project portfolio DAG (`FEAT-118`).
pub fn analyze_portfolio_blast_radius(
    req: &AnalyzePortfolioBlastRadiusRequest,
    portfolio: &[PortfolioProjectNode],
) -> AnalyzePortfolioBlastRadiusResponse {
    let mut rev_by_proj: BTreeMap<&str, i64> = BTreeMap::new();
    let mut downstream_dependents: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    let mut queue: VecDeque<(&str, u32)> = VecDeque::new();
    let mut visited: BTreeSet<&str> = BTreeSet::new();

    for node in portfolio.iter().filter(|n| n.org_id == req.org_id) {
        rev_by_proj.insert(
            node.project_id.as_str(),
            node.annual_revenue_micros_usd.max(0),
        );
        for up in &node.upstream_project_dependencies {
            downstream_dependents
                .entry(up.as_str())
                .or_default()
                .push(node.project_id.as_str());
        }
        if node
            .locked_mpns_and_ips
            .iter()
            .any(|id| id == &req.affected_mpn_or_ip_id)
            && visited.insert(node.project_id.as_str())
        {
            queue.push_back((node.project_id.as_str(), 1));
        }
    }

    let mut max_depth = 0_u32;
    while let Some((curr_proj, depth)) = queue.pop_front() {
        max_depth = max_depth.max(depth);
        if let Some(children) = downstream_dependents.get(curr_proj) {
            for &child in children {
                if visited.insert(child) {
                    queue.push_back((child, depth + 1));
                }
            }
        }
    }

    let mut impacted: Vec<String> = visited.iter().map(|s| (*s).to_string()).collect();
    impacted.sort();
    let total_rev = visited.iter().fold(0_i64, |acc, p| {
        acc.saturating_add(*rev_by_proj.get(p).unwrap_or(&0))
    });

    AnalyzePortfolioBlastRadiusResponse {
        impacted_project_ids: impacted,
        transitive_depth: max_depth,
        estimated_revenue_at_risk_micros_usd: total_rev,
    }
}
