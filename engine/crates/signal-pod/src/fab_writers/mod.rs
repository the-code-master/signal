// Module: engine/crates/signal-pod/src/fab_writers/mod.rs — In-Memory Gerber X3, Excellon, ODB++ & IPC-2581 Rev C Writers
// Invariants: Pure RAM Vec<u8> generation ONLY (zero local disk writes, zero browser download); feeds signal-sftp-egress (07-*).
// Spec: architecture/07-security-encryption-dlp-and-sftp.md, architecture/19-cad-cam-3d-printing-and-plm-operations.md

use signal_geom::Point2Nm;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FabTrackPrimitive {
    pub start_nm: Point2Nm,
    pub end_nm: Point2Nm,
    pub width_nm: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FabDrillHit {
    pub pos_nm: Point2Nm,
    pub diameter_nm: i64,
}

/// Synthesizes an in-memory **Gerber X3** (`RS-274X` + `TF.FileFunction` attributes) byte buffer (`07-*`).
pub fn write_gerber_x3_in_memory(
    layer_function: &str,
    part_id: &str,
    tracks: &[FabTrackPrimitive],
) -> Vec<u8> {
    let mut out = String::new();
    use std::fmt::Write as _;
    let _ = writeln!(&mut out, "%TF.GenerationSoftware,Signal,EDA,0.1.0*%");
    let _ = writeln!(&mut out, "%TF.Part,{part_id}*%");
    let _ = writeln!(&mut out, "%TF.FileFunction,{layer_function}*%");
    let _ = writeln!(&mut out, "%FSLAX46Y46*%");
    let _ = writeln!(&mut out, "%MOMM*%");
    let _ = writeln!(&mut out, "%ADD10C,0.150000*%");
    let _ = writeln!(&mut out, "D10*");
    for tr in tracks {
        // 4.6 mm format => 1 unit = 1 nm!
        let _ = writeln!(&mut out, "X{}Y{}D02*", tr.start_nm.x.0, tr.start_nm.y.0);
        let _ = writeln!(&mut out, "X{}Y{}D01*", tr.end_nm.x.0, tr.end_nm.y.0);
    }
    let _ = writeln!(&mut out, "M02*");
    out.into_bytes()
}

/// Synthesizes an in-memory **Excellon 2 Metric NC Drill** byte buffer (`07-*`).
pub fn write_excellon_drill_in_memory(hits: &[FabDrillHit]) -> Vec<u8> {
    let mut out = String::from("M48\nMETRIC,TZ\nT01C0.250\n%\nT01\n");
    for h in hits {
        use std::fmt::Write as _;
        let x_mm = (h.pos_nm.x.0 as f64) * 1.0e-6;
        let y_mm = (h.pos_nm.y.0 as f64) * 1.0e-6;
        let _ = writeln!(&mut out, "X{x_mm:.4}Y{y_mm:.4}");
    }
    out.push_str("M30\n");
    out.into_bytes()
}

/// Synthesizes an in-memory **ODB++** structured manifest & layer feature stream (`07-*`).
pub fn write_odb_plusplus_manifest_in_memory(
    job_name: &str,
    layer_names: &[&str],
    tracks: &[FabTrackPrimitive],
) -> Vec<u8> {
    let mut out = format!("ODB++_ARCHIVE_V8|JOB={job_name}\n[MATRIX]\n");
    for (idx, name) in layer_names.iter().enumerate() {
        use std::fmt::Write as _;
        let _ = writeln!(
            &mut out,
            "ROW={} CONTEXT=BOARD TYPE=SIGNAL NAME={name}",
            idx + 1
        );
    }
    out.push_str("[FEATURES]\n");
    for tr in tracks {
        use std::fmt::Write as _;
        let _ = writeln!(
            &mut out,
            "L {} {} {} {} W={}",
            tr.start_nm.x.0, tr.start_nm.y.0, tr.end_nm.x.0, tr.end_nm.y.0, tr.width_nm
        );
    }
    out.into_bytes()
}

/// Synthesizes an in-memory **IPC-2581 Rev C** single-file XML CAM exchange buffer (`07-*`).
pub fn write_ipc2581_revc_xml_in_memory(
    project_id: &str,
    tracks: &[FabTrackPrimitive],
    drills: &[FabDrillHit],
) -> Vec<u8> {
    let mut xml = format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<IPC-2581 revision=\"C\" units=\"MILLIMETER\">\n  <Content roleRef=\"Owner\"/>\n  <Ecad name=\"{project_id}\">\n    <CadData>\n"
    );
    for tr in tracks {
        use std::fmt::Write as _;
        let _ = writeln!(
            &mut xml,
            "      <Line startX=\"{:.6}\" startY=\"{:.6}\" endX=\"{:.6}\" endY=\"{:.6}\" lineWidth=\"{:.6}\"/>",
            (tr.start_nm.x.0 as f64) * 1.0e-6,
            (tr.start_nm.y.0 as f64) * 1.0e-6,
            (tr.end_nm.x.0 as f64) * 1.0e-6,
            (tr.end_nm.y.0 as f64) * 1.0e-6,
            (tr.width_nm as f64) * 1.0e-6
        );
    }
    for d in drills {
        use std::fmt::Write as _;
        let _ = writeln!(
            &mut xml,
            "      <Hole x=\"{:.6}\" y=\"{:.6}\" diameter=\"{:.6}\" platingStatus=\"PLATED\"/>",
            (d.pos_nm.x.0 as f64) * 1.0e-6,
            (d.pos_nm.y.0 as f64) * 1.0e-6,
            (d.diameter_nm as f64) * 1.0e-6
        );
    }
    xml.push_str("    </CadData>\n  </Ecad>\n</IPC-2581>\n");
    xml.into_bytes()
}
