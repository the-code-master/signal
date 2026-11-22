// Module: engine/crates/signal-cosim/src/lib.rs — Cycle-Accurate MCU/Firmware/BIOS Co-Simulator & Protocol Suite
// Invariants: Lockstep RV32IM + MMIO ADC/GPIO/PWM; streaming protocol decode; SVD/DTS, SAT pinmux & cold-boot checks.
// Spec: architecture/17-extended-feature-specifications-ui-and-rust.md
#![forbid(unsafe_code)]

use thiserror::Error;

pub mod boot_and_hil;
pub mod memmap_pinmux_energy;
pub mod rv32_and_protocol;

pub use boot_and_hil::{
    BootMilestoneKind, BootTraceEvent, HilFaultKind, HilWatchdogVerdict, run_firmware_bios_cosim,
    run_hil_watchdog_fault_scenario, verify_cold_boot_post_sequence,
};
pub use memmap_pinmux_energy::{
    BatteryEnergyLedger, FirmwareEnergySample, MemoryApertureRegion, PinmuxPeripheralDemand,
    SolvedPinmuxAssignment, accumulate_firmware_energy_and_battery, solve_conflict_free_pinmux,
    verify_svd_ld_dts_memmap,
};
pub use rv32_and_protocol::{
    MMIO_ADC_IN_ADDR, MMIO_GPIO_OUT_ADDR, MMIO_PWM_DUTY_ADDR, RawBusCaptureEvent, Rv32Inst,
    Rv32MixedSignalCore, decode_bus_protocol_frames,
};

#[derive(Debug, Error, PartialEq, Eq)]
pub enum CosimError {
    #[error("no conflict-free MCU alternate-function pinmux assignment exists")]
    UnsatisfiablePinmux,
    #[error("co-simulation duration ({0} ps) must be strictly positive")]
    InvalidSimDuration(i64),
}

#[cfg(test)]
mod tests {
    use super::*;
    use signal_core::Microvolts;
    use signal_proto::v1::{ProtocolPacketSpan, RunFirmwareBiosCosimRequest};

    #[test]
    fn steps_rv32im_mmio_adc_to_pwm_loop_and_runs_bios_cosim_suite() {
        let mut cpu = Rv32MixedSignalCore::new(10_000, 3_300_000);
        cpu.adc_pin_voltage = Microvolts(1_650_000); // 50% of 3.3V -> ~2047 SAR code
        let prog = [
            Rv32Inst::Addi {
                rd: 1,
                rs1: 0,
                imm: MMIO_GPIO_OUT_ADDR as i32,
            },
            Rv32Inst::Lw {
                rd: 2,
                rs1: 1,
                offset: 4, // MMIO_ADC_IN_ADDR
            },
            Rv32Inst::Sw {
                rs2: 2,
                rs1: 1,
                offset: 8, // MMIO_PWM_DUTY_ADDR (clamped to 1000)
            },
            Rv32Inst::Sw {
                rs2: 2,
                rs1: 1,
                offset: 0, // MMIO_GPIO_OUT_ADDR
            },
        ];
        while cpu.step_instruction(&prog) {}
        assert_eq!(cpu.cycle_count, 4);
        assert_eq!(cpu.gpio_out_mask, 2047);
        assert_eq!(cpu.pwm_duty_permille, 1_000);

        let resp = run_firmware_bios_cosim(&RunFirmwareBiosCosimRequest {
            fencing: None,
            branch_id: "fw-main".into(),
            cpu_arch: "RV32IMAC".into(),
            sim_duration_ps: 2_000_000_000,
        })
        .expect("cosim ok");
        assert!(resp.cold_boot_post_passed);
        assert!(resp.svd_dts_memmap_disjoint);
        assert!(resp.sat_pinmux_conflict_free);
        assert!(resp.total_firmware_energy_nj > 0);
        assert_eq!(resp.decoded_packets.len(), 2);

        let wdt = run_hil_watchdog_fault_scenario(
            HilFaultKind::SupplyBrownoutDrop {
                rail_uv: 2_400_000,
                bor_threshold_uv: 2_700_000,
            },
            50_000_000_000,
            Some(12_000_000_000),
        );
        assert!(wdt.fault_detected && wdt.watchdog_recovered_within_budget);

        // Verify 64-bit string serde roundtrip on golden Phase 6 ProtocolPacketSpan fixture!
        let raw_fixture =
            include_str!("../../../../contracts/fixtures/cosim_twin_shmoo.fixture.json");
        let root: serde_json::Value = serde_json::from_str(raw_fixture).expect("fixture json");
        let pkt: ProtocolPacketSpan =
            serde_json::from_value(root["protocol_packet"].clone()).expect("packet deser");
        assert_eq!(pkt.end_time_ps, 9_007_199_254_740_993_i64);
        assert!(pkt.crc_or_ack_valid);
    }
}
