// Module: engine/crates/signal-core/src/expr_solver.rs — Deterministic Parametric Equation Evaluator (FEAT-010)
// Invariants: Recursive-descent parser (no eval/regex); uses core_math for cross-ISA determinism; depth <= 64.
// Spec: architecture/01-product-vision-and-features.md, architecture/17-extended-feature-specifications-ui-and-rust.md
use crate::CoreError;
use crate::core_math::{det_cos, det_exp, det_ln, det_sin};
use crate::units::parse_spice_si;
use std::collections::BTreeMap;

const MAX_EXPR_DEPTH: usize = 64;
const LN_10: f64 = std::f64::consts::LN_10;
const PI_CONST: f64 = std::f64::consts::PI;

#[derive(Debug, Clone, PartialEq)]
pub struct SolvedEquation {
    pub target_variable: Option<String>,
    pub value: f64,
}

#[derive(Debug, Clone, PartialEq)]
enum Token {
    Number(f64),
    Ident(String),
    Plus,
    Minus,
    Star,
    Slash,
    Caret,
    LParen,
    RParen,
    Comma,
}

fn tokenize(src: &str) -> Result<Vec<Token>, CoreError> {
    let mut tokens = Vec::new();
    let bytes = src.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        let ch = bytes[i];
        if ch.is_ascii_whitespace() {
            i += 1;
            continue;
        }
        if let Some(op_tok) = match ch {
            b'+' => Some(Token::Plus),
            b'-' => Some(Token::Minus),
            b'*' => Some(Token::Star),
            b'/' => Some(Token::Slash),
            b'^' => Some(Token::Caret),
            b'(' => Some(Token::LParen),
            b')' => Some(Token::RParen),
            b',' => Some(Token::Comma),
            _ => None,
        } {
            tokens.push(op_tok);
            i += 1;
            continue;
        }
        match ch {
            b'0'..=b'9' | b'.' => {
                let start = i;
                while i < bytes.len()
                    && (bytes[i].is_ascii_alphanumeric()
                        || bytes[i] == b'.'
                        || ((bytes[i] == b'+' || bytes[i] == b'-')
                            && i > start
                            && (bytes[i - 1] == b'e' || bytes[i - 1] == b'E')))
                {
                    i += 1;
                }
                let val = parse_spice_si(&src[start..i])?;
                tokens.push(Token::Number(val));
            }
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                let start = i;
                while i < bytes.len() && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_') {
                    i += 1;
                }
                tokens.push(Token::Ident(src[start..i].to_string()));
            }
            other => {
                return Err(CoreError::ExpressionError(format!(
                    "unexpected character `{}` in expression",
                    char::from(other)
                )));
            }
        }
    }
    Ok(tokens)
}

struct Parser<'a> {
    tokens: &'a [Token],
    pos: usize,
    vars: &'a BTreeMap<String, f64>,
}

impl<'a> Parser<'a> {
    fn peek(&self) -> Option<&'a Token> {
        self.tokens.get(self.pos)
    }

    fn parse_expr(&mut self, depth: usize) -> Result<f64, CoreError> {
        if depth > MAX_EXPR_DEPTH {
            return Err(CoreError::ExpressionError(
                "expression nesting > 64".to_string(),
            ));
        }
        let mut lhs = self.parse_term(depth + 1)?;
        while let Some(tok) = self.peek() {
            match tok {
                Token::Plus => {
                    self.pos += 1;
                    lhs += self.parse_term(depth + 1)?;
                }
                Token::Minus => {
                    self.pos += 1;
                    lhs -= self.parse_term(depth + 1)?;
                }
                _ => break,
            }
        }
        Ok(lhs)
    }

    fn parse_term(&mut self, depth: usize) -> Result<f64, CoreError> {
        let mut lhs = self.parse_power(depth + 1)?;
        while let Some(tok) = self.peek() {
            match tok {
                Token::Star => {
                    self.pos += 1;
                    lhs *= self.parse_power(depth + 1)?;
                }
                Token::Slash => {
                    self.pos += 1;
                    let rhs = self.parse_power(depth + 1)?;
                    if rhs.abs() < 1.0e-300 {
                        return Err(CoreError::ExpressionError("division by zero".to_string()));
                    }
                    lhs /= rhs;
                }
                _ => break,
            }
        }
        Ok(lhs)
    }

    fn parse_power(&mut self, depth: usize) -> Result<f64, CoreError> {
        let base = self.parse_unary(depth + 1)?;
        if matches!(self.peek(), Some(Token::Caret)) {
            self.pos += 1;
            let exp = self.parse_power(depth + 1)?;
            if base < 0.0 && exp.fract() != 0.0 {
                return Err(CoreError::ExpressionError(
                    "fractional power of negative".to_string(),
                ));
            }
            if base == 0.0 {
                return Ok(0.0);
            }
            return Ok(if base > 0.0 {
                det_exp(exp * det_ln(base))
            } else {
                base.powi(exp as i32)
            });
        }
        Ok(base)
    }

    fn parse_unary(&mut self, depth: usize) -> Result<f64, CoreError> {
        match self.peek() {
            Some(Token::Plus) => {
                self.pos += 1;
                self.parse_unary(depth + 1)
            }
            Some(Token::Minus) => {
                self.pos += 1;
                Ok(-self.parse_unary(depth + 1)?)
            }
            _ => self.parse_primary(depth + 1),
        }
    }

    fn parse_primary(&mut self, depth: usize) -> Result<f64, CoreError> {
        let tok = self.peek().ok_or_else(|| {
            CoreError::ExpressionError("unexpected end of expression".to_string())
        })?;
        match tok {
            Token::Number(v) => {
                self.pos += 1;
                Ok(*v)
            }
            Token::LParen => {
                self.pos += 1;
                let inner = self.parse_expr(depth + 1)?;
                if !matches!(self.peek(), Some(Token::RParen)) {
                    return Err(CoreError::ExpressionError(
                        "missing closing `)`".to_string(),
                    ));
                }
                self.pos += 1;
                Ok(inner)
            }
            Token::Ident(name) => {
                self.pos += 1;
                if matches!(self.peek(), Some(Token::LParen)) {
                    self.pos += 1;
                    let arg0 = self.parse_expr(depth + 1)?;
                    let mut arg1 = None;
                    if matches!(self.peek(), Some(Token::Comma)) {
                        self.pos += 1;
                        arg1 = Some(self.parse_expr(depth + 1)?);
                    }
                    if !matches!(self.peek(), Some(Token::RParen)) {
                        return Err(CoreError::ExpressionError(
                            "missing `)` in function call".to_string(),
                        ));
                    }
                    self.pos += 1;
                    eval_builtin_fn(name, arg0, arg1)
                } else if name.eq_ignore_ascii_case("pi") {
                    Ok(PI_CONST)
                } else {
                    self.vars.get(name).copied().ok_or_else(|| {
                        CoreError::ExpressionError(format!("unknown variable `{name}`"))
                    })
                }
            }
            other => Err(CoreError::ExpressionError(format!(
                "unexpected token `{other:?}`"
            ))),
        }
    }
}

