// Module: engine/crates/signal-mcad/src/writers_step_gltf_3mf.rs — In-Memory STEP AP242 (PMI), glTF 2.0 & 3MF Writers
// Invariants: Zero local disk writes; emits deterministic ISO-10303-242 STEP, glTF 2.0 JSON, and 3MF XML bytes (19-*).
// Spec: architecture/19-cad-cam-3d-printing-and-plm-operations.md
use signal_proto::v1::BoundingBox3dNm;

fn box_corners_mm(bounds: &BoundingBox3dNm) -> ([f64; 3], [f64; 3]) {
    let min_c = bounds.min_corner.unwrap_or_default();
    let max_c = bounds.max_corner.unwrap_or_default();
    (
        [
            (min_c.x_nm as f64) * 1e-6,
            (min_c.y_nm as f64) * 1e-6,
            (min_c.z_nm as f64) * 1e-6,
        ],
        [
            (max_c.x_nm as f64) * 1e-6,
            (max_c.y_nm as f64) * 1e-6,
            (max_c.z_nm as f64) * 1e-6,
        ],
    )
}

/// Emits an in-memory ISO-10303-242 STEP AP242 solid model with embedded GD&T PMI tolerance annotations.
pub fn write_step_ap242_with_pmi_bytes(
    part_name: &str,
    bounds_nm: &BoundingBox3dNm,
    flatness_tol_um: u32,
) -> Vec<u8> {
    let (mn, mx) = box_corners_mm(bounds_nm);
    let tol_mm = (flatness_tol_um as f64) * 1e-3;
    let step = format!(
        "ISO-10303-21;\n\
HEADER;\n\
FILE_DESCRIPTION(('Signal Parametric B-Rep Solid with GD&T PMI'),'2;1');\n\
FILE_NAME('{part_name}.step','2026-11-22T00:00:00',('Signal MCAD'),('Signal Design'),'signal-mcad','signal-pod','');\n\
FILE_SCHEMA(('AP242_MANAGED_MODEL_BASED_3D_ENGINEERING_MIM_LF {{ 1 0 10303 442 1 1 4 }}'));\n\
ENDSEC;\n\
DATA;\n\
#10=PRODUCT('{part_name}','{part_name}','',(#11));\n\
#11=PRODUCT_CONTEXT('',#12,'mechanical');\n\
#12=APPLICATION_CONTEXT('managed model based 3d engineering');\n\
#20=CARTESIAN_POINT('min_corner',({:.6},{:.6},{:.6}));\n\
#21=CARTESIAN_POINT('max_corner',({:.6},{:.6},{:.6}));\n\
#30=MANIFOLD_SOLID_BREP('{part_name}_solid',#31);\n\
#31=CLOSED_SHELL('',(#32));\n\
#32=ADVANCED_FACE('datum_A',(),#33,.T.);\n\
#33=PLANE('',#20);\n\
#40=DATUM('A','Primary Mounting Datum',#10,.F.,'A');\n\
#41=FLATNESS_TOLERANCE('PMI_FLATNESS_A','Mounting Boss Coplanarity',{tol_mm:.4},#32);\n\
ENDSEC;\n\
END-ISO-10303-21;\n",
        mn[0], mn[1], mn[2], mx[0], mx[1], mx[2]
    );
    step.into_bytes()
}

/// Emits a deterministic glTF 2.0 JSON scene representation for zero-leak viewport/AR visualization.
pub fn write_gltf2_scene_bytes(node_name: &str, bounds_nm: &BoundingBox3dNm) -> Vec<u8> {
    let (mn, mx) = box_corners_mm(bounds_nm);
    let gltf = format!(
        "{{\n  \"asset\": {{ \"version\": \"2.0\", \"generator\": \"signal-mcad\" }},\n  \"scene\": 0,\n  \"scenes\": [{{ \"name\": \"{node_name}\", \"nodes\": [0] }}],\n  \"nodes\": [{{ \"name\": \"{node_name}\", \"mesh\": 0 }}],\n  \"meshes\": [{{\n    \"name\": \"{node_name}_mesh\",\n    \"primitives\": [{{ \"attributes\": {{ \"POSITION\": 0 }}, \"mode\": 4 }}]\n  }}],\n  \"accessors\": [{{\n    \"componentType\": 5126,\n    \"count\": 8,\n    \"type\": \"VEC3\",\n    \"min\": [{:.6}, {:.6}, {:.6}],\n    \"max\": [{:.6}, {:.6}, {:.6}]\n  }}]\n}}\n",
        mn[0], mn[1], mn[2], mx[0], mx[1], mx[2]
    );
    gltf.into_bytes()
}

/// Emits a watertight 12-triangle 3MF XML model (`<model unit="millimeter">`) in RAM.
pub fn write_3mf_model_bytes(object_name: &str, bounds_nm: &BoundingBox3dNm) -> Vec<u8> {
    let (mn, mx) = box_corners_mm(bounds_nm);
    let verts = [
        (mn[0], mn[1], mn[2]),
        (mx[0], mn[1], mn[2]),
        (mx[0], mx[1], mn[2]),
        (mn[0], mx[1], mn[2]),
        (mn[0], mn[1], mx[2]),
        (mx[0], mn[1], mx[2]),
        (mx[0], mx[1], mx[2]),
        (mn[0], mx[1], mx[2]),
    ];
    let tris: [(usize, usize, usize); 12] = [
        (0, 2, 1),
        (0, 3, 2),
        (4, 5, 6),
        (4, 6, 7),
        (0, 1, 5),
        (0, 5, 4),
        (2, 3, 7),
        (2, 7, 6),
        (1, 2, 6),
        (1, 6, 5),
        (3, 0, 4),
        (3, 4, 7),
    ];

    let mut xml = String::from(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
<model unit=\"millimeter\" xml:lang=\"en-US\" xmlns=\"http://schemas.microsoft.com/3dmanufacturing/core/2015/02\">\n\
  <metadata name=\"Title\">",
    );
    xml.push_str(object_name);
    xml.push_str("</metadata>\n  <resources>\n    <object id=\"1\" type=\"model\">\n      <mesh>\n        <vertices>\n");
    for (vx, vy, vz) in verts {
        xml.push_str(&format!(
            "          <vertex x=\"{vx:.6}\" y=\"{vy:.6}\" z=\"{vz:.6}\" />\n"
        ));
    }
    xml.push_str("        </vertices>\n        <triangles>\n");
    for (v1, v2, v3) in tris {
        xml.push_str(&format!(
            "          <triangle v1=\"{v1}\" v2=\"{v2}\" v3=\"{v3}\" />\n"
        ));
    }
    xml.push_str(
        "        </triangles>\n      </mesh>\n    </object>\n  </resources>\n  <build>\n    <item objectid=\"1\" />\n  </build>\n</model>\n",
    );
    xml.into_bytes()
}
