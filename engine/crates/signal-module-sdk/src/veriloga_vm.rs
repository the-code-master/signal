// Module: engine/crates/signal-module-sdk/src/veriloga_vm.rs — Pure-Rust Deterministic Verilog-A Stack Bytecode VM
// Invariants: Evaluates static I(v) and dynamic ddt q(v) IR via Dual64 AD; zero LLVM in signal-pod (F-115).
// Spec: architecture/03-rust-eda-engine.md, architecture/15-application-security-and-project-isolation.md
use signal_component::{ConservativeStamp, Dual64, ElectronicModule, MultiTerminalStamp};
use signal_core::Microvolts;
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq)]
pub enum VaOp {
    PushConst(f64),
    PushParam(String),
    PushBranchV,
    Add,
    Sub,
    Mul,
    Div,
    LimExp,
    Tanh,
}

/// Pure-Rust AOT-compiled 2-terminal Verilog-A module executing deterministic stack bytecode (`F-115`).
#[derive(Debug, Clone)]
pub struct VerilogACompiledModule {
    pub module_name: String,
    pub urn: String,
    pub ports: [String; 2],
    pub parameters: BTreeMap<String, f64>,
    pub static_ir: Vec<VaOp>,
    pub dynamic_ddt_ir: Vec<VaOp>,
}

impl VerilogACompiledModule {
    pub fn eval_ir(&self, ir: &[VaOp], v_branch: Dual64) -> Dual64 {
        if ir.is_empty() {
            return Dual64::constant(0.0);
        }
        let mut stack: Vec<Dual64> = Vec::with_capacity(ir.len());
        for op in ir {
            match op {
                VaOp::PushConst(c) => stack.push(Dual64::constant(*c)),
                VaOp::PushParam(name) => {
                    let v = self.parameters.get(name).copied().unwrap_or(0.0);
                    stack.push(Dual64::constant(v));
                }
                VaOp::PushBranchV => stack.push(v_branch),
                VaOp::Add => {
                    let b = stack.pop().unwrap_or_default();
                    let a = stack.pop().unwrap_or_default();
                    stack.push(a + b);
                }
                VaOp::Sub => {
                    let b = stack.pop().unwrap_or_default();
                    let a = stack.pop().unwrap_or_default();
                    stack.push(a - b);
                }
                VaOp::Mul => {
                    let b = stack.pop().unwrap_or_default();
                    let a = stack.pop().unwrap_or_default();
                    stack.push(a * b);
                }
                VaOp::Div => {
                    let b = stack.pop().unwrap_or(Dual64::constant(1.0));
                    let a = stack.pop().unwrap_or_default();
                    stack.push(a / b);
                }
                VaOp::LimExp => {
                    let a = stack.pop().unwrap_or_default();
                    stack.push(a.limexp(40.0));
                }
                VaOp::Tanh => {
                    let a = stack.pop().unwrap_or_default();
                    stack.push(a.tanh());
                }
            }
        }
        stack.pop().unwrap_or_default()
    }
}

impl ElectronicModule for VerilogACompiledModule {
    fn model_urn(&self) -> &str {
        &self.urn
    }

    fn terminal_count(&self) -> usize {
        2
    }

    fn evaluate_conservative(
        &self,
        terminal_bias: &[Microvolts],
        _temp_mk: i64,
    ) -> ConservativeStamp {
        let vp = terminal_bias.first().map_or(0.0, |v| v.to_si_f64());
        let vn = terminal_bias.get(1).map_or(0.0, |v| v.to_si_f64());
        let vd = Dual64::variable(vp - vn);
        let i_dual = self.eval_ir(&self.static_ir, vd);
        let q_dual = self.eval_ir(&self.dynamic_ddt_ir, vd);
        ConservativeStamp {
            current_amps: i_dual.val,
            charge_coulombs: q_dual.val,
            flux_webers: 0.0,
            conductance_siemens: i_dual.eps,
        }
    }

    fn evaluate_multi_terminal(
        &self,
        terminal_bias: &[Microvolts],
        _temp_mk: i64,
    ) -> MultiTerminalStamp {
        let vp = terminal_bias.first().map_or(0.0, |v| v.to_si_f64());
        let vn = terminal_bias.get(1).map_or(0.0, |v| v.to_si_f64());
        let vd = Dual64::variable(vp - vn);
        let i_dual = self.eval_ir(&self.static_ir, vd);
        let q_dual = self.eval_ir(&self.dynamic_ddt_ir, vd);
        MultiTerminalStamp {
            terminal_currents_a: vec![i_dual.val, -i_dual.val],
            terminal_charges_c: vec![q_dual.val, -q_dual.val],
            branch_fluxes_wb: Vec::new(),
            g_jacobian: vec![vec![i_dual.eps, -i_dual.eps], vec![-i_dual.eps, i_dual.eps]],
            c_jacobian: vec![vec![q_dual.eps, -q_dual.eps], vec![-q_dual.eps, q_dual.eps]],
            junction_temp_mk: 300_150,
        }
    }
}
