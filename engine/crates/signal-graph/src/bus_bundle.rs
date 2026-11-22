// Module: engine/crates/signal-graph/src/bus_bundle.rs — Typed Bus Slice Parser & Protocol Bundle Expander (FEAT-002)
// Invariants: Preserves exact bit-slice ordering (`DATA[31:0]` vs `DATA[0:31]`); caps slice width <= 1024.
// Spec: architecture/01-product-vision-and-features.md, architecture/17-extended-feature-specifications-ui-and-rust.md
use crate::GraphError;
use signal_proto::v1::BusSliceDescriptor;

const MAX_BUS_SLICE_WIDTH: usize = 1024;

/// Parses a bus slice expression such as `"DATA[31:0]"`, `"LANE[0:7:2]"`, or `"CLK,RST_N"` (`FEAT-002`).
pub fn parse_bus_slice(
    bundle_id: &str,
    protocol_kind: &str,
    slice_expression: &str,
) -> Result<BusSliceDescriptor, GraphError> {
    let mut expanded = Vec::new();
    for part in slice_expression.split(',') {
        let token = part.trim();
        if token.is_empty() {
            return Err(GraphError::InvalidCommandOrSlice(
                "empty bus slice segment".to_string(),
            ));
        }
        if let Some((prefix, bracket_rest)) = token.split_once('[') {
            let inner = bracket_rest.strip_suffix(']').ok_or_else(|| {
                GraphError::InvalidCommandOrSlice(format!("unclosed `[` in `{token}`"))
            })?;
            let base = prefix.trim();
            if base.is_empty() {
                return Err(GraphError::InvalidCommandOrSlice(
                    "missing bus prefix".to_string(),
                ));
            }
            let nums: Vec<i32> = inner
                .split(':')
                .map(|s| {
                    s.trim()
                        .parse::<i32>()
                        .map_err(|_| GraphError::InvalidCommandOrSlice(token.to_string()))
                })
                .collect::<Result<_, _>>()?;
            match nums.as_slice() {
                [single] => {
                    if *single < 0 {
                        return Err(GraphError::InvalidCommandOrSlice(token.to_string()));
                    }
                    expanded.push(format!("{base}[{single}]"));
                }
                [msb, lsb] => push_range(&mut expanded, base, *msb, *lsb, 1)?,
                [msb, lsb, step] => {
                    if *step <= 0 {
                        return Err(GraphError::InvalidCommandOrSlice(
                            "bus step <= 0".to_string(),
                        ));
                    }
                    push_range(&mut expanded, base, *msb, *lsb, *step)?;
                }
                _ => return Err(GraphError::InvalidCommandOrSlice(token.to_string())),
            }
        } else {
            expanded.push(token.to_string());
        }
    }

    Ok(BusSliceDescriptor {
        bundle_id: bundle_id.to_string(),
        protocol_kind: protocol_kind.to_string(),
        slice_expression: slice_expression.to_string(),
        expanded_member_nets: expanded,
    })
}

fn push_range(
    out: &mut Vec<String>,
    base: &str,
    start: i32,
    end: i32,
    step: i32,
) -> Result<(), GraphError> {
    if start < 0 || end < 0 {
        return Err(GraphError::InvalidCommandOrSlice(
            "negative bus bit index".to_string(),
        ));
    }
    let width = ((start - end).unsigned_abs() as usize) / (step as usize) + 1;
    if out.len() + width > MAX_BUS_SLICE_WIDTH {
        return Err(GraphError::InvalidCommandOrSlice(
            "bus width exceeds 1024 cap".to_string(),
        ));
    }
    if start >= end {
        let mut idx = start;
        while idx >= end {
            out.push(format!("{base}[{idx}]"));
            idx -= step;
        }
    } else {
        let mut idx = start;
        while idx <= end {
            out.push(format!("{base}[{idx}]"));
            idx += step;
        }
    }
    Ok(())
}

/// Expands canonical interface protocol bundles (`AXI4_LITE`, `SPI`, `I2C`, `USB2_DIFF`) (`FEAT-002`).
pub fn expand_protocol_bundle(
    bundle_id: &str,
    prefix: &str,
    protocol: &str,
) -> Result<BusSliceDescriptor, GraphError> {
    let p = if prefix.is_empty() { "" } else { prefix };
    let sep = if prefix.is_empty() { "" } else { "_" };
    let spec = match protocol.trim().to_ascii_uppercase().as_str() {
        "AXI4_LITE" => format!(
            "{p}{sep}AWADDR[31:0],{p}{sep}AWVALID,{p}{sep}AWREADY,{p}{sep}WDATA[31:0],{p}{sep}WSTRB[3:0],{p}{sep}WVALID,{p}{sep}WREADY,{p}{sep}BRESP[1:0],{p}{sep}BVALID,{p}{sep}BREADY,{p}{sep}ARADDR[31:0],{p}{sep}ARVALID,{p}{sep}ARREADY,{p}{sep}RDATA[31:0],{p}{sep}RRESP[1:0],{p}{sep}RVALID,{p}{sep}RREADY"
        ),
        "SPI" => format!("{p}{sep}SCLK,{p}{sep}MOSI,{p}{sep}MISO,{p}{sep}CS_N"),
        "I2C" => format!("{p}{sep}SCL,{p}{sep}SDA"),
        "USB2_DIFF" => format!("{p}{sep}DP,{p}{sep}DN"),
        other => {
            return Err(GraphError::InvalidCommandOrSlice(format!(
                "unsupported protocol bundle `{other}`"
            )));
        }
    };
    parse_bus_slice(bundle_id, protocol, &spec)
}
