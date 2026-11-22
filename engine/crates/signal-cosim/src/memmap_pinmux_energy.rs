// Module: engine/crates/signal-cosim/src/memmap_pinmux_energy.rs — SVD/LD/DTS Memory-Map, SAT Pinmux & Energy Ledger (FEAT-054..056)
// Invariants: Detects address-aperture overlaps; solves conflict-free MCU pinmux; integrates V*I*dt in nanojoules.
// Spec: architecture/17-extended-feature-specifications-ui-and-rust.md
use crate::CosimError;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemoryApertureRegion {
    pub name: String,
    pub source_kind: String, // "SVD", "LD", or "DTS"
    pub base_addr: u64,
    pub size_bytes: u64,
}

/// Verifies that CMSIS-SVD peripheral apertures, `.ld` linker sections & `.dts` nodes are disjoint and 4-byte aligned (`FEAT-054`).
pub fn verify_svd_ld_dts_memmap(regions: &[MemoryApertureRegion]) -> Vec<String> {
    let mut violations = Vec::new();
    for (i, a) in regions.iter().enumerate() {
        if a.size_bytes == 0 || !a.base_addr.is_multiple_of(4) {
            violations.push(format!("MMAP-ALIGN-01: `{}` misaligned or empty", a.name));
        }
        let a_end = a.base_addr.saturating_add(a.size_bytes);
        for b in regions.iter().skip(i + 1) {
            let b_end = b.base_addr.saturating_add(b.size_bytes);
            if a.base_addr < b_end && b.base_addr < a_end {
                violations.push(format!(
                    "MMAP-OVLP-01: `{}` ({}) overlaps `{}` ({})",
                    a.name, a.source_kind, b.name, b.source_kind
                ));
            }
        }
    }
    violations
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PinmuxPeripheralDemand {
    pub func_name: String,
    pub candidate_mcu_pins: Vec<(u32, u32)>, // (mcu_pin_number, ratline_crossing_cost)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SolvedPinmuxAssignment {
    pub func_to_pin: BTreeMap<String, u32>,
    pub total_ratline_cost: u32,
    pub rust_hal_init_code: String,
}

fn search_best_pinmux(
    idx: usize,
    demands: &[PinmuxPeripheralDemand],
    used_pins: &mut BTreeSet<u32>,
    curr_map: &mut BTreeMap<String, u32>,
    curr_cost: u32,
    best: &mut Option<(u32, BTreeMap<String, u32>)>,
) {
    if let Some((best_cost, _)) = best
        && curr_cost >= *best_cost
    {
        return;
    }
    if idx == demands.len() {
        *best = Some((curr_cost, curr_map.clone()));
        return;
    }
    for &(pin, cost) in &demands[idx].candidate_mcu_pins {
        if used_pins.insert(pin) {
            curr_map.insert(demands[idx].func_name.clone(), pin);
            search_best_pinmux(
                idx + 1,
                demands,
                used_pins,
                curr_map,
                curr_cost + cost,
                best,
            );
            curr_map.remove(&demands[idx].func_name);
            used_pins.remove(&pin);
        }
    }
}

/// Branch-and-bound SAT/CSP MCU alternate-function pinmux solver + Rust HAL init generator (`FEAT-055`).
pub fn solve_conflict_free_pinmux(
    demands: &[PinmuxPeripheralDemand],
) -> Result<SolvedPinmuxAssignment, CosimError> {
    let mut used = BTreeSet::new();
    let mut curr = BTreeMap::new();
    let mut best = None;
    search_best_pinmux(0, demands, &mut used, &mut curr, 0, &mut best);

    let (cost, map) = best.ok_or(CosimError::UnsatisfiablePinmux)?;
    let mut hal = String::from("pub fn configure_board_pinmux() {\n");
    for (func, pin) in &map {
        hal.push_str(&format!("    hal::gpio::pin({pin}).into_af(\"{func}\");\n"));
    }
    hal.push_str("}\n");

    Ok(SolvedPinmuxAssignment {
        func_to_pin: map,
        total_ratline_cost: cost,
        rust_hal_init_code: hal,
    })
}

#[derive(Debug, Clone, PartialEq)]
pub struct FirmwareEnergySample {
    pub symbol_name: String,
    pub vdd_uv: i64,
    pub idd_na: i64,
    pub duration_ps: i64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BatteryEnergyLedger {
    pub per_symbol_energy_nj: BTreeMap<String, i64>,
    pub total_energy_nj: i64,
    pub estimated_battery_life_hours: f64,
}

/// Integrates $\int V_{DD}(t) I_{DD}(t)\,dt$ per firmware symbol and estimates coin-cell battery lifetime (`FEAT-056`).
pub fn accumulate_firmware_energy_and_battery(
    samples: &[FirmwareEnergySample],
    duty_cycle_period_s: f64,
    battery_capacity_mah: f64,
    nominal_battery_v: f64,
) -> BatteryEnergyLedger {
    let mut per_sym_pj: BTreeMap<String, f64> = BTreeMap::new();
    let mut total_pj = 0.0_f64;

    for s in samples {
        // V(uV=1e-6) * I(nA=1e-9) * dt(ps=1e-12) = 1e-27 J = 1e-15 pJ
        let energy_j = (s.vdd_uv.max(0) as f64)
            * 1e-6
            * (s.idd_na.max(0) as f64)
            * 1e-9
            * (s.duration_ps.max(0) as f64)
            * 1e-12;
        let pj = energy_j * 1e12;
        *per_sym_pj.entry(s.symbol_name.clone()).or_insert(0.0) += pj;
        total_pj += pj;
    }

    let per_symbol_energy_nj = per_sym_pj
        .into_iter()
        .map(|(k, pj)| (k, (pj * 1e-3).round() as i64))
        .collect();
    let total_nj = (total_pj * 1e-3).round() as i64;

    let avg_power_w = ((total_pj * 1e-12) / duty_cycle_period_s.max(1e-6)).max(1e-9);
    let battery_wh = (battery_capacity_mah.max(1.0) * 1e-3) * nominal_battery_v.max(1.5);
    let life_hours = (battery_wh / avg_power_w) * 0.85; // 85% usable cell derating

    BatteryEnergyLedger {
        per_symbol_energy_nj,
        total_energy_nj: total_nj,
        estimated_battery_life_hours: life_hours,
    }
}
