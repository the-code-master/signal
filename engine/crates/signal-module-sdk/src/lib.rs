// Module: engine/crates/signal-module-sdk/src/lib.rs — SPICE Multi-Dialect Parser, LLVM-Free Verilog-A AOT & MicroVM IPC
// Invariants: Zero LLVM / zero dlopen inside signal-pod (F-114, F-115); #![forbid(unsafe_code)].
// Spec: architecture/03-rust-eda-engine.md, architecture/15-application-security-and-project-isolation.md
#![forbid(unsafe_code)]

use thiserror::Error;

pub mod microvm_ipc;
pub mod spice_parser;
pub mod veriloga_aot;
pub mod veriloga_vm;

pub use microvm_ipc::{
    ChildMicroVmPolicy, MicroVmShmRing, OsdiStampRequestFrame, OsdiStampResponseFrame,
    SandboxModelKind, decode_osdi_ipc_request, encode_osdi_ipc_request,
};
pub use spice_parser::{
    ParsedNetlist, SpiceDialect, SpiceElement, SpiceModelCard, SpiceSubcktDef,
    flatten_netlist_hierarchy, parse_spice_netlist,
};
pub use veriloga_aot::{VaOp, VerilogACompiledModule, compile_veriloga_aot};

#[derive(Debug, Error, PartialEq, Eq)]
pub enum SdkError {
    #[error("SPICE netlist parse error: {0}")]
    SpiceParseError(String),
    #[error("Verilog-A AOT compile error: {0}")]
    VerilogACompileError(String),
    #[error("Child-MicroVM OSDI/IBIS-AMI sandbox policy/IPC error: {0}")]
    MicroVmSandboxViolation(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use signal_component::ElectronicModule;
    use signal_core::Microvolts;

    #[test]
    fn parses_ngspice_and_spectre_subckts_and_flattens_hierarchy() {
        let deck = r#"
* Buck RC filter subcircuit
.model DMOD D (is=1e-14 n=1.05)
.subckt RC_STAGE in out
R1 in out 1k
C1 out 0 100n
.ends
V1 vin 0 5
X1 vin vmid RC_STAGE
D1 vmid 0 DMOD
"#;
        let parsed = parse_spice_netlist(deck, SpiceDialect::Ngspice).expect("parse ngspice");
        assert!(parsed.models.contains_key("DMOD"));
        let flat = flatten_netlist_hierarchy(&parsed).expect("flatten");
        assert_eq!(flat.len(), 4); // V1, X1.R1, X1.C1, D1

        let spectre = "Rload (vout 0) resistor r=4.7k";
        let sp = parse_spice_netlist(spectre, SpiceDialect::Spectre).expect("spectre");
        assert_eq!(sp.top_elements[0].primitive, 'R');
        assert!((sp.top_elements[0].numeric_value.expect("val") - 4_700.0).abs() < 1e-9);
    }

    #[test]
    fn compiles_veriloga_rc_diode_branch_without_llvm_and_evaluates_exact_ad() {
        let va_src = r#"
module nonlinear_rc(p, n);
    electrical p, n;
    parameter real g0 = 0.002;
    parameter real c0 = 1e-9;
    analog begin
        I(p, n) <+ g0 * V(p, n) + ddt(c0 * V(p, n));
    end
endmodule
"#;
        let compiled = compile_veriloga_aot(va_src).expect("compile va");
        let stamp =
            compiled.evaluate_multi_terminal(&[Microvolts(2_000_000), Microvolts(0)], 300_150);
        assert!((stamp.terminal_currents_a[0] - 0.004).abs() < 1e-12);
        assert!((stamp.terminal_charges_c[0] - 2.0e-9).abs() < 1e-18);
        assert!((stamp.g_jacobian[0][0] - 0.002).abs() < 1e-12);
        assert!((stamp.c_jacobian[0][0] - 1.0e-9).abs() < 1e-18);
    }

    #[test]
    fn microvm_shm_ring_enforces_zero_network_policy_and_roundtrips_osdi_frames() {
        let mut bad_policy = ChildMicroVmPolicy::STRICT_ZERO_NETWORK;
        bad_policy.network_namespace_none = false;
        assert!(MicroVmShmRing::new(bad_policy, 4).is_err());

        let mut ring =
            MicroVmShmRing::new(ChildMicroVmPolicy::STRICT_ZERO_NETWORK, 4).expect("ring");
        let req = OsdiStampRequestFrame {
            seq_id: 77,
            kind: SandboxModelKind::OsdiCompactModel,
            temp_mk: 300_150,
            terminal_voltages_v: vec![1.8, 0.9, 0.0],
        };
        ring.push_request(&req).expect("push");
        let popped = ring.pop_request().expect("pop").expect("frame");
        assert_eq!(popped, req);
    }
}
