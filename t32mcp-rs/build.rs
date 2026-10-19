use serde_json::Value;

/// Workaround for typify 0.8.0, which panics with
/// `assertion failed: merged_schema.metadata.is_none()` when an `allOf`
/// contains two or more `$ref` subschemas whose referenced definitions
/// (transitively) carry sibling metadata (e.g. `description`).
///
/// If the schema contains such an `allOf`, strips `description`/`title`/
/// `default` from every `$ref` node and returns the path to a cleaned copy
/// written under `OUT_DIR`. Otherwise returns the original path unchanged.
fn sanitize_schema(schema: &str) -> String {
    fn has_multi_ref_allof(value: &Value) -> bool {
        match value {
            Value::Object(obj) => {
                if let Some(Value::Array(subs)) = obj.get("allOf") {
                    let ref_count = subs
                        .iter()
                        .filter(|s| matches!(s, Value::Object(o) if o.contains_key("$ref")))
                        .count();
                    if ref_count >= 2 {
                        return true;
                    }
                }
                obj.values().any(has_multi_ref_allof)
            }
            Value::Array(arr) => arr.iter().any(has_multi_ref_allof),
            _ => false,
        }
    }

    fn strip_ref_metadata(value: &mut Value) {
        match value {
            Value::Object(obj) => {
                if obj.contains_key("$ref") {
                    obj.remove("description");
                    obj.remove("title");
                    obj.remove("default");
                }
                for (_, v) in obj.iter_mut() {
                    strip_ref_metadata(v);
                }
            }
            Value::Array(arr) => {
                for v in arr {
                    strip_ref_metadata(v);
                }
            }
            _ => {}
        }
    }

    let schema_str =
        std::fs::read_to_string(schema).unwrap_or_else(|e| panic!("failed to read {schema}: {e}"));
    let mut value: Value = serde_json::from_str(&schema_str)
        .unwrap_or_else(|e| panic!("failed to parse {schema}: {e}"));

    if !has_multi_ref_allof(&value) {
        return schema.to_string();
    }

    strip_ref_metadata(&mut value);

    let out_dir = std::env::var("OUT_DIR").expect("OUT_DIR not set");
    let cleaned_path = std::path::Path::new(&out_dir).join(schema);
    if let Some(parent) = cleaned_path.parent() {
        std::fs::create_dir_all(parent).expect("failed to create OUT_DIR subdirectory");
    }
    std::fs::write(&cleaned_path, serde_json::to_string_pretty(&value).unwrap())
        .expect("failed to write cleaned schema");
    cleaned_path.to_string_lossy().into_owned()
}

/// Generates the Rust types for `schema` into `rs`, like `cargo typify <schema> -o <rs>`
/// (typify 0.8.0 with the builder interface), but without the external cargo-typify tool.
fn build(schema: &str, rs: &str) {
    println!("cargo::rerun-if-changed={}", schema);
    // also regenerate if the generated file was edited or deleted
    println!("cargo::rerun-if-changed={}", rs);

    let input = sanitize_schema(schema);
    let content =
        std::fs::read_to_string(&input).unwrap_or_else(|e| panic!("failed to read {input}: {e}"));
    let root: schemars::schema::RootSchema = serde_json::from_str(&content)
        .unwrap_or_else(|e| panic!("failed to parse {input} as JSON Schema: {e}"));

    let mut settings = typify::TypeSpaceSettings::default();
    settings.with_struct_builder(true);
    let mut type_space = typify::TypeSpace::new(&settings);
    type_space
        .add_root_schema(root)
        .unwrap_or_else(|e| panic!("schema conversion failed for {input}: {e}"));

    let intro = "#![allow(clippy::redundant_closure_call)]
#![allow(clippy::needless_lifetimes)]
#![allow(clippy::match_single_binding)]
#![allow(clippy::clone_on_copy)]
";
    let contents = format!("{intro}\n{}", type_space.to_stream());
    let contents = rustfmt_wrapper::rustfmt(contents)
        .unwrap_or_else(|e| panic!("failed to format the generated code for {input}: {e:?}"));

    // only write on changes, so the generated file does not trigger rebuilds
    if std::fs::read_to_string(rs).ok().as_deref() != Some(contents.as_str()) {
        std::fs::write(rs, contents).unwrap_or_else(|e| panic!("failed to write {rs}: {e}"));
    }
}

fn main() {
    println!("cargo::rerun-if-changed=build.rs");

    const MCP_LEGACY: [&str; 2] = ["mcp_2025_03_26", "mcp_2025_11_25"];
    const MCP_CURRENT: &str = "mcp_2026_07_28";

    for version in MCP_LEGACY {
        let schema_json = format!("{version}.json");
        let schema_rs = format!("src/{version}.rs");
        build(&schema_json, &schema_rs);
    }

    let schema_json = format!("{MCP_CURRENT}.json");
    let schema_rs = "src/mcp.rs";
    build(&schema_json, schema_rs);
}
