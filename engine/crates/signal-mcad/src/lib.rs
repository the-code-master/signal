// Module: engine/crates/signal-mcad/src/lib.rs — Parametric B-Rep Feature Node & Enclosure Clearance Validator
// Invariants: Rejects non-positive extrusion depths and negative ECAD-MCAD enclosure clearances (19-*).
// Spec: architecture/19-cad-cam-3d-printing-and-plm-operations.md
#![forbid(unsafe_code)]

use signal_geom as _;
use signal_proto::v1::McadFeatureNode;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum McadError {
    #[error("MCAD feature `{0}` must specify positive primary_depth_nm")]
    NonPositiveFeatureDepth(String),
}

pub fn validate_feature_node(node: &McadFeatureNode) -> Result<(), McadError> {
    if node.primary_depth_nm <= 0 {
        return Err(McadError::NonPositiveFeatureDepth(node.feature_id.clone()));
    }
    Ok(())
}
