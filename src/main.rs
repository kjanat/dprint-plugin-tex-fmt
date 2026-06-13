fn main() {
    let schema = schemars::schema_for!(dprint_plugin_tex_fmt::Configuration);
    let value = serde_json::to_value(&schema).unwrap();
    let sorted = json_schema_sort::sorted_schema(value);
    println!("{}", serde_json::to_string_pretty(&sorted).unwrap());
}
