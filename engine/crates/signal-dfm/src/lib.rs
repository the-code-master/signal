// Module: engine/crates/signal-dfm/src/lib.rs — Panelization, DFA, Solder Fatigue, GD&T SVG, Should-Cost, ICT & MSL
// Invariants: #![forbid(unsafe_code)]; exact i128/nm keepout checks, stencil AR > 0.66, and J-STD-020 MSL floor-life audit.
// Spec: architecture/03-rust-eda-engine.md, architecture/17-extended-feature-specifications-ui-and-rust.md
#![forbid(unsafe_code)]

use thiserror::Error;

pub mod drawing_cost_ict_msl;
pub mod panel_dfa_fatigue;

pub use drawing_cost_ict_msl::{
    estimate_unit_should_cost_and_fpy, evaluate_dfm_and_panelization_rpc,
    optimize_ict_and_jtag_testpoints, render_fab_gdt_drawing_svg, verify_jstd020_msl_and_reflow,
};
pub use panel_dfa_fatigue::{
    PanelLayoutSummary, compose_manufacturing_panel,
    evaluate_bga_solder_fatigue_and_board_modal_hz, verify_conformal_coating_keepouts,
    verify_dfa_stencil_and_tombstone,
};

#[derive(Debug, Error, PartialEq, Eq)]
pub enum DfmError {
    #[error("invalid panelization or DFM parameter: {0}")]
    InvalidParameter(&'static str),
}

#[cfg(test)]
mod tests {
    use super::*;
    use signal_geom::{BBox2Nm, Point2Nm};
    use signal_proto::v1::EvaluateDfmAndPanelizationRequest;

    #[test]
    fn panelizer_dfa_solder_fatigue_coating_svg_cost_ict_and_msl_verified() {
        let resp = evaluate_dfm_and_panelization_rpc(&EvaluateDfmAndPanelizationRequest {
            branch_id: "main".into(),
            panel_cols: 2,
            panel_rows: 3,
            rail_width_nm: 5_000_000,
            target_build_qty: 1_000,
        });
        assert!(resp.dfa_and_flex_strain_clean && resp.msl_reflow_compliant);
        assert!(resp.board_utilization_pct > 70.0 && resp.ict_jtag_net_coverage_pct >= 95.0);
        assert!(resp.solder_fatigue_cycles_to_fail > 1_000);

        let svg = render_fab_gdt_drawing_svg("CTRL-REV-B", 65_000_000, 45_000_000, 6, &[200, 300]);
        assert!(svg.starts_with("<svg") && svg.contains("T01: 0.20 mm"));

        let coat = [BBox2Nm::from_points(
            Point2Nm::new(0, 0),
            Point2Nm::new(10_000_000, 10_000_000),
        )];
        let keepouts = vec![(
            "J1_USB_C".to_string(),
            BBox2Nm::from_points(
                Point2Nm::new(5_000_000, 5_000_000),
                Point2Nm::new(8_000_000, 8_000_000),
            ),
        )];
        assert_eq!(
            verify_conformal_coating_keepouts(&coat, &keepouts),
            vec!["J1_USB_C".to_string()]
        );

        // MSL 3 exposed > 168 hours fails J-STD-020 without bake!
        assert!(!verify_jstd020_msl_and_reflow(3, 200.0, 245.0, 260.0));
    }
}
