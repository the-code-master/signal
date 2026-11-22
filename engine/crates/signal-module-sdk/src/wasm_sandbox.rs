// Module: engine/crates/signal-module-sdk/src/wasm_sandbox.rs — Sandboxed Custom Component SDK & Automated Smoke Harness (FEAT-074)
// Invariants: Zero host syscalls/network; enforces deterministic instruction fuel, 256 KiB scratchpad limit, and passivity checks.
// Spec: architecture/03-rust-eda-engine.md, architecture/17-extended-feature-specifications-ui-and-rust.md

use crate::SdkError;
use signal_component::{ConservativeStamp, ElectronicModule, MultiTerminalStamp};
use signal_core::{Microvolts, det_tanh};

pub const CUSTOM_COMPONENT_MAX_SCRATCH_BYTES: usize = 256 * 1024;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SandboxBytecodeOp {
    PushTerminalVoltage(usize),
    PushParameter(usize),
    PushConst(f64),
    Add,
    Sub,
    Mul,
    DivChecked,
    DetTanh,
    EmitBranchCurrent { pos: usize, neg: usize },
    EmitBranchCharge { pos: usize, neg: usize },
}

#[derive(Debug, Clone)]
pub struct WasmComponentSandbox {
    pub model_id: String,
    pub terminal_count: usize,
    pub parameters: Vec<f64>,
    pub bytecode: Vec<SandboxBytecodeOp>,
    pub max_fuel_per_eval: u64,
    pub declared_scratch_bytes: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SandboxSmokeReport {
    pub zero_bias_leakage_ok: bool,
    pub passive_quadrant_ok: bool,
    pub jacobian_finite_ok: bool,
    pub max_fuel_consumed: u64,
}

impl WasmComponentSandbox {
    pub fn new(
        model_id: &str,
        terminal_count: usize,
        parameters: Vec<f64>,
        bytecode: Vec<SandboxBytecodeOp>,
        max_fuel_per_eval: u64,
        declared_scratch_bytes: usize,
    ) -> Result<Self, SdkError> {
        if terminal_count == 0 || terminal_count > 16 {
            return Err(SdkError::CustomWasmSandboxError(format!(
                "invalid terminal_count {terminal_count} (expected 1..=16)"
            )));
        }
        if declared_scratch_bytes > CUSTOM_COMPONENT_MAX_SCRATCH_BYTES {
            return Err(SdkError::CustomWasmSandboxError(format!(
                "scratchpad {declared_scratch_bytes} B exceeds 256 KiB cap"
            )));
        }
        if bytecode.is_empty() || max_fuel_per_eval == 0 {
            return Err(SdkError::CustomWasmSandboxError(
                "empty bytecode or zero fuel budget".into(),
            ));
        }
        Ok(Self {
            model_id: model_id.to_string(),
            terminal_count,
            parameters,
            bytecode,
            max_fuel_per_eval,
            declared_scratch_bytes,
        })
    }

    /// Executes metered custom component bytecode for a single voltage vector, returning `(currents_a, charges_c, fuel_used)`.
    pub fn eval_metered(&self, v_term: &[f64]) -> Result<(Vec<f64>, Vec<f64>, u64), SdkError> {
        if v_term.len() != self.terminal_count {
            return Err(SdkError::CustomWasmSandboxError(
                "terminal voltage length mismatch".into(),
            ));
        }
        let mut stack = Vec::with_capacity(16);
        let mut currents = vec![0.0_f64; self.terminal_count];
        let mut charges = vec![0.0_f64; self.terminal_count];
        let mut fuel_used = 0_u64;

        for &op in &self.bytecode {
            fuel_used += 1;
            if fuel_used > self.max_fuel_per_eval {
                return Err(SdkError::CustomWasmSandboxError(format!(
                    "custom component `{}` exceeded fuel budget {}",
                    self.model_id, self.max_fuel_per_eval
                )));
            }
            match op {
                SandboxBytecodeOp::PushTerminalVoltage(idx) => {
                    let v = *v_term.get(idx).ok_or_else(|| {
                        SdkError::CustomWasmSandboxError("terminal index out of bounds".into())
                    })?;
                    stack.push(v);
                }
                SandboxBytecodeOp::PushParameter(idx) => {
                    let p = *self.parameters.get(idx).ok_or_else(|| {
                        SdkError::CustomWasmSandboxError("parameter index out of bounds".into())
                    })?;
                    stack.push(p);
                }
                SandboxBytecodeOp::PushConst(c) => stack.push(c),
                SandboxBytecodeOp::Add => {
                    let (b, a) = pop2(&mut stack)?;
                    stack.push(a + b);
                }
                SandboxBytecodeOp::Sub => {
                    let (b, a) = pop2(&mut stack)?;
                    stack.push(a - b);
                }
                SandboxBytecodeOp::Mul => {
                    let (b, a) = pop2(&mut stack)?;
                    stack.push(a * b);
                }
                SandboxBytecodeOp::DivChecked => {
                    let (b, a) = pop2(&mut stack)?;
                    let denom = if b.abs() < 1e-18 {
                        1e-18_f64.copysign(b)
                    } else {
                        b
                    };
                    stack.push(a / denom);
                }
                SandboxBytecodeOp::DetTanh => {
                    let a = pop1(&mut stack)?;
                    stack.push(det_tanh(a));
                }
                SandboxBytecodeOp::EmitBranchCurrent { pos, neg } => {
                    let val = pop1(&mut stack)?;
                    add_branch(&mut currents, pos, neg, val)?;
                }
                SandboxBytecodeOp::EmitBranchCharge { pos, neg } => {
                    let val = pop1(&mut stack)?;
                    add_branch(&mut charges, pos, neg, val)?;
                }
            }
        }

        if currents
            .iter()
            .chain(charges.iter())
            .any(|x| !x.is_finite())
        {
            return Err(SdkError::CustomWasmSandboxError(
                "non-finite NaN/Inf stamp rejected".into(),
            ));
        }
        Ok((currents, charges, fuel_used))
    }

