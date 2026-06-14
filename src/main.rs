fn main() {
    let schema = schemars::schema_for!(dprint_plugin_tex_fmt::Configuration);
    let mut value = serde_json::to_value(&schema).unwrap();

    if let serde_json::Value::Object(ref mut map) = value {
        map.insert(
            "$id".to_string(),
            serde_json::json!(concat!(
                "https://raw.githubusercontent.com/kjanat/dprint-plugin-tex-fmt/v",
                env!("CARGO_PKG_VERSION"),
                "/schema.json"
            )),
        );
    }

    let sorted = json_schema_sort::sorted_schema(value);
    let out = serde_json::to_string_pretty(&sorted).unwrap() + "\n";

    std::fs::write("schema.json", &out).expect("failed to write schema.json");
    println!("wrote schema.json");
}