fn eval_builtin_fn(name: &str, a: f64, b: Option<f64>) -> Result<f64, CoreError> {
    match (name, b) {
        ("sqrt", None) => {
            if a < 0.0 {
                return Err(CoreError::ExpressionError(
                    "sqrt of negative value".to_string(),
                ));
            }
            Ok(a.sqrt())
        }
        ("log10", None) => {
            if a <= 0.0 {
                return Err(CoreError::ExpressionError(
                    "log10 of non-positive value".to_string(),
                ));
            }
            Ok(det_ln(a) / LN_10)
        }
        ("ln", None) => {
            if a <= 0.0 {
                return Err(CoreError::ExpressionError(
                    "ln of non-positive value".to_string(),
                ));
            }
            Ok(det_ln(a))
        }
        ("exp", None) => Ok(det_exp(a)),
        ("sin", None) => Ok(det_sin(a)),
        ("cos", None) => Ok(det_cos(a)),
        ("abs", None) => Ok(a.abs()),
        ("min", Some(rhs)) => Ok(a.min(rhs)),
        ("max", Some(rhs)) => Ok(a.max(rhs)),
        _ => Err(CoreError::ExpressionError(format!(
            "unsupported function `{name}`"
        ))),
    }
}

/// Evaluates a parametric schematic equation (`R_top = R_bot * (V_out / V_ref - 1)`) or pure expression (`FEAT-010`).
pub fn evaluate_parametric_equation(
    equation: &str,
    vars: &BTreeMap<String, f64>,
) -> Result<SolvedEquation, CoreError> {
    let (target_variable, rhs) = if let Some((lhs, rest)) = equation.split_once('=') {
        let ident = lhs.trim();
        if ident.is_empty()
            || !ident
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b == b'_')
        {
            return Err(CoreError::ExpressionError(format!(
                "invalid LHS variable `{ident}`"
            )));
        }
        (Some(ident.to_string()), rest)
    } else {
        (None, equation)
    };
    let tokens = tokenize(rhs)?;
    if tokens.is_empty() {
        return Err(CoreError::ExpressionError("empty expression".to_string()));
    }
    let mut parser = Parser {
        tokens: &tokens,
        pos: 0,
        vars,
    };
    let value = parser.parse_expr(0)?;
    if parser.pos != tokens.len() {
        return Err(CoreError::ExpressionError(
            "trailing tokens in expression".to_string(),
        ));
    }
    Ok(SolvedEquation {
        target_variable,
        value,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evaluates_feedback_divider_and_transcendental_equations() {
        let mut vars = BTreeMap::new();
        vars.insert("R_bot".to_string(), 10_000.0);
        vars.insert("V_out".to_string(), 3.3);
        vars.insert("V_ref".to_string(), 0.8);

        let solved = evaluate_parametric_equation("R_top = R_bot * (V_out / V_ref - 1)", &vars)
            .expect("equation");
        assert_eq!(solved.target_variable.as_deref(), Some("R_top"));
        assert!((solved.value - 31_250.0).abs() < 1e-9);

        let fc = evaluate_parametric_equation("1 / (2 * pi * sqrt(10u * 100n))", &vars)
            .expect("resonant freq");
        assert!((fc.value - 159_154.943_091_895_35).abs() < 1e-6);

        let db = evaluate_parametric_equation("20 * log10(100) + 2^3", &vars).expect("db");
        assert!((db.value - 48.0).abs() < 1e-9);
    }
}
