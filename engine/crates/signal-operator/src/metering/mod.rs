// Module: engine/crates/signal-operator/src/metering/mod.rs — Per-Pod/Org FinOps Metering & Quota Circuit-Breaker (FEAT-116, F-098)
// Invariants: Accrues exact integer micros USD for CPU-ms, GPU-ms & AI tokens; trips 80% soft-warn & 100% hard-cap circuit-breakers.
// Spec: architecture/08-identity-enterprise-and-project-model.md, architecture/17-extended-feature-specifications-ui-and-rust.md
use signal_proto::v1::{QueryEnterpriseFinOpsRequest, QueryEnterpriseFinOpsResponse};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PodComputeMeterSample {
    pub org_id: String,
    pub project_id: String,
    pub recorded_at_utc_ms: i64,
    pub cpu_core_ms: i64,
    pub gpu_ms: i64,
    pub ai_tokens: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FinOpsRateCard {
    pub cpu_core_hour_micros_usd: i64,
    pub gpu_hour_micros_usd: i64,
    pub million_ai_tokens_micros_usd: i64,
    pub org_hard_cap_micros_usd: i64,
}

impl Default for FinOpsRateCard {
    fn default() -> Self {
        Self {
            cpu_core_hour_micros_usd: 45_000,        // $0.045 / CPU-core-hour
            gpu_hour_micros_usd: 3_600_000,          // $3.60 / GPU-hour ($0.001 / GPU-sec)
            million_ai_tokens_micros_usd: 2_500_000, // $2.50 / 1M tokens
            org_hard_cap_micros_usd: 5_000_000_000,  // $5,000.00 monthly cap
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FinOpsEvaluationOutcome {
    pub response: QueryEnterpriseFinOpsResponse,
    pub soft_warn_80pct_tripped: bool,
}

/// Aggregates CPU-core-ms, GPU-ms & AI tokens within `[window_start_utc_ms, window_end_utc_ms]` and evaluates FinOps caps (`FEAT-116`).
pub fn evaluate_enterprise_finops_window(
    req: &QueryEnterpriseFinOpsRequest,
    samples: &[PodComputeMeterSample],
    rates: FinOpsRateCard,
) -> FinOpsEvaluationOutcome {
    let mut cpu_ms: i64 = 0;
    let mut gpu_ms: i64 = 0;
    let mut tokens: i64 = 0;

    for s in samples {
        if s.org_id == req.org_id
            && s.recorded_at_utc_ms >= req.window_start_utc_ms
            && s.recorded_at_utc_ms <= req.window_end_utc_ms
        {
            cpu_ms = cpu_ms.saturating_add(s.cpu_core_ms.max(0));
            gpu_ms = gpu_ms.saturating_add(s.gpu_ms.max(0));
            tokens = tokens.saturating_add(s.ai_tokens.max(0));
        }
    }

    let ms_per_hour: i128 = 3_600_000;
    let cpu_spend = ((i128::from(cpu_ms) * i128::from(rates.cpu_core_hour_micros_usd))
        + ms_per_hour / 2)
        / ms_per_hour;
    let gpu_spend = ((i128::from(gpu_ms) * i128::from(rates.gpu_hour_micros_usd))
        + ms_per_hour / 2)
        / ms_per_hour;
    let ai_spend = ((i128::from(tokens) * i128::from(rates.million_ai_tokens_micros_usd))
        + 500_000)
        / 1_000_000;

    let accrued_micros = (cpu_spend + gpu_spend + ai_spend).clamp(0, i128::from(i64::MAX)) as i64;
    let cap = rates.org_hard_cap_micros_usd.max(1);
    let soft_warn = i128::from(accrued_micros) * 100 >= i128::from(cap) * 80;
    let hard_tripped = accrued_micros >= cap;

    FinOpsEvaluationOutcome {
        response: QueryEnterpriseFinOpsResponse {
            cpu_core_ms_consumed: cpu_ms,
            gpu_ms_consumed: gpu_ms,
            ai_tokens_consumed: tokens,
            accrued_spend_micros_usd: accrued_micros,
            hard_cap_micros_usd: cap,
            hard_budget_cap_tripped: hard_tripped,
        },
        soft_warn_80pct_tripped: soft_warn,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accrues_cpu_gpu_ai_spend_and_trips_soft_80pct_and_hard_100pct_caps() {
        let samples = [PodComputeMeterSample {
            org_id: "org-acme".into(),
            project_id: "proj-radar".into(),
            recorded_at_utc_ms: 1_500,
            cpu_core_ms: 36_000_000, // 10 CPU-hours -> $0.45 (450_000 micros)
            gpu_ms: 3_600_000,       // 1 GPU-hour   -> $3.60 (3_600_000 micros)
            ai_tokens: 1_000_000,    // 1M tokens    -> $2.50 (2_500_000 micros) => total $6.55
        }];
        let mut rates = FinOpsRateCard {
            org_hard_cap_micros_usd: 7_000_000, // $7.00 cap -> $6.55 is 93.6% (soft warn ON, hard OFF)
            ..Default::default()
        };

        let req = QueryEnterpriseFinOpsRequest {
            org_id: "org-acme".into(),
            window_start_utc_ms: 1_000,
            window_end_utc_ms: 2_000,
        };
        let warn_out = evaluate_enterprise_finops_window(&req, &samples, rates);
        assert_eq!(warn_out.response.accrued_spend_micros_usd, 6_550_000);
        assert!(warn_out.soft_warn_80pct_tripped);
        assert!(!warn_out.response.hard_budget_cap_tripped);

        rates.org_hard_cap_micros_usd = 6_000_000; // $6.00 cap -> hard cap TRIPPED!
        let hard_out = evaluate_enterprise_finops_window(&req, &samples, rates);
        assert!(hard_out.response.hard_budget_cap_tripped);
    }
}
