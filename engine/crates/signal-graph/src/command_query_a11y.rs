// Module: engine/crates/signal-graph/src/command_query_a11y.rs — Omni-Palette CLI, Topological Query & WCAG A11y Tree
// Invariants: Deterministic CLI parser (FEAT-091), boolean predicate query (FEAT-099), WCAG spatial A11y + A* wire (FEAT-100).
// Spec: architecture/01-product-vision-and-features.md, architecture/17-extended-feature-specifications-ui-and-rust.md
use crate::GraphError;
use crate::beautify::route_orthogonal_wire_astar;
use crate::cow_arena::CoWGraphSnapshot;
use signal_core::parse_spice_si;
use signal_geom::{BBox2Nm, Point2Nm};
use signal_proto::v1::{
    A11ySpatialNode, EvaluateSelectionQueryResponse, ExecuteCommandDslResponse,
    GetAccessibilityTreeResponse, GraphMutationKind, GraphMutationOp, Point2dNm, SubprojectDomain,
};
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct QueryableEntity {
    pub entity_id: String,
    pub component_type: String,
    pub numeric_attrs: BTreeMap<String, f64>,
}

fn parse_coord_pair(raw: &str) -> Result<(i64, i64), GraphError> {
    let trimmed = raw.trim().trim_start_matches('(').trim_end_matches(')');
    let (xs, ys) = trimmed
        .split_once(',')
        .ok_or_else(|| GraphError::InvalidCommandOrSlice(format!("invalid coords `{raw}`")))?;
    let x = xs
        .trim()
        .parse::<i64>()
        .map_err(|_| GraphError::InvalidCommandOrSlice(raw.to_string()))?;
    let y = ys
        .trim()
        .parse::<i64>()
        .map_err(|_| GraphError::InvalidCommandOrSlice(raw.to_string()))?;
    Ok((x, y))
}

/// Deterministic Omni-Palette CLI parser (`place R 10k at (100,200)` / `connect R1.1 VCC`, `FEAT-091`).
pub fn execute_omni_command_dsl(
    command_line: &str,
) -> Result<ExecuteCommandDslResponse, GraphError> {
    let trimmed = command_line.trim();
    if trimmed.is_empty() {
        return Err(GraphError::InvalidCommandOrSlice(
            "empty CLI command".to_string(),
        ));
    }

    // Normalize whitespace inside `(x, y)` before splitting tokens
    let mut compact = String::with_capacity(trimmed.len());
    let mut in_parens = false;
    for ch in trimmed.chars() {
        if ch == '(' {
            in_parens = true;
        } else if ch == ')' {
            in_parens = false;
        }
        if !(in_parens && ch.is_ascii_whitespace()) {
            compact.push(ch);
        }
    }
    let parts: Vec<&str> = compact.split_whitespace().collect();

    match parts.as_slice() {
        ["place", kind_tok, val_tok, "at", coords] => {
            let (x_nm, y_nm) = parse_coord_pair(coords)?;
            let si_val = (parse_spice_si(val_tok)
                .map_err(|e| GraphError::InvalidCommandOrSlice(e.to_string()))?
                * 1000.0)
                .round() as i64;
            let op = GraphMutationOp {
                op_id: format!("cli_place_{kind_tok}_{x_nm}_{y_nm}"),
                kind: GraphMutationKind::PlaceInstance as i32,
                entity_id: format!("{kind_tok}_NEW"),
                domain: SubprojectDomain::Schematic as i32,
                position_nm: Some(Point2dNm { x_nm, y_nm }),
                trace_width_nm: 0,
                net_name: String::new(),
                param_key: (*kind_tok).to_string(),
                param_si_value: si_val,
            };
            Ok(ExecuteCommandDslResponse {
                accepted: true,
                normalized_action: format!("PLACE {kind_tok} value={val_tok} AT ({x_nm},{y_nm})"),
                generated_ops: vec![op],
            })
        }
        ["connect", pin_ref, net_name] => {
            let op = GraphMutationOp {
                op_id: format!("cli_conn_{pin_ref}_{net_name}"),
                kind: GraphMutationKind::ConnectNet as i32,
                entity_id: (*pin_ref).to_string(),
                domain: SubprojectDomain::Schematic as i32,
                position_nm: None,
                trace_width_nm: 0,
                net_name: (*net_name).to_string(),
                param_key: String::new(),
                param_si_value: 0,
            };
            Ok(ExecuteCommandDslResponse {
                accepted: true,
                normalized_action: format!("CONNECT {pin_ref} -> {net_name}"),
                generated_ops: vec![op],
            })
        }
        ["move", entity_id, "to", coords] => {
            let (x_nm, y_nm) = parse_coord_pair(coords)?;
            let op = GraphMutationOp {
                op_id: format!("cli_move_{entity_id}"),
                kind: GraphMutationKind::MoveInstance as i32,
                entity_id: (*entity_id).to_string(),
                domain: SubprojectDomain::Schematic as i32,
                position_nm: Some(Point2dNm { x_nm, y_nm }),
                trace_width_nm: 0,
                net_name: String::new(),
                param_key: String::new(),
                param_si_value: 0,
            };
            Ok(ExecuteCommandDslResponse {
                accepted: true,
                normalized_action: format!("MOVE {entity_id} TO ({x_nm},{y_nm})"),
                generated_ops: vec![op],
            })
        }
        ["delete", entity_id] => {
            let op = GraphMutationOp {
                op_id: format!("cli_del_{entity_id}"),
                kind: GraphMutationKind::DeleteEntity as i32,
                entity_id: (*entity_id).to_string(),
                domain: SubprojectDomain::Schematic as i32,
                ..Default::default()
            };
            Ok(ExecuteCommandDslResponse {
                accepted: true,
                normalized_action: format!("DELETE {entity_id}"),
                generated_ops: vec![op],
            })
        }
        _ => Err(GraphError::InvalidCommandOrSlice(command_line.to_string())),
    }
}

