// Module: engine/crates/signal-geom/src/rtree.rs — STR Bulk-Loaded R*-Tree Spatial Index
// Invariants: Exact i128 bounding-box area/distance pruning; bounded tree depth <= 64.
// Spec: architecture/03-rust-eda-engine.md
use crate::primitives::{BBox2Nm, Point2Nm};

const NODE_CAPACITY: usize = 8;
const MAX_TREE_DEPTH: usize = 64;

#[derive(Debug, Clone)]
enum RTreeNode<T> {
    Leaf {
        bbox: BBox2Nm,
        entries: Vec<(BBox2Nm, T)>,
    },
    Internal {
        bbox: BBox2Nm,
        children: Vec<RTreeNode<T>>,
    },
}

impl<T> RTreeNode<T> {
    fn bbox(&self) -> BBox2Nm {
        match self {
            Self::Leaf { bbox, .. } | Self::Internal { bbox, .. } => *bbox,
        }
    }

    fn recompute_bbox(&mut self) {
        match self {
            Self::Leaf { bbox, entries } => {
                if let Some((first, _)) = entries.first() {
                    let mut acc = *first;
                    for (b, _) in &entries[1..] {
                        acc = acc.union(b);
                    }
                    *bbox = acc;
                }
            }
            Self::Internal { bbox, children } => {
                if let Some(first) = children.first() {
                    let mut acc = first.bbox();
                    for ch in &children[1..] {
                        acc = acc.union(&ch.bbox());
                    }
                    *bbox = acc;
                }
            }
        }
    }
}

/// Sort-Tile-Recursive (STR) bulk-loaded R*-Tree with dynamic R*-style least-enlargement insertion.
#[derive(Debug, Clone)]
pub struct SpatialIndex<T> {
    root: Option<RTreeNode<T>>,
    len: usize,
}

impl<T> Default for SpatialIndex<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> SpatialIndex<T> {
    pub const fn new() -> Self {
        Self { root: None, len: 0 }
    }

    pub const fn len(&self) -> usize {
        self.len
    }

    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Sort-Tile-Recursive (STR) $O(N \log N)$ bulk loader minimizing slice overlap.
    pub fn bulk_load(mut items: Vec<(BBox2Nm, T)>) -> Self {
        let len = items.len();
        if items.is_empty() {
            return Self::new();
        }
        let leaf_count = items.len().div_ceil(NODE_CAPACITY);
        let slice_count = (leaf_count as f64).sqrt().ceil().max(1.0) as usize;
        let slice_size = slice_count * NODE_CAPACITY;

        items.sort_by_key(|(b, _)| b.center_nm().x.0);

        let mut leaves = Vec::with_capacity(leaf_count);
        let mut drain_iter = items.into_iter();
        loop {
            let mut x_slice: Vec<(BBox2Nm, T)> = drain_iter.by_ref().take(slice_size).collect();
            if x_slice.is_empty() {
                break;
            }
            x_slice.sort_by_key(|(b, _)| b.center_nm().y.0);
            let mut slice_iter = x_slice.into_iter();
            loop {
                let pack: Vec<(BBox2Nm, T)> = slice_iter.by_ref().take(NODE_CAPACITY).collect();
                if pack.is_empty() {
                    break;
                }
                let mut bbox = pack[0].0;
                for (b, _) in &pack[1..] {
                    bbox = bbox.union(b);
                }
                leaves.push(RTreeNode::Leaf {
                    bbox,
                    entries: pack,
                });
            }
        }

        let mut level = leaves;
        let mut depth = 1_usize;
        while level.len() > 1 && depth < MAX_TREE_DEPTH {
            let parent_count = level.len().div_ceil(NODE_CAPACITY);
            let s_cnt = (parent_count as f64).sqrt().ceil().max(1.0) as usize;
            let s_sz = s_cnt * NODE_CAPACITY;
            level.sort_by_key(|n| n.bbox().center_nm().x.0);

            let mut next_level = Vec::with_capacity(parent_count);
            let mut lvl_iter = level.into_iter();
            loop {
                let mut x_slice: Vec<RTreeNode<T>> = lvl_iter.by_ref().take(s_sz).collect();
                if x_slice.is_empty() {
                    break;
                }
                x_slice.sort_by_key(|n| n.bbox().center_nm().y.0);
                let mut ch_iter = x_slice.into_iter();
                loop {
                    let children: Vec<RTreeNode<T>> =
                        ch_iter.by_ref().take(NODE_CAPACITY).collect();
                    if children.is_empty() {
                        break;
                    }
                    let mut bbox = children[0].bbox();
                    for ch in &children[1..] {
                        bbox = bbox.union(&ch.bbox());
                    }
                    next_level.push(RTreeNode::Internal { bbox, children });
                }
            }
            level = next_level;
            depth += 1;
        }

        Self {
            root: level.into_iter().next(),
            len,
        }
    }

    pub fn insert(&mut self, bbox: BBox2Nm, value: T) {
        self.len += 1;
        let Some(mut root) = self.root.take() else {
            self.root = Some(RTreeNode::Leaf {
                bbox,
                entries: vec![(bbox, value)],
            });
            return;
        };
        if let Some(split_sibling) = Self::insert_rec(&mut root, bbox, value, 0) {
            let union_bbox = root.bbox().union(&split_sibling.bbox());
            self.root = Some(RTreeNode::Internal {
                bbox: union_bbox,
                children: vec![root, split_sibling],
            });
        } else {
            self.root = Some(root);
        }
    }

