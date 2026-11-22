// Module: engine/crates/signal-fpga/src/io_and_sdc.rs — FPGA Bank V_CCO, SSN, BGA Escape & SDC/XDC CDC Verifier (FEAT-051, 052)
// Invariants: Rejects conflicting bank V_CCO standards or unsynchronized multi-clock crossings (FEAT-051, FEAT-052).
// Spec: architecture/17-extended-feature-specifications-ui-and-rust.md
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FpgaPinAssignment {
    pub signal_name: String,
    pub bank_id: u32,
    pub required_vcco_mv: u32,
    pub is_lvds_diff: bool,
    pub drive_ma: u32,
    pub bga_ball_order: u32,
    pub pcb_target_order: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FpgaIoPlanReport {
    pub vcco_compatible: bool,
    pub ssn_within_budget: bool,
    pub pcb_escape_crossings: u32,
    pub violations: Vec<String>,
}

/// Evaluates FPGA bank $V_{CCO}$ electrical compatibility, SSN ground-bounce budget & BGA dogbone ratline crossings (`FEAT-051`).
pub fn plan_fpga_bank_io_and_escape(
    pins: &[FpgaPinAssignment],
    max_bank_drive_ma: u32,
) -> FpgaIoPlanReport {
    let mut bank_vcco: BTreeMap<u32, u32> = BTreeMap::new();
    let mut bank_drive_sum: BTreeMap<u32, u32> = BTreeMap::new();
    let mut violations = Vec::new();
    let mut vcco_ok = true;

    for p in pins {
        let entry = bank_vcco.entry(p.bank_id).or_insert(p.required_vcco_mv);
        if *entry != p.required_vcco_mv {
            vcco_ok = false;
            violations.push(format!(
                "IO-VCCO-01: pin `{}` needs {} mV in bank {} locked to {} mV",
                p.signal_name, p.required_vcco_mv, p.bank_id, *entry
            ));
        }
        if !p.is_lvds_diff {
            *bank_drive_sum.entry(p.bank_id).or_insert(0) += p.drive_ma;
        }
    }

    let mut ssn_ok = true;
    for (bank, total_ma) in bank_drive_sum {
        if total_ma > max_bank_drive_ma {
            ssn_ok = false;
            violations.push(format!(
                "IO-SSN-01: bank {bank} single-ended switching current {total_ma} mA exceeds {max_bank_drive_ma} mA SSN limit"
            ));
        }
    }

    // Count ratline inversions between BGA escape sequence and PCB connector sequence
    let mut crossings = 0_u32;
    for i in 0..pins.len() {
        for j in (i + 1)..pins.len() {
            let bga_cmp = pins[i].bga_ball_order.cmp(&pins[j].bga_ball_order);
            let pcb_cmp = pins[i].pcb_target_order.cmp(&pins[j].pcb_target_order);
            if bga_cmp != std::cmp::Ordering::Equal
                && pcb_cmp != std::cmp::Ordering::Equal
                && bga_cmp != pcb_cmp
            {
                crossings += 1;
            }
        }
    }

    FpgaIoPlanReport {
        vcco_compatible: vcco_ok,
        ssn_within_budget: ssn_ok,
        pcb_escape_crossings: crossings,
        violations,
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CdcCrossingArc {
    pub src_clk: String,
    pub dst_clk: String,
    pub has_two_ff_or_async_fifo: bool,
    pub has_sdc_false_or_max_delay: bool,
    pub multi_bit_without_gray_code: bool,
}

/// Structural SDC/XDC Clock-Domain-Crossing (`CDC`) and false/max-delay constraint verifier (`FEAT-052`).
pub fn verify_sdc_cdc_constraints(arcs: &[CdcCrossingArc]) -> Vec<String> {
    let mut errs = Vec::new();
    for arc in arcs {
        if arc.src_clk == arc.dst_clk {
            continue;
        }
        if !arc.has_two_ff_or_async_fifo {
            errs.push(format!(
                "CDC-SYNC-01: unsynchronized crossing `{}` -> `{}`",
                arc.src_clk, arc.dst_clk
            ));
        }
        if !arc.has_sdc_false_or_max_delay {
            errs.push(format!(
                "SDC-TIM-01: missing set_max_delay -datapath_only / set_clock_groups on `{}` -> `{}`",
                arc.src_clk, arc.dst_clk
            ));
        }
        if arc.multi_bit_without_gray_code {
            errs.push(format!(
                "CDC-GRAY-01: multi-bit bus `{}` -> `{}` lacks Gray-code or FIFO handshake",
                arc.src_clk, arc.dst_clk
            ));
        }
    }
    errs
}
