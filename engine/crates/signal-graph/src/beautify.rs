// Module: engine/crates/signal-graph/src/beautify.rs — Sugiyama Layer Rank, Barycenter Crossing Minimization & A* Untangler
// Invariants: Deterministic topological rank + barycenter sweep + Manhattan A* obstacle router (`FEAT-001`).
// Spec: architecture/01-product-vision-and-features.md, architecture/17-extended-feature-specifications-ui-and-rust.md
use signal_geom::{BBox2Nm, Point2Nm};
use signal_proto::v1::{
    BeautifySchematicResponse, GraphMutationKind, GraphMutationOp, Point2dNm, SubprojectDomain,
};
use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap};

#[derive(Debug, Clone)]
pub struct SchematicBlock {
    pub entity_id: String,
    pub position_nm: Point2Nm,
}

/// Counts pairwise inversions (crossings) between two adjacent Sugiyama layers.
pub fn count_two_layer_crossings(
    upper_order: &[String],
    lower_order: &[String],
    edges: &[(String, String)],
) -> u32 {
    let up_pos: BTreeMap<&str, usize> = upper_order
        .iter()
        .enumerate()
        .map(|(i, s)| (s.as_str(), i))
        .collect();
    let lo_pos: BTreeMap<&str, usize> = lower_order
        .iter()
        .enumerate()
        .map(|(i, s)| (s.as_str(), i))
        .collect();

    let mut pairs: Vec<(usize, usize)> = edges
        .iter()
        .filter_map(|(u, v)| Some((*up_pos.get(u.as_str())?, *lo_pos.get(v.as_str())?)))
        .collect();
    pairs.sort_unstable();

    let mut crossings = 0_u32;
    for i in 0..pairs.len() {
        for j in (i + 1)..pairs.len() {
            if pairs[i].0 < pairs[j].0 && pairs[i].1 > pairs[j].1 {
                crossings += 1;
            }
        }
    }
    crossings
}

