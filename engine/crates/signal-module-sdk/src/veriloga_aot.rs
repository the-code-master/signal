// Module: engine/crates/signal-module-sdk/src/veriloga_aot.rs — Pure-Rust LLVM-Free AOT Verilog-A Parser & IR Lowering
// Invariants: Compiles `analog begin I(p,n) <+ ... ddt(...) end` to deterministic stack IR; zero LLVM in signal-pod (F-115).
// Spec: architecture/03-rust-eda-engine.md, architecture/15-application-security-and-project-isolation.md
use crate::SdkError;
pub use crate::veriloga_vm::{VaOp, VerilogACompiledModule};
use std::collections::BTreeMap;

/// Compiles a Verilog-A compact model (`module ... analog begin I(p,n) <+ ... + ddt(...); end endmodule`) (`F-115`).
pub fn compile_veriloga_aot(source: &str) -> Result<VerilogACompiledModule, SdkError> {
    let mod_pos = source
        .find("module ")
        .ok_or_else(|| SdkError::VerilogACompileError("missing `module` keyword".to_string()))?;
    let after_mod = &source[mod_pos + 7..];
    let (mod_name_raw, rest_hdr) = after_mod
        .split_once('(')
        .ok_or_else(|| SdkError::VerilogACompileError("missing port list".to_string()))?;
    let (ports_raw, _) = rest_hdr
        .split_once(')')
        .ok_or_else(|| SdkError::VerilogACompileError("unclosed port list".to_string()))?;
    let port_vec: Vec<String> = ports_raw
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();
    if port_vec.len() != 2 {
        return Err(SdkError::VerilogACompileError(
            "AOT Verilog-A kernel expects 2-port branch module".to_string(),
        ));
    }

    let mut parameters = BTreeMap::new();
    for seg in source.split(';') {
        let s = seg.trim();
        if let Some(rest) = s.strip_prefix("parameter real ")
            && let Some((k, v)) = rest.split_once('=')
        {
            let val: f64 = v
                .trim()
                .parse()
                .map_err(|_| SdkError::VerilogACompileError(format!("bad parameter `{s}`")))?;
            parameters.insert(k.trim().to_string(), val);
        }
    }

    let analog_pos = source
        .find("analog begin")
        .ok_or_else(|| SdkError::VerilogACompileError("missing `analog begin`".to_string()))?;
    let body_after = &source[analog_pos + 12..];
    let body = body_after
        .split("end")
        .next()
        .ok_or_else(|| SdkError::VerilogACompileError("missing `end`".to_string()))?;

    let mut static_ir = Vec::new();
    let mut dynamic_ddt_ir = Vec::new();

    for stmt in body.split(';') {
        let st = stmt.trim();
        if st.is_empty() {
            continue;
        }
        let (_, rhs) = st
            .split_once("<+")
            .ok_or_else(|| SdkError::VerilogACompileError(format!("expected `<+` in `{st}`")))?;
        split_and_compile_rhs(
            rhs.trim(),
            &port_vec,
            &parameters,
            &mut static_ir,
            &mut dynamic_ddt_ir,
        )?;
    }

    let module_name = mod_name_raw.trim().to_string();
    Ok(VerilogACompiledModule {
        urn: format!("urn:signal:veriloga_aot:{module_name}"),
        module_name,
        ports: [port_vec[0].clone(), port_vec[1].clone()],
        parameters,
        static_ir,
        dynamic_ddt_ir,
    })
}

fn split_and_compile_rhs(
    rhs: &str,
    ports: &[String],
    params: &BTreeMap<String, f64>,
    static_ir: &mut Vec<VaOp>,
    ddt_ir: &mut Vec<VaOp>,
) -> Result<(), SdkError> {
    let mut rest = rhs;
    while let Some(ddt_idx) = rest.find("ddt(") {
        let before = rest[..ddt_idx].trim().trim_end_matches('+').trim();
        if !before.is_empty() {
            append_accum(static_ir, compile_va_expr(before, ports, params)?);
        }
        let after_ddt = &rest[ddt_idx + 4..];
        let mut depth = 1_i32;
        let mut end_idx = None;
        for (i, ch) in after_ddt.char_indices() {
            if ch == '(' {
                depth += 1;
            } else if ch == ')' {
                depth -= 1;
                if depth == 0 {
                    end_idx = Some(i);
                    break;
                }
            }
        }
        let close =
            end_idx.ok_or_else(|| SdkError::VerilogACompileError("unclosed ddt(".to_string()))?;
        append_accum(ddt_ir, compile_va_expr(&after_ddt[..close], ports, params)?);
        rest = after_ddt[close + 1..].trim().trim_start_matches('+').trim();
    }
    if !rest.is_empty() {
        append_accum(static_ir, compile_va_expr(rest, ports, params)?);
    }
    Ok(())
}

