include!("src/schema_types.rs");

fn main() {
    println!("cargo:rerun-if-changed=src/schema_types.rs");
    println!("cargo:rerun-if-changed=Cargo.lock");

    let wasm = "target/wasm32-unknown-unknown/wasm-release/dprint_plugin_tex_fmt.wasm";
    println!("cargo:rerun-if-changed={wasm}");
    if std::path::Path::new(wasm).exists() {
        std::fs::copy(wasm, "plugin.wasm").expect("failed to copy plugin.wasm");
    }

    let tex_fmt_version = tex_fmt_version();

    // Generate schema.json
    let schema = schemars::schema_for!(Configuration);
    let mut value = serde_json::to_value(&schema).unwrap();
    if let serde_json::Value::Object(ref mut map) = value {
        map.insert(
            "$id".to_string(),
            serde_json::json!(format!(
                "https://plugins.dprint.dev/kjanat/tex-fmt/{}/schema.json",
                env!("CARGO_PKG_VERSION")
            )),
        );
        map.insert(
            "x-file-extensions".to_string(),
            serde_json::json!(FILE_EXTENSIONS),
        );
    }
    let sorted = json_schema_sort::sorted_schema(value);
    let schema_out = serde_json::to_string_pretty(&sorted).unwrap() + "\n";
    std::fs::write("schema.json", &schema_out).expect("failed to write schema.json");

    // Build texFmt example block from schema defaults
    let mut example = serde_json::Map::new();
    if let Some(props) = sorted.get("properties").and_then(|p| p.as_object()) {
        for (key, val) in props {
            if let Some(default) = val.get("default") {
                example.insert(key.clone(), default.clone());
            }
        }
    }
    let texfmt_json = serde_json::to_string_pretty(&serde_json::Value::Object(example)).unwrap();
    let texfmt_indented = {
        let mut lines = texfmt_json.lines();
        let first = lines.next().unwrap_or("{").to_string();
        let rest = lines.map(|l| format!("  {l}")).collect::<Vec<_>>().join("\n");
        if rest.is_empty() { first } else { format!("{first}\n{rest}") }
    };

    let version = env!("CARGO_PKG_VERSION");
    let extensions = FILE_EXTENSIONS
        .iter()
        .map(|e| format!("`.{e}`"))
        .collect::<Vec<_>>()
        .join(", ");

    let fragment = format!(
        r#"## Installation

```sh
dprint add kjanat/tex-fmt
```

Or add manually to `dprint.json`:

```json
{{
  "$schema": "https://dprint.dev/schemas/v0.json",
  "texFmt": {texfmt_indented},
  "plugins": [
    "https://plugins.dprint.dev/kjanat/tex-fmt-{version}.wasm"
  ]
}}
```

## Compatibility

- Based on [WGUNDERWOOD/tex-fmt](https://github.com/WGUNDERWOOD/tex-fmt) **{tex_fmt_version}**
- Formats files with extensions: {extensions}
"#
    );
    std::fs::write("release-fragment.md", fragment).expect("failed to write release-fragment.md");
}

fn tex_fmt_version() -> String {
    let lock = std::fs::read_to_string("Cargo.lock").unwrap_or_default();
    lock.split("[[package]]")
        .find_map(|chunk| {
            if chunk.contains("name = \"tex-fmt\"") {
                chunk.lines().find_map(|l| {
                    l.trim()
                        .strip_prefix("version = \"")?
                        .strip_suffix('"')
                        .map(|s| s.to_string())
                })
            } else {
                None
            }
        })
        .unwrap_or_else(|| "0.5".to_string())
}
