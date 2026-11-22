// Module: engine/crates/signal-plm/src/multi_bom_passport.rs — Multi-Domain eBOM/mBOM/sBOM Compiler & Digital Product Passport (F-112)
// Invariants: Synchronizes ECAD+MCAD+CAM/Print+Packaging into eBOM/mBOM/sBOM with exact USD micros & CO2e mg rollup (F-112).
// Spec: architecture/19-cad-cam-3d-printing-and-plm-operations.md
use crate::PlmError;
use signal_proto::v1::{
    BomLineItem, BomViewKind, CompileMultiViewBomRequest, CompileMultiViewBomResponse,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BomSourceDomain {
    EcadPcb,
    McadMechanical,
    CamPrintStock,
    ManufacturingConsumable,
    Packaging,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MultiDomainBomCandidate {
    pub upid: String,
    pub orderable_mpn: String,
    pub domain: BomSourceDomain,
    pub base_qty: u32,
    pub scrap_allowance_pct: f64,
    pub unit_cost_micros_usd: i64,
    pub embodied_co2e_mg: i64,
    pub rohs_reach_pfas_compliant: bool,
    pub workstation_step_id: String,
    pub field_replaceable_spare: bool,
    pub available_stock_units: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DigitalProductPassport {
    pub super_commit_id: String,
    pub total_unit_cost_micros_usd: i64,
    pub total_embodied_co2e_mg: i64,
    pub all_rohs_reach_pfas_compliant: bool,
    pub non_compliant_upids: Vec<String>,
}

/// Compiles synchronized `eBOM`, `mBOM` (with workstation steps, consumables & scrap), or `sBOM` views + Digital Product Passport (`F-112`).
pub fn compile_multiview_bom(
    req: &CompileMultiViewBomRequest,
    candidates: &[MultiDomainBomCandidate],
) -> Result<(CompileMultiViewBomResponse, DigitalProductPassport), PlmError> {
    let view = BomViewKind::try_from(req.view).unwrap_or(BomViewKind::Unspecified);
    if view == BomViewKind::Unspecified {
        return Err(PlmError::UnspecifiedBomView);
    }

    let mut lines = Vec::new();
    let mut total_cost_micros: i64 = 0;
    let mut total_co2e_mg: i64 = 0;
    let mut min_ctb_units = u32::MAX;
    let mut non_compliant = Vec::new();

    for (idx, item) in candidates.iter().enumerate() {
        let include = match view {
            BomViewKind::Ebom => !matches!(
                item.domain,
                BomSourceDomain::ManufacturingConsumable | BomSourceDomain::Packaging
            ),
            BomViewKind::Mbom => true,
            BomViewKind::Sbom => item.field_replaceable_spare,
            BomViewKind::Unspecified => false,
        };
        if !include || item.base_qty == 0 {
            continue;
        }

        let effective_qty = if view == BomViewKind::Mbom {
            let factor = 1.0 + (item.scrap_allowance_pct.clamp(0.0, 50.0) / 100.0);
            ((f64::from(item.base_qty) * factor).ceil() as u32).max(item.base_qty)
        } else {
            item.base_qty
        };

        let line_cost = item
            .unit_cost_micros_usd
            .saturating_mul(i64::from(effective_qty));
        let line_co2 = item
            .embodied_co2e_mg
            .saturating_mul(i64::from(effective_qty));
        total_cost_micros = total_cost_micros.saturating_add(line_cost);
        total_co2e_mg = total_co2e_mg.saturating_add(line_co2);

        if !item.rohs_reach_pfas_compliant {
            non_compliant.push(item.upid.clone());
        }
        let ctb_for_line = item.available_stock_units / effective_qty.max(1);
        min_ctb_units = min_ctb_units.min(ctb_for_line);

        lines.push(BomLineItem {
            line_id: format!("L{:04}", idx + 1),
            upid: item.upid.clone(),
            orderable_mpn: item.orderable_mpn.clone(),
            quantity_per_unit: effective_qty,
            unit_cost_micros_usd: item.unit_cost_micros_usd,
            embodied_co2e_mg: item.embodied_co2e_mg,
            rohs_pfas_compliant: item.rohs_reach_pfas_compliant,
            workstation_step_id: if view == BomViewKind::Mbom {
                item.workstation_step_id.clone()
            } else {
                String::new()
            },
        });
    }

    if lines.is_empty() {
        min_ctb_units = 0;
    }

    let passport = DigitalProductPassport {
        super_commit_id: req.super_commit_id.clone(),
        total_unit_cost_micros_usd: total_cost_micros,
        total_embodied_co2e_mg: total_co2e_mg,
        all_rohs_reach_pfas_compliant: non_compliant.is_empty(),
        non_compliant_upids: non_compliant,
    };

    Ok((
        CompileMultiViewBomResponse {
            view: view as i32,
            lines,
            total_unit_cost_micros_usd: total_cost_micros,
            clear_to_build_max_units: min_ctb_units,
            total_embodied_co2e_mg: total_co2e_mg,
        },
        passport,
    ))
}