/// Runs Sugiyama topological layer assignment + 4-pass barycenter crossing reduction (`FEAT-001`).
pub fn beautify_schematic_layout(
    blocks: &[SchematicBlock],
    directed_edges: &[(String, String)],
    grid_pitch_nm: i64,
) -> BeautifySchematicResponse {
    let pitch = grid_pitch_nm.max(100_000);
    let mut layer_of: BTreeMap<String, usize> = BTreeMap::new();
    for b in blocks {
        layer_of.insert(b.entity_id.clone(), 0);
    }

    // Longest-path DAG rank bounded to N passes (cycle-safe)
    for _ in 0..blocks.len() {
        let mut changed = false;
        for (src, dst) in directed_edges {
            if src == dst {
                continue;
            }
            let u_rank = layer_of.get(src).copied().unwrap_or(0);
            let v_rank = layer_of.get(dst).copied().unwrap_or(0);
            if v_rank <= u_rank && u_rank + 1 < blocks.len() {
                layer_of.insert(dst.clone(), u_rank + 1);
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }

    let max_layer = layer_of.values().copied().max().unwrap_or(0);
    let y_map: BTreeMap<&str, i64> = blocks
        .iter()
        .map(|b| (b.entity_id.as_str(), b.position_nm.y.0))
        .collect();

    let mut layers: Vec<Vec<String>> = vec![Vec::new(); max_layer + 1];
    for (id, &lyr) in &layer_of {
        layers[lyr].push(id.clone());
    }
    for lyr in &mut layers {
        lyr.sort_by_key(|id| (y_map.get(id.as_str()).copied().unwrap_or(0), id.clone()));
    }

    let total_crossings = |lyrs: &[Vec<String>]| -> u32 {
        let mut sum = 0;
        for l in 0..lyrs.len().saturating_sub(1) {
            sum += count_two_layer_crossings(&lyrs[l], &lyrs[l + 1], directed_edges);
        }
        sum
    };

    let before = total_crossings(&layers);

    // Downward + upward barycenter sweeps
    for _ in 0..4 {
        for l in 1..layers.len() {
            let prev = layers[l - 1].clone();
            reorder_layer_barycenter(&mut layers[l], &prev, directed_edges, true);
        }
        for l in (0..layers.len().saturating_sub(1)).rev() {
            let next = layers[l + 1].clone();
            reorder_layer_barycenter(&mut layers[l], &next, directed_edges, false);
        }
    }

    let after = total_crossings(&layers);
    let col_spacing = pitch * 10;
    let row_spacing = pitch * 6;
    let mut ops = Vec::with_capacity(blocks.len());

    for (col, lyr) in layers.iter().enumerate() {
        for (row, id) in lyr.iter().enumerate() {
            ops.push(GraphMutationOp {
                op_id: format!("beautify_move_{id}"),
                kind: GraphMutationKind::MoveInstance as i32,
                entity_id: id.clone(),
                domain: SubprojectDomain::Schematic as i32,
                position_nm: Some(Point2dNm {
                    x_nm: (col as i64) * col_spacing,
                    y_nm: (row as i64) * row_spacing,
                }),
                trace_width_nm: 0,
                net_name: String::new(),
                param_key: String::new(),
                param_si_value: 0,
            });
        }
    }

    BeautifySchematicResponse {
        crossing_count_before: before,
        crossing_count_after: after,
        proposed_ops: ops,
    }
}

fn reorder_layer_barycenter(
    target: &mut [String],
    fixed: &[String],
    edges: &[(String, String)],
    fixed_is_upper: bool,
) {
    let fixed_pos: BTreeMap<&str, usize> = fixed
        .iter()
        .enumerate()
        .map(|(i, s)| (s.as_str(), i))
        .collect();
    let mut scored: Vec<(i64, usize, String)> = target
        .iter()
        .enumerate()
        .map(|(orig_idx, node)| {
            let mut sum = 0_usize;
            let mut cnt = 0_usize;
            for (u, v) in edges {
                if fixed_is_upper && v == node {
                    if let Some(&p) = fixed_pos.get(u.as_str()) {
                        sum += p * 1000;
                        cnt += 1;
                    }
                } else if !fixed_is_upper
                    && u == node
                    && let Some(&p) = fixed_pos.get(v.as_str())
                {
                    sum += p * 1000;
                    cnt += 1;
                }
            }
            let bary = sum.checked_div(cnt).unwrap_or(orig_idx * 1000) as i64;
            (bary, orig_idx, node.clone())
        })
        .collect();
    scored.sort();
    for (dst, (_, _, node)) in target.iter_mut().zip(scored) {
        *dst = node;
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct AStarState {
    f_cost: i64,
    g_cost: i64,
    pt: (i64, i64),
    dir: u8,
}

impl Ord for AStarState {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .f_cost
            .cmp(&self.f_cost)
            .then_with(|| self.pt.cmp(&other.pt))
    }
}

impl PartialOrd for AStarState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Grid-snapped A* orthogonal wire router with bend penalties avoiding obstacle bounding boxes (`FEAT-001`, `FEAT-100`).
pub fn route_orthogonal_wire_astar(
    start: Point2Nm,
    goal: Point2Nm,
    obstacles: &[BBox2Nm],
    grid_step_nm: i64,
) -> Vec<Point2Nm> {
    let step = grid_step_nm.max(1);
    let snap = |v: i64| (v / step) * step;
    let s = (snap(start.x.0), snap(start.y.0));
    let g = (snap(goal.x.0), snap(goal.y.0));
    if s == g {
        return vec![Point2Nm::new(s.0, s.1)];
    }

    let min_x = s.0.min(g.0) - step * 8;
    let max_x = s.0.max(g.0) + step * 8;
    let min_y = s.1.min(g.1) - step * 8;
    let max_y = s.1.max(g.1) + step * 8;

    type GridDirKey = ((i64, i64), u8);
    let mut open = BinaryHeap::new();
    let mut best_g: BTreeMap<GridDirKey, i64> = BTreeMap::new();
    let mut parent: BTreeMap<GridDirKey, GridDirKey> = BTreeMap::new();
    let mut closed: BTreeSet<GridDirKey> = BTreeSet::new();

    let h = |p: (i64, i64)| (p.0 - g.0).abs() + (p.1 - g.1).abs();
    open.push(AStarState {
        f_cost: h(s),
        g_cost: 0,
        pt: s,
        dir: 0,
    });
    best_g.insert((s, 0), 0);

    let dirs: [(i64, i64, u8); 4] = [(step, 0, 1), (-step, 0, 2), (0, step, 3), (0, -step, 4)];
    let mut goal_state = None;
    let mut expansions = 0_usize;

    while let Some(cur) = open.pop() {
        if cur.pt == g {
            goal_state = Some((cur.pt, cur.dir));
            break;
        }
        if !closed.insert((cur.pt, cur.dir)) || expansions > 8_000 {
            continue;
        }
        expansions += 1;

        for (dx, dy, ndir) in dirs {
            let nxt = (cur.pt.0 + dx, cur.pt.1 + dy);
            if nxt.0 < min_x || nxt.0 > max_x || nxt.1 < min_y || nxt.1 > max_y {
                continue;
            }
            let npt = Point2Nm::new(nxt.0, nxt.1);
            if nxt != g && obstacles.iter().any(|ob| ob.contains_point(npt)) {
                continue;
            }
            let bend_penalty = if cur.dir != 0 && cur.dir != ndir {
                step / 2
            } else {
                0
            };
            let ng = cur.g_cost + step + bend_penalty;
            if ng < best_g.get(&(nxt, ndir)).copied().unwrap_or(i64::MAX) {
                best_g.insert((nxt, ndir), ng);
                parent.insert((nxt, ndir), (cur.pt, cur.dir));
                open.push(AStarState {
                    f_cost: ng + h(nxt),
                    g_cost: ng,
                    pt: nxt,
                    dir: ndir,
                });
            }
        }
    }

    let Some(mut st) = goal_state else {
        return vec![start, Point2Nm::new(goal.x.0, start.y.0), goal];
    };
    let mut path = vec![Point2Nm::new(st.0.0, st.0.1)];
    while let Some(&prev) = parent.get(&st) {
        path.push(Point2Nm::new(prev.0.0, prev.0.1));
        st = prev;
    }
    path.reverse();
    path
}