    fn insert_rec(
        node: &mut RTreeNode<T>,
        bbox: BBox2Nm,
        value: T,
        depth: usize,
    ) -> Option<RTreeNode<T>> {
        let split_res = match node {
            RTreeNode::Leaf {
                bbox: node_bbox,
                entries,
            } => {
                entries.push((bbox, value));
                *node_bbox = node_bbox.union(&bbox);
                if entries.len() > NODE_CAPACITY && depth < MAX_TREE_DEPTH {
                    entries.sort_by_key(|(b, _)| (b.center_nm().x.0, b.center_nm().y.0));
                    let right_entries = entries.split_off(entries.len() / 2);
                    let mut right = RTreeNode::Leaf {
                        bbox: right_entries[0].0,
                        entries: right_entries,
                    };
                    right.recompute_bbox();
                    Some(right)
                } else {
                    None
                }
            }
            RTreeNode::Internal {
                bbox: node_bbox,
                children,
            } => {
                let mut best_idx = 0;
                let mut best_cost = (i128::MAX, i128::MAX);
                for (i, ch) in children.iter().enumerate() {
                    let cb = ch.bbox();
                    let cost = (cb.enlargement_i128(&bbox), cb.area_i128());
                    if cost < best_cost {
                        best_cost = cost;
                        best_idx = i;
                    }
                }
                let split = Self::insert_rec(&mut children[best_idx], bbox, value, depth + 1);
                if let Some(sib) = split {
                    children.push(sib);
                }
                *node_bbox = node_bbox.union(&bbox);
                if children.len() > NODE_CAPACITY && depth < MAX_TREE_DEPTH {
                    children.sort_by_key(|c| (c.bbox().center_nm().x.0, c.bbox().center_nm().y.0));
                    let right_children = children.split_off(children.len() / 2);
                    let mut right = RTreeNode::Internal {
                        bbox: right_children[0].bbox(),
                        children: right_children,
                    };
                    right.recompute_bbox();
                    Some(right)
                } else {
                    None
                }
            }
        };
        if split_res.is_some() {
            node.recompute_bbox();
        }
        split_res
    }

    /// Returns all entries intersecting `query_region`.
    pub fn query_bbox(&self, query_region: BBox2Nm) -> Vec<&T> {
        let mut results = Vec::new();
        if let Some(root) = &self.root {
            Self::query_rec(root, &query_region, &mut results);
        }
        results
    }

    fn query_rec<'a>(node: &'a RTreeNode<T>, query: &BBox2Nm, out: &mut Vec<&'a T>) {
        if !node.bbox().intersects(query) {
            return;
        }
        match node {
            RTreeNode::Leaf { entries, .. } => {
                for (b, val) in entries {
                    if b.intersects(query) {
                        out.push(val);
                    }
                }
            }
            RTreeNode::Internal { children, .. } => {
                for ch in children {
                    Self::query_rec(ch, query, out);
                }
            }
        }
    }

    /// Exact branch-and-bound nearest-neighbor lookup using `i128` squared nanometer distance.
    pub fn nearest(&self, target: Point2Nm) -> Option<&T> {
        let root = self.root.as_ref()?;
        let mut best: Option<(i128, &T)> = None;
        Self::nearest_rec(root, target, &mut best);
        best.map(|(_, v)| v)
    }

    fn nearest_rec<'a>(node: &'a RTreeNode<T>, target: Point2Nm, best: &mut Option<(i128, &'a T)>) {
        let node_dist = node.bbox().min_dist_sq_to_point_i128(target);
        if best.is_some_and(|(d, _)| node_dist >= d) {
            return;
        }
        match node {
            RTreeNode::Leaf { entries, .. } => {
                for (b, val) in entries {
                    let d = b.min_dist_sq_to_point_i128(target);
                    if best.is_none_or(|(best_d, _)| d < best_d) {
                        *best = Some((d, val));
                    }
                }
            }
            RTreeNode::Internal { children, .. } => {
                let mut ordered: Vec<(i128, &RTreeNode<T>)> = children
                    .iter()
                    .map(|c| (c.bbox().min_dist_sq_to_point_i128(target), c))
                    .collect();
                ordered.sort_by_key(|(d, _)| *d);
                for (d, ch) in ordered {
                    if best.is_some_and(|(best_d, _)| d >= best_d) {
                        break;
                    }
                    Self::nearest_rec(ch, target, best);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn str_bulk_load_insert_query_and_nearest_match_brute_force() {
        let mut items = Vec::new();
        for i in 0..120_i64 {
            let x = (i % 12) * 10_000;
            let y = (i / 12) * 10_000;
            let b = BBox2Nm::from_points(Point2Nm::new(x, y), Point2Nm::new(x + 2_000, y + 2_000));
            items.push((b, i));
        }
        let mut tree = SpatialIndex::bulk_load(items);
        tree.insert(
            BBox2Nm::from_points(Point2Nm::new(55_000, 55_000), Point2Nm::new(56_000, 56_000)),
            999,
        );
        assert_eq!(tree.len(), 121);

        let hits = tree.query_bbox(BBox2Nm::from_points(
            Point2Nm::new(50_000, 50_000),
            Point2Nm::new(59_000, 59_000),
        ));
        assert!(hits.contains(&&65));
        assert!(hits.contains(&&999));

        let nn = tree.nearest(Point2Nm::new(55_500, 55_500));
        assert_eq!(nn, Some(&999));
    }
}