fn append_accum(target: &mut Vec<VaOp>, mut term: Vec<VaOp>) {
    if target.is_empty() {
        *target = term;
    } else {
        target.append(&mut term);
        target.push(VaOp::Add);
    }
}

fn compile_va_expr(
    expr: &str,
    ports: &[String],
    params: &BTreeMap<String, f64>,
) -> Result<Vec<VaOp>, SdkError> {
    let v_pat = format!("V({},{})", ports[0], ports[1]);
    let v_pat_sp = format!("V({}, {})", ports[0], ports[1]);
    let normalized = expr
        .trim()
        .replace(&v_pat_sp, "__VBR__")
        .replace(&v_pat, "__VBR__");
    let mut parser = VaExprParser {
        chars: normalized.as_bytes(),
        pos: 0,
        params,
        out: Vec::new(),
    };
    parser.parse_add_sub()?;
    Ok(parser.out)
}

struct VaExprParser<'a> {
    chars: &'a [u8],
    pos: usize,
    params: &'a BTreeMap<String, f64>,
    out: Vec<VaOp>,
}

impl<'a> VaExprParser<'a> {
    fn skip_ws(&mut self) {
        while self.pos < self.chars.len() && self.chars[self.pos].is_ascii_whitespace() {
            self.pos += 1;
        }
    }

    fn parse_add_sub(&mut self) -> Result<(), SdkError> {
        self.parse_mul_div()?;
        loop {
            self.skip_ws();
            match self.chars.get(self.pos) {
                Some(b'+') => {
                    self.pos += 1;
                    self.parse_mul_div()?;
                    self.out.push(VaOp::Add);
                }
                Some(b'-') => {
                    self.pos += 1;
                    self.parse_mul_div()?;
                    self.out.push(VaOp::Sub);
                }
                _ => break,
            }
        }
        Ok(())
    }

    fn parse_mul_div(&mut self) -> Result<(), SdkError> {
        self.parse_atom()?;
        loop {
            self.skip_ws();
            match self.chars.get(self.pos) {
                Some(b'*') => {
                    self.pos += 1;
                    self.parse_atom()?;
                    self.out.push(VaOp::Mul);
                }
                Some(b'/') => {
                    self.pos += 1;
                    self.parse_atom()?;
                    self.out.push(VaOp::Div);
                }
                _ => break,
            }
        }
        Ok(())
    }

    fn parse_atom(&mut self) -> Result<(), SdkError> {
        self.skip_ws();
        if self.chars.get(self.pos) == Some(&b'(') {
            self.pos += 1;
            self.parse_add_sub()?;
            self.skip_ws();
            if self.chars.get(self.pos) != Some(&b')') {
                return Err(SdkError::VerilogACompileError("missing `)`".to_string()));
            }
            self.pos += 1;
            return Ok(());
        }
        let start = self.pos;
        while self.pos < self.chars.len()
            && (self.chars[self.pos].is_ascii_alphanumeric()
                || matches!(self.chars[self.pos], b'_' | b'.'))
        {
            self.pos += 1;
        }
        let tok = std::str::from_utf8(&self.chars[start..self.pos]).unwrap_or("");
        if tok.is_empty() {
            return Err(SdkError::VerilogACompileError(
                "unexpected token in Verilog-A".to_string(),
            ));
        }
        self.skip_ws();
        if self.chars.get(self.pos) == Some(&b'(') {
            self.pos += 1;
            self.parse_add_sub()?;
            self.skip_ws();
            if self.chars.get(self.pos) != Some(&b')') {
                return Err(SdkError::VerilogACompileError(
                    "missing `)` after call".to_string(),
                ));
            }
            self.pos += 1;
            match tok {
                "limexp" | "exp" => self.out.push(VaOp::LimExp),
                "tanh" => self.out.push(VaOp::Tanh),
                other => {
                    return Err(SdkError::VerilogACompileError(format!(
                        "unsupported Verilog-A function `{other}`"
                    )));
                }
            }
            return Ok(());
        }
        if tok == "__VBR__" {
            self.out.push(VaOp::PushBranchV);
        } else if self.params.contains_key(tok) {
            self.out.push(VaOp::PushParam(tok.to_string()));
        } else if let Ok(v) = tok.parse::<f64>() {
            self.out.push(VaOp::PushConst(v));
        } else {
            return Err(SdkError::VerilogACompileError(format!(
                "unknown identifier `{tok}`"
            )));
        }
        Ok(())
    }
}
