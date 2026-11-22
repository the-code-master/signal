// Module: engine/crates/signal-cosim/src/rv32_and_protocol.rs — Lockstep RV32IM Mixed-Signal Core & Bus Protocol Decoder (FEAT-053)
// Invariants: Steps RV32IM MMIO GPIO/ADC/PWM in lockstep with ps clock; decodes I2C/SPI/CAN_FD/UART/AXI4_LITE frames.
// Spec: architecture/17-extended-feature-specifications-ui-and-rust.md
use signal_core::{Microvolts, Picoseconds};
use signal_proto::v1::ProtocolPacketSpan;

pub const MMIO_GPIO_OUT_ADDR: u32 = 0x4000_0000;
pub const MMIO_ADC_IN_ADDR: u32 = 0x4000_0004;
pub const MMIO_PWM_DUTY_ADDR: u32 = 0x4000_0008;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rv32Inst {
    Addi {
        rd: usize,
        rs1: usize,
        imm: i32,
    },
    Add {
        rd: usize,
        rs1: usize,
        rs2: usize,
    },
    Sub {
        rd: usize,
        rs1: usize,
        rs2: usize,
    },
    Mul {
        rd: usize,
        rs1: usize,
        rs2: usize,
    },
    Lw {
        rd: usize,
        rs1: usize,
        offset: i32,
    },
    Sw {
        rs2: usize,
        rs1: usize,
        offset: i32,
    },
    Bne {
        rs1: usize,
        rs2: usize,
        rel_inst: isize,
    },
}

/// Lockstep cycle-accurate RV32IM / Cortex-M subset core coupled to analog MMIO GPIO/ADC/PWM pins.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rv32MixedSignalCore {
    pub regs: [u32; 32],
    pub pc_index: usize,
    pub cycle_count: u64,
    pub clock_period: Picoseconds,
    pub gpio_out_mask: u32,
    pub pwm_duty_permille: u32,
    pub adc_pin_voltage: Microvolts,
    pub vdd_rail: Microvolts,
}

impl Rv32MixedSignalCore {
    pub fn new(clock_period_ps: i64, vdd_uv: i64) -> Self {
        Self {
            regs: [0; 32],
            pc_index: 0,
            cycle_count: 0,
            clock_period: Picoseconds(clock_period_ps.max(100)),
            gpio_out_mask: 0,
            pwm_duty_permille: 0,
            adc_pin_voltage: Microvolts(0),
            vdd_rail: Microvolts(vdd_uv.max(1_000_000)),
        }
    }

