// Module: engine/crates/signal-proto/src/bigint_serde.rs — Exact 64-Bit Integer <-> JSON String Serde Helpers
// Invariants: Preserves full 64-bit precision (> 2^53 - 1) across Rust, Python int, and TypeScript bigint (F-144).
// Spec: architecture/10-api-and-data-contracts.md
use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

pub mod i64_str {
    use super::*;

    pub fn serialize<S: Serializer>(value: &i64, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&value.to_string())
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<i64, D::Error> {
        struct I64Visitor;
        impl Visitor<'_> for I64Visitor {
            type Value = i64;
            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str("a base-10 string or integer representing i64")
            }
            fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
                v.parse::<i64>().map_err(de::Error::custom)
            }
            fn visit_i64<E: de::Error>(self, v: i64) -> Result<Self::Value, E> {
                Ok(v)
            }
            fn visit_u64<E: de::Error>(self, v: u64) -> Result<Self::Value, E> {
                i64::try_from(v).map_err(de::Error::custom)
            }
        }
        deserializer.deserialize_any(I64Visitor)
    }
}

pub mod u64_str {
    use super::*;

    pub fn serialize<S: Serializer>(value: &u64, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&value.to_string())
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<u64, D::Error> {
        struct U64Visitor;
        impl Visitor<'_> for U64Visitor {
            type Value = u64;
            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str("a base-10 string or unsigned integer representing u64")
            }
            fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
                v.parse::<u64>().map_err(de::Error::custom)
            }
            fn visit_u64<E: de::Error>(self, v: u64) -> Result<Self::Value, E> {
                Ok(v)
            }
        }
        deserializer.deserialize_any(U64Visitor)
    }
}

pub mod vec_i64_str {
    use super::*;

    pub fn serialize<S: Serializer>(values: &[i64], serializer: S) -> Result<S::Ok, S::Error> {
        let strings: Vec<String> = values.iter().map(ToString::to_string).collect();
        strings.serialize(serializer)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Vec<i64>, D::Error> {
        let strings = Vec::<String>::deserialize(deserializer)?;
        strings
            .into_iter()
            .map(|s| s.parse::<i64>().map_err(de::Error::custom))
            .collect()
    }
}

pub mod vec_u64_str {
    use super::*;

    pub fn serialize<S: Serializer>(values: &[u64], serializer: S) -> Result<S::Ok, S::Error> {
        let strings: Vec<String> = values.iter().map(ToString::to_string).collect();
        strings.serialize(serializer)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Vec<u64>, D::Error> {
        let strings = Vec::<String>::deserialize(deserializer)?;
        strings
            .into_iter()
            .map(|s| s.parse::<u64>().map_err(de::Error::custom))
            .collect()
    }
}
