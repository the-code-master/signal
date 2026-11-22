// Module: engine/crates/signal-graph/src/constraint_matrix.rs — Hierarchical Net-Class Constraint Matrix (FEAT-095)
// Invariants: Resolves pairwise (ClassA, ClassB) max-clearance, min-width, diff-pair Z0, and HV isolation overrides.
// Spec: architecture/03-rust-eda-engine.md, architecture/17-extended-feature-specifications-ui-and-rust.md

use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NetClassSpec {
    pub class_name: String,
    pub min_width_nm: i64,
    pub base_clearance_nm: i64,
    pub target_diff_z0_mohm: Option<i64>,
    pub max_pair_skew_fs: Option<i64>,
}

/// Bulk Net-Class Constraint Matrix (`FEAT-095`) resolving symmetric `(ClassA, ClassB)` clearance & impedance rules.
#[derive(Debug, Default, Clone)]
pub struct ConstraintMatrixResolver {
    classes: BTreeMap<String, NetClassSpec>,
    pairwise_clearance_nm: BTreeMap<(String, String), i64>,
}

impl ConstraintMatrixResolver {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn upsert_net_class(&mut self, spec: NetClassSpec) {
        self.classes.insert(spec.class_name.clone(), spec);
    }

    pub fn set_pairwise_clearance_nm(&mut self, class_a: &str, class_b: &str, clearance_nm: i64) {
        let key = ordered_pair(class_a, class_b);
        self.pairwise_clearance_nm.insert(key, clearance_nm);
    }

    /// Resolves effective clearance (`nm`) between `class_a` and `class_b`:
    /// explicit pairwise matrix cell takes precedence, falling back to `max(base_a, base_b)`.
    pub fn resolve_clearance_nm(&self, class_a: &str, class_b: &str, default_nm: i64) -> i64 {
        let key = ordered_pair(class_a, class_b);
        if let Some(&explicit) = self.pairwise_clearance_nm.get(&key) {
            return explicit;
        }
        let ca = self
            .classes
            .get(class_a)
            .map_or(default_nm, |c| c.base_clearance_nm);
        let cb = self
            .classes
            .get(class_b)
            .map_or(default_nm, |c| c.base_clearance_nm);
        ca.max(cb)
    }

    pub fn class_spec(&self, class_name: &str) -> Option<&NetClassSpec> {
        self.classes.get(class_name)
    }
}

fn ordered_pair(a: &str, b: &str) -> (String, String) {
    if a <= b {
        (a.to_string(), b.to_string())
    } else {
        (b.to_string(), a.to_string())
    }
}