    pub fn step_instruction(&mut self, program: &[Rv32Inst]) -> bool {
        let Some(&inst) = program.get(self.pc_index) else {
            return false;
        };
        let mut next_pc = self.pc_index + 1;
        match inst {
            Rv32Inst::Addi { rd, rs1, imm } => {
                if rd > 0 && rd < 32 {
                    self.regs[rd] = self.regs[rs1 & 31].wrapping_add(imm as u32);
                }
            }
            Rv32Inst::Add { rd, rs1, rs2 } => {
                if rd > 0 && rd < 32 {
                    self.regs[rd] = self.regs[rs1 & 31].wrapping_add(self.regs[rs2 & 31]);
                }
            }
            Rv32Inst::Sub { rd, rs1, rs2 } => {
                if rd > 0 && rd < 32 {
                    self.regs[rd] = self.regs[rs1 & 31].wrapping_sub(self.regs[rs2 & 31]);
                }
            }
            Rv32Inst::Mul { rd, rs1, rs2 } => {
                if rd > 0 && rd < 32 {
                    self.regs[rd] = self.regs[rs1 & 31].wrapping_mul(self.regs[rs2 & 31]);
                }
            }
            Rv32Inst::Lw { rd, rs1, offset } => {
                let addr = self.regs[rs1 & 31].wrapping_add(offset as u32);
                let val = if addr == MMIO_ADC_IN_ADDR {
                    // 12-bit SAR ADC quantized against V_DD
                    let ratio = ((self.adc_pin_voltage.0.max(0) as i128) * 4095)
                        / (self.vdd_rail.0.max(1) as i128);
                    ratio.clamp(0, 4095) as u32
                } else if addr == MMIO_GPIO_OUT_ADDR {
                    self.gpio_out_mask
                } else {
                    0
                };
                if rd > 0 && rd < 32 {
                    self.regs[rd] = val;
                }
            }
            Rv32Inst::Sw { rs2, rs1, offset } => {
                let addr = self.regs[rs1 & 31].wrapping_add(offset as u32);
                let val = self.regs[rs2 & 31];
                if addr == MMIO_GPIO_OUT_ADDR {
                    self.gpio_out_mask = val;
                } else if addr == MMIO_PWM_DUTY_ADDR {
                    self.pwm_duty_permille = val.min(1_000);
                }
            }
            Rv32Inst::Bne { rs1, rs2, rel_inst } => {
                if self.regs[rs1 & 31] != self.regs[rs2 & 31] {
                    next_pc = (self.pc_index as isize + rel_inst).max(0) as usize;
                }
            }
        }
        self.regs[0] = 0;
        self.pc_index = next_pc;
        self.cycle_count += 1;
        true
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RawBusCaptureEvent {
    I2cWrite {
        start_ps: i64,
        end_ps: i64,
        addr7: u8,
        reg_or_data: u8,
        acked: bool,
    },
    SpiTransfer {
        start_ps: i64,
        end_ps: i64,
        mosi_byte: u8,
        miso_byte: u8,
    },
    CanFdFrame {
        start_ps: i64,
        end_ps: i64,
        arb_id: u32,
        payload: Vec<u8>,
        crc_ok: bool,
    },
    Uart8N1Byte {
        start_ps: i64,
        end_ps: i64,
        byte: u8,
        stop_bit_high: bool,
    },
    Axi4LiteWrite {
        start_ps: i64,
        end_ps: i64,
        awaddr: u32,
        wdata: u32,
        bresp_okay: bool,
    },
}

/// Decodes streaming bus protocol events (`I2C`, `SPI`, `CAN_FD`, `UART`, `AXI4_LITE`) into `ProtocolPacketSpan` (`FEAT-053`).
pub fn decode_bus_protocol_frames(events: &[RawBusCaptureEvent]) -> Vec<ProtocolPacketSpan> {
    events
        .iter()
        .map(|ev| match ev {
            RawBusCaptureEvent::I2cWrite {
                start_ps,
                end_ps,
                addr7,
                reg_or_data,
                acked,
            } => ProtocolPacketSpan {
                bus_protocol: "I2C".into(),
                start_time_ps: *start_ps,
                end_time_ps: *end_ps,
                decoded_summary: format!(
                    "START | ADDR 0x{addr7:02X} WR | DATA 0x{reg_or_data:02X} | {}",
                    if *acked { "ACK" } else { "NACK" }
                ),
                crc_or_ack_valid: *acked,
            },
            RawBusCaptureEvent::SpiTransfer {
                start_ps,
                end_ps,
                mosi_byte,
                miso_byte,
            } => ProtocolPacketSpan {
                bus_protocol: "SPI".into(),
                start_time_ps: *start_ps,
                end_time_ps: *end_ps,
                decoded_summary: format!(
                    "CS_LOW | MOSI 0x{mosi_byte:02X} | MISO 0x{miso_byte:02X}"
                ),
                crc_or_ack_valid: true,
            },
            RawBusCaptureEvent::CanFdFrame {
                start_ps,
                end_ps,
                arb_id,
                payload,
                crc_ok,
            } => ProtocolPacketSpan {
                bus_protocol: "CAN_FD".into(),
                start_time_ps: *start_ps,
                end_time_ps: *end_ps,
                decoded_summary: format!("ID 0x{arb_id:03X} | DLC {} | BRS", payload.len()),
                crc_or_ack_valid: *crc_ok,
            },
            RawBusCaptureEvent::Uart8N1Byte {
                start_ps,
                end_ps,
                byte,
                stop_bit_high,
            } => ProtocolPacketSpan {
                bus_protocol: "UART".into(),
                start_time_ps: *start_ps,
                end_time_ps: *end_ps,
                decoded_summary: format!("8N1 0x{byte:02X}"),
                crc_or_ack_valid: *stop_bit_high,
            },
            RawBusCaptureEvent::Axi4LiteWrite {
                start_ps,
                end_ps,
                awaddr,
                wdata,
                bresp_okay,
            } => ProtocolPacketSpan {
                bus_protocol: "AXI4_LITE".into(),
                start_time_ps: *start_ps,
                end_time_ps: *end_ps,
                decoded_summary: format!("AWADDR 0x{awaddr:08X} | WDATA 0x{wdata:08X} | BRESP"),
                crc_or_ack_valid: *bresp_okay,
            },
        })
        .collect()
}