    /// Automated DC/passivity/Jacobian smoke-test harness required before publishing a custom SDK module (`FEAT-074`).
    pub fn run_smoke_test_harness(&self) -> Result<SandboxSmokeReport, SdkError> {
        let zeros = vec![0.0_f64; self.terminal_count];
        let (i0, _, f0) = self.eval_metered(&zeros)?;
        let zero_bias_leakage_ok = i0.iter().all(|i| i.abs() <= 1e-9);

        let mut bias = vec![0.0_f64; self.terminal_count];
        bias[0] = 1.0;
        let (ib, _, f1) = self.eval_metered(&bias)?;
        let dissipated_w: f64 = bias.iter().zip(ib.iter()).map(|(v, i)| v * i).sum();
        let passive_quadrant_ok = dissipated_w >= -1e-12;

        let uv: Vec<Microvolts> = bias
            .iter()
            .map(|v| Microvolts((v * 1_000_000.0) as i64))
            .collect();
        let stamp = self.evaluate_multi_terminal(&uv, 300_150);
        let jacobian_finite_ok = stamp
            .g_jacobian
            .iter()
            .chain(stamp.c_jacobian.iter())
            .flatten()
            .all(|x| x.is_finite());

        Ok(SandboxSmokeReport {
            zero_bias_leakage_ok,
            passive_quadrant_ok,
            jacobian_finite_ok,
            max_fuel_consumed: f0.max(f1),
        })
    }
}

fn pop1(stack: &mut Vec<f64>) -> Result<f64, SdkError> {
    stack
        .pop()
        .ok_or_else(|| SdkError::CustomWasmSandboxError("bytecode stack underflow".into()))
}

fn pop2(stack: &mut Vec<f64>) -> Result<(f64, f64), SdkError> {
    let b = pop1(stack)?;
    let a = pop1(stack)?;
    Ok((b, a))
}

fn add_branch(vec: &mut [f64], pos: usize, neg: usize, val: f64) -> Result<(), SdkError> {
    if pos >= vec.len() || neg >= vec.len() {
        return Err(SdkError::CustomWasmSandboxError(
            "branch terminal out of bounds".into(),
        ));
    }
    vec[pos] += val;
    vec[neg] -= val;
    Ok(())
}

fn zero_multi_stamp(n: usize, temp_mk: i64) -> MultiTerminalStamp {
    MultiTerminalStamp {
        terminal_currents_a: vec![0.0; n],
        terminal_charges_c: vec![0.0; n],
        branch_fluxes_wb: Vec::new(),
        g_jacobian: vec![vec![0.0; n]; n],
        c_jacobian: vec![vec![0.0; n]; n],
        junction_temp_mk: temp_mk,
    }
}

impl ElectronicModule for WasmComponentSandbox {
    fn model_urn(&self) -> &str {
        &self.model_id
    }

    fn terminal_count(&self) -> usize {
        self.terminal_count
    }

    fn evaluate_conservative(
        &self,
        terminal_bias: &[Microvolts],
        temp_mk: i64,
    ) -> ConservativeStamp {
        let multi = self.evaluate_multi_terminal(terminal_bias, temp_mk);
        ConservativeStamp {
            current_amps: multi.terminal_currents_a.first().copied().unwrap_or(0.0),
            charge_coulombs: multi.terminal_charges_c.first().copied().unwrap_or(0.0),
            flux_webers: 0.0,
            conductance_siemens: multi
                .g_jacobian
                .first()
                .and_then(|r| r.first())
                .copied()
                .unwrap_or(0.0),
        }
    }

    fn evaluate_multi_terminal(
        &self,
        terminal_voltages_uv: &[Microvolts],
        temp_mk: i64,
    ) -> MultiTerminalStamp {
        let n = self.terminal_count;
        let base_v: Vec<f64> = terminal_voltages_uv
            .iter()
            .map(|uv| (uv.0 as f64) * 1e-6)
            .collect();
        let Ok((base_i, base_q, _)) = self.eval_metered(&base_v) else {
            return zero_multi_stamp(n, temp_mk);
        };
        let mut stamp = zero_multi_stamp(n, temp_mk);
        stamp.terminal_currents_a = base_i.clone();
        stamp.terminal_charges_c = base_q.clone();

        let eps = 1e-6_f64;
        for col in 0..n {
            let mut pert = base_v.clone();
            pert[col] += eps;
            if let Ok((pi, pq, _)) = self.eval_metered(&pert) {
                for row in 0..n {
                    stamp.g_jacobian[row][col] = (pi[row] - base_i[row]) / eps;
                    stamp.c_jacobian[row][col] = (pq[row] - base_q[row]) / eps;
                }
            }
        }
        stamp
    }
}
