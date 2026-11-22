// Module: engine/crates/signal-cam/src/lib.rs — Subtractive CNC & Additive 3D Print Parameter Guards
// Invariants: Rejects zero/negative cutter diameters or 3D-print layer heights (19-*).
// Spec: architecture/19-cad-cam-3d-printing-and-plm-operations.md
#![forbid(unsafe_code)]

use signal_proto::v1::{GenerateCncToolpathRequest, SliceBRepFor3dPrintRequest};
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum CamError {
    #[error("CNC tool diameter ({0} nm) must be strictly positive")]
    InvalidToolDiameter(i64),
    #[error("3D print layer height ({0} nm) must be between 5_000 nm and 1_000_000 nm")]
    InvalidLayerHeight(i64),
}

pub fn validate_cnc_request(req: &GenerateCncToolpathRequest) -> Result<(), CamError> {
    if req.tool_diameter_nm <= 0 {
        return Err(CamError::InvalidToolDiameter(req.tool_diameter_nm));
    }
    Ok(())
}

pub fn validate_print3d_slice_request(req: &SliceBRepFor3dPrintRequest) -> Result<(), CamError> {
    if !(5_000..=1_000_000).contains(&req.layer_height_nm) {
        return Err(CamError::InvalidLayerHeight(req.layer_height_nm));
    }
    Ok(())
}
