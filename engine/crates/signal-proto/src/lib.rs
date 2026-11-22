// Module: engine/crates/signal-proto/src/lib.rs — Canonical Generated Signal v1 Protobuf & Tonic Types
// Invariants: #![forbid(unsafe_code)]; single-build inclusion from OUT_DIR; verified >2^53-1 JSON round-trips (F-143, F-144).
// Spec: architecture/10-api-and-data-contracts.md
#![forbid(unsafe_code)]

use workspace_hack as _;

pub mod bigint_serde;

#[allow(clippy::all, missing_debug_implementations)]
pub mod v1 {
    tonic::include_proto!("signal.v1");
}

#[cfg(test)]
mod tests {
    use super::v1::{ApplyTransactionRequest, FencingAuthority};
    use prost::Message;
    use std::error::Error;
    use std::fs;
    use std::path::PathBuf;

    fn fixtures_dir() -> Result<PathBuf, Box<dyn Error>> {
        let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../contracts/fixtures");
        Ok(dir)
    }

    #[test]
    fn roundtrips_fencing_authority_beyond_ieee754_safe_int() -> Result<(), Box<dyn Error>> {
        let raw = fs::read_to_string(fixtures_dir()?.join("fencing_authority.fixture.json"))?;
        let parsed: FencingAuthority = serde_json::from_str(&raw)?;
        assert_eq!(parsed.fencing_epoch, 9_007_199_254_740_993_u64);
        assert_eq!(parsed.issued_at_utc_ms, 1_795_324_800_000_i64);

        let reserialized = serde_json::to_value(&parsed)?;
        assert_eq!(
            reserialized.get("fencing_epoch").and_then(|v| v.as_str()),
            Some("9007199254740993")
        );

        let wire = parsed.encode_to_vec();
        let decoded = FencingAuthority::decode(wire.as_slice())?;
        assert_eq!(decoded.fencing_epoch, 9_007_199_254_740_993_u64);
        Ok(())
    }

    #[test]
    fn roundtrips_graph_mutation_tx_signed_and_unsigned_64bit_si_fields()
    -> Result<(), Box<dyn Error>> {
        let raw = fs::read_to_string(fixtures_dir()?.join("graph_mutation_tx.fixture.json"))?;
        let tx: ApplyTransactionRequest = serde_json::from_str(&raw)?;
        assert_eq!(tx.expected_base_revision, 9_007_199_254_740_999_u64);
        assert_eq!(tx.operations.len(), 1);

        let op = &tx.operations[0];
        let pos = op.position_nm.as_ref().ok_or("missing position_nm")?;
        assert_eq!(pos.x_nm, 9_007_199_254_740_993_i64);
        assert_eq!(pos.y_nm, -9_007_199_254_740_995_i64);
        assert_eq!(op.param_si_value, 9_007_199_254_741_001_i64);

        let reserialized = serde_json::to_string(&tx)?;
        assert!(reserialized.contains("\"9007199254740993\""));
        assert!(reserialized.contains("\"-9007199254740995\""));
        assert!(reserialized.contains("\"9007199254741001\""));
        Ok(())
    }
}
