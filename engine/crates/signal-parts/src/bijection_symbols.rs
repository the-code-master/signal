// Module: engine/crates/signal-parts/src/bijection_symbols.rs — 4-Way Pin Bijection, IEEE/IEC Symbols & Bank Splitter
// Invariants: Enforces 1:1 bijection across Symbol Pin <-> Footprint Pad <-> SPICE Port <-> 3D STEP Lead (F-064, FEAT-072).
// Spec: architecture/18-universal-component-and-datasheet-repository.md, architecture/17-extended-feature-specifications-ui-and-rust.md

use crate::PartsError;
use signal_core::sha512_256_hex;
use signal_proto::v1::{
    BoundingBox2dNm, FunctionalSymbolBank, MasterPinRecord, SplitSymbolBanksRequest,
    SplitSymbolBanksResponse, SymbolGraphicAst, SymbolStandard,
};
use std::collections::BTreeSet;

/// Verifies strict 4-way bijection across `MasterPinRecord` entries (`F-064`):
/// non-empty unique `pin_number`, unique `pad_designator`, unique nonzero `spice_port_id`,
/// and explicit 3D `step_lead_centroid_nm` coordinates.
pub fn verify_four_way_master_pin_bijection(pins: &[MasterPinRecord]) -> Result<(), PartsError> {
    if pins.is_empty() {
        return Err(PartsError::PinBijectionViolation(
            "component has zero master pins".into(),
        ));
    }
    let mut seen_pins = BTreeSet::new();
    let mut seen_pads = BTreeSet::new();
    let mut seen_spice_ports = BTreeSet::new();

    for p in pins {
        if p.pin_number.trim().is_empty() || !seen_pins.insert(p.pin_number.trim().to_string()) {
            return Err(PartsError::PinBijectionViolation(format!(
                "duplicate or empty symbol pin_number `{}`",
                p.pin_number
            )));
        }
        if p.pad_designator.trim().is_empty()
            || !seen_pads.insert(p.pad_designator.trim().to_string())
        {
            return Err(PartsError::PinBijectionViolation(format!(
                "duplicate or missing footprint pad_designator `{}` for pin `{}`",
                p.pad_designator, p.pin_number
            )));
        }
        if p.spice_port_id == 0 || !seen_spice_ports.insert(p.spice_port_id) {
            return Err(PartsError::PinBijectionViolation(format!(
                "duplicate or zero spice_port_id {} on pin `{}`",
                p.spice_port_id, p.pin_number
            )));
        }
        if p.step_lead_centroid_nm.is_none() {
            return Err(PartsError::PinBijectionViolation(format!(
                "missing 3D STEP lead centroid for pin `{}`",
                p.pin_number
            )));
        }
    }
    Ok(())
}

/// Generates Dual-Standard (`IEEE 315` zig-zag/curved & `IEC 60617` rectangular metric) `SymbolGraphicAst` pair (`18-*`).
pub fn build_dual_standard_symbol_asts(
    upid: &str,
    pins: &[MasterPinRecord],
    unit_count: u32,
) -> [SymbolGraphicAst; 2] {
    let half_h_nm = (i64::from((pins.len() as u32).max(2)) * 1_270_000).max(2_540_000);
    let bounds_ieee = BoundingBox2dNm {
        min_x_nm: -5_080_000,
        min_y_nm: -half_h_nm,
        max_x_nm: 5_080_000,
        max_y_nm: half_h_nm,
    };
    let bounds_iec = BoundingBox2dNm {
        min_x_nm: -5_000_000,
        min_y_nm: -half_h_nm,
        max_x_nm: 5_000_000,
        max_y_nm: half_h_nm,
    };
    let units = unit_count.max(1);
    let ieee_svg = format!("IEEE315:{upid}:pins={}:units={units}:zigzag", pins.len());
    let iec_svg = format!("IEC60617:{upid}:pins={}:units={units}:rectbox", pins.len());

    [
        SymbolGraphicAst {
            standard: SymbolStandard::Ieee315 as i32,
            unit_count: units,
            body_bounds_nm: Some(bounds_ieee),
            svg_path_digest_blake3: sha512_256_hex(ieee_svg.as_bytes()),
        },
        SymbolGraphicAst {
            standard: SymbolStandard::Iec60617 as i32,
            unit_count: units,
            body_bounds_nm: Some(bounds_iec),
            svg_path_digest_blake3: sha512_256_hex(iec_svg.as_bytes()),
        },
    ]
}

fn classify_pin_functional_group(sig_name: &str) -> &'static str {
    let up = sig_name.trim().to_ascii_uppercase();
    if up.contains("VCC")
        || up.contains("VDD")
        || up.contains("GND")
        || up.contains("VSS")
        || up.contains("VBAT")
    {
        "Power & Ground"
    } else if up.contains("CLK") || up.contains("OSC") || up.contains("RST") || up.contains("BOOT")
    {
        "Clock, Reset & Control"
    } else if up.contains("PCIE")
        || up.contains("USB")
        || up.contains("DDR")
        || up.contains("SERDES")
        || up.contains("ETH")
    {
        "High-Speed Transceivers"
    } else {
        "GPIO & Peripherals"
    }
}

/// Splits high-pin-count MCU/FPGA symbols (`100–2,000+` pins) into logical functional banks (`FEAT-072`).
pub fn split_symbol_into_functional_banks(
    pins: &[MasterPinRecord],
    req: &SplitSymbolBanksRequest,
) -> SplitSymbolBanksResponse {
    let cap = (req.max_pins_per_bank as usize).clamp(4, 256);
    let categories = [
        "Power & Ground",
        "Clock, Reset & Control",
        "High-Speed Transceivers",
        "GPIO & Peripherals",
    ];
    let mut banks = Vec::new();
    let mut next_unit = 1_u32;

    for &cat in &categories {
        let matching: Vec<String> = pins
            .iter()
            .filter(|p| classify_pin_functional_group(&p.canonical_signal_name) == cat)
            .map(|p| p.pin_number.clone())
            .collect();
        for (part_idx, chunk) in matching.chunks(cap).enumerate() {
            let title = if matching.len() > cap {
                format!("{cat} (Bank {})", part_idx + 1)
            } else {
                cat.to_string()
            };
            banks.push(FunctionalSymbolBank {
                unit_index: next_unit,
                bank_title: title,
                pin_numbers: chunk.to_vec(),
            });
            next_unit += 1;
        }
    }
    SplitSymbolBanksResponse { banks }
}
