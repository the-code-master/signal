// Module: engine/crates/signal-proto/build.rs — Build-Time Protobuf & Serde 64-Bit String Attribute Generator
// Invariants: Compiles all signal/v1/*.proto once into OUT_DIR; attaches string-serde to every 64-bit integer field (F-143, F-144).
// Spec: architecture/10-api-and-data-contracts.md
#![forbid(unsafe_code)]

use prost_build as _;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

struct ProtoScan {
    messages: Vec<String>,
    field_attrs: Vec<(String, &'static str)>,
}

fn collect_proto_files(proto_dir: &Path) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let mut files = Vec::new();
    for entry in fs::read_dir(proto_dir)? {
        let path = entry?.path();
        if path.extension().and_then(|ext| ext.to_str()) == Some("proto") {
            files.push(path);
        }
    }
    files.sort();
    Ok(files)
}

fn scan_proto_metadata(proto_files: &[PathBuf]) -> Result<ProtoScan, Box<dyn Error>> {
    let mut messages = Vec::new();
    let mut field_attrs = Vec::new();
    for file in proto_files {
        let content = fs::read_to_string(file)?;
        let mut current_msg: Option<String> = None;
        for raw_line in content.lines() {
            let line = raw_line.trim();
            if line.starts_with("//") || line.is_empty() {
                continue;
            }
            if let Some(rest) = line.strip_prefix("message ") {
                let name = rest
                    .split_whitespace()
                    .next()
                    .unwrap_or_default()
                    .trim_end_matches('{');
                if !name.is_empty() {
                    messages.push(format!(".signal.v1.{name}"));
                    current_msg = Some(name.to_string());
                }
                continue;
            }
            if line == "}" {
                current_msg = None;
                continue;
            }
            let Some(msg) = current_msg.as_deref() else {
                continue;
            };
            let tokens: Vec<&str> = line.split_whitespace().collect();
            if tokens.len() < 3 {
                continue;
            }
            let (is_repeated, ty, field) = if tokens[0] == "repeated" && tokens.len() >= 4 {
                (true, tokens[1], tokens[2])
            } else {
                (false, tokens[0], tokens[1])
            };
            let helper = match (is_repeated, ty) {
                (false, "int64" | "sint64" | "sfixed64") => Some("crate::bigint_serde::i64_str"),
                (false, "uint64" | "fixed64") => Some("crate::bigint_serde::u64_str"),
                (true, "int64" | "sint64" | "sfixed64") => Some("crate::bigint_serde::vec_i64_str"),
                (true, "uint64" | "fixed64") => Some("crate::bigint_serde::vec_u64_str"),
                _ => None,
            };
            if let Some(serde_mod) = helper {
                field_attrs.push((format!(".signal.v1.{msg}.{field}"), serde_mod));
            }
        }
    }
    Ok(ProtoScan {
        messages,
        field_attrs,
    })
}

fn main() -> Result<(), Box<dyn Error>> {
    let manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR")?);
    let proto_root = manifest_dir.join("../../../contracts/proto");
    let v1_dir = proto_root.join("signal/v1");
    println!("cargo:rerun-if-changed={}", v1_dir.display());

    let proto_files = collect_proto_files(&v1_dir)?;
    for pf in &proto_files {
        println!("cargo:rerun-if-changed={}", pf.display());
    }

    let scan = scan_proto_metadata(&proto_files)?;
    let mut builder = tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]");

    for fq_msg in &scan.messages {
        builder = builder.type_attribute(fq_msg, "#[serde(default)]");
    }
    for (fq_field, serde_mod) in &scan.field_attrs {
        let attr = format!("#[serde(with = \"{serde_mod}\")]");
        builder = builder.field_attribute(fq_field, attr);
    }

    builder.compile_protos(&proto_files, &[proto_root])?;
    Ok(())
}