fn eval_scalar_side(expr: &str, ent: &QueryableEntity) -> Result<f64, GraphError> {
    let s = expr.trim();
    if let Some((a, b)) = s.split_once('*') {
        return Ok(eval_scalar_side(a, ent)? * eval_scalar_side(b, ent)?);
    }
    if let Some((a, b)) = s.split_once('/') {
        let denom = eval_scalar_side(b, ent)?;
        if denom.abs() < 1e-30 {
            return Err(GraphError::InvalidCommandOrSlice(
                "query division by zero".to_string(),
            ));
        }
        return Ok(eval_scalar_side(a, ent)? / denom);
    }
    if let Ok(v) = parse_spice_si(s) {
        return Ok(v);
    }
    ent.numeric_attrs
        .get(s)
        .copied()
        .ok_or_else(|| GraphError::InvalidCommandOrSlice(format!("unknown attribute `{s}`")))
}

fn eval_atomic_predicate(clause: &str, ent: &QueryableEntity) -> Result<bool, GraphError> {
    let c = clause.trim();
    if let Some(rest) = c.strip_prefix("NOT ") {
        return Ok(!eval_atomic_predicate(rest, ent)?);
    }
    if let Some((k, v)) = c.split_once(':') {
        return match k.trim() {
            "type" => Ok(ent.component_type.eq_ignore_ascii_case(v.trim())),
            "id" => Ok(ent.entity_id == v.trim()),
            other => Err(GraphError::InvalidCommandOrSlice(format!(
                "unknown key `{other}`"
            ))),
        };
    }
    for (op_str, op_kind) in [
        ("<=", 0),
        (">=", 1),
        ("!=", 2),
        ("=", 3),
        ("<", 4),
        (">", 5),
    ] {
        if let Some((lhs_s, rhs_s)) = c.split_once(op_str) {
            let lhs = eval_scalar_side(lhs_s, ent)?;
            let rhs = eval_scalar_side(rhs_s, ent)?;
            return Ok(match op_kind {
                0 => lhs <= rhs,
                1 => lhs >= rhs,
                2 => (lhs - rhs).abs() > 1e-12,
                3 => (lhs - rhs).abs() <= 1e-12,
                4 => lhs < rhs,
                _ => lhs > rhs,
            });
        }
    }
    Err(GraphError::InvalidCommandOrSlice(format!(
        "invalid predicate `{c}`"
    )))
}

fn eval_boolean_query(query: &str, ent: &QueryableEntity) -> Result<bool, GraphError> {
    for or_branch in query.split(" OR ") {
        let mut branch_ok = true;
        for and_clause in or_branch.split(" AND ") {
            if !eval_atomic_predicate(and_clause, ent)? {
                branch_ok = false;
                break;
            }
        }
        if branch_ok {
            return Ok(true);
        }
    }
    Ok(false)
}

/// Evaluates topological filter predicates such as `"type:cap AND voltage_rating < 2*net_max_v"` (`FEAT-099`).
pub fn evaluate_selection_query(
    entities: &[QueryableEntity],
    query_expression: &str,
) -> Result<EvaluateSelectionQueryResponse, GraphError> {
    let mut matched = Vec::new();
    for ent in entities {
        if eval_boolean_query(query_expression, ent)? {
            matched.push(ent.entity_id.clone());
        }
    }
    Ok(EvaluateSelectionQueryResponse {
        matched_entity_ids: matched,
        evaluated_node_count: entities.len() as u32,
    })
}

/// Builds WCAG 2.2 AA spatially-ordered screen-reader tree with netlist summaries (`FEAT-100`).
pub fn build_wcag_accessibility_tree(snapshot: &CoWGraphSnapshot) -> GetAccessibilityTreeResponse {
    let mut ents = snapshot.entities_sorted();
    ents.sort_by_key(|e| (e.position_nm.y.0, e.position_nm.x.0, e.entity_id.as_str()));

    let nodes = ents
        .into_iter()
        .map(|ent| {
            let summaries: Vec<String> = ent
                .pin_nets
                .iter()
                .map(|(pin, net)| format!("Pin {pin} connected to net {net}"))
                .collect();
            A11ySpatialNode {
                entity_id: ent.entity_id.clone(),
                aria_label: format!(
                    "{} {} at ({} nm, {} nm), {} connected pins",
                    ent.component_type,
                    ent.entity_id,
                    ent.position_nm.x.0,
                    ent.position_nm.y.0,
                    summaries.len()
                ),
                position_nm: Some(Point2dNm {
                    x_nm: ent.position_nm.x.0,
                    y_nm: ent.position_nm.y.0,
                }),
                connected_net_summaries: summaries,
            }
        })
        .collect();

    GetAccessibilityTreeResponse { nodes }
}

/// Keyboard-only WCAG point-to-point orthogonal wire router around component obstacles (`FEAT-100`).
pub fn route_keyboard_a11y_wire(
    from_pin_nm: Point2Nm,
    to_pin_nm: Point2Nm,
    obstacles: &[BBox2Nm],
    grid_step_nm: i64,
) -> Vec<Point2Nm> {
    route_orthogonal_wire_astar(from_pin_nm, to_pin_nm, obstacles, grid_step_nm)
}
