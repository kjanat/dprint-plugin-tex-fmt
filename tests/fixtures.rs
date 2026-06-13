use dprint_core::configuration::{ConfigKeyMap, ConfigKeyValue, GlobalConfiguration};
use dprint_core::plugins::SyncPluginHandler;
use dprint_plugin_tex_fmt::PluginHandler;
use std::path::Path;
use std::{env, fs};

fn json_to_config(value: serde_json::Value) -> ConfigKeyValue {
    match value {
        serde_json::Value::Bool(b) => ConfigKeyValue::Bool(b),
        serde_json::Value::Number(n) => ConfigKeyValue::Number(n.as_i64().unwrap() as i32),
        serde_json::Value::String(s) => ConfigKeyValue::String(s),
        serde_json::Value::Null => ConfigKeyValue::Null,
        serde_json::Value::Array(arr) => {
            ConfigKeyValue::Array(arr.into_iter().map(json_to_config).collect())
        }
        serde_json::Value::Object(obj) => {
            let map = obj
                .into_iter()
                .map(|(k, v)| (k, json_to_config(v)))
                .collect();
            ConfigKeyValue::Object(map)
        }
    }
}

fn load_config(fixture_dir: &Path) -> ConfigKeyMap {
    let config_path = fixture_dir.join("config.json");
    if !config_path.exists() {
        return ConfigKeyMap::new();
    }
    let json: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&config_path).unwrap()).unwrap();
    match json {
        serde_json::Value::Object(obj) => obj
            .into_iter()
            .map(|(k, v)| (k, json_to_config(v)))
            .collect(),
        _ => panic!("config.json must be a JSON object"),
    }
}

fn format_source(src: &str, config: ConfigKeyMap) -> String {
    let mut handler = PluginHandler::new();
    let result = handler.resolve_config(config, &GlobalConfiguration::default());
    assert!(
        result.diagnostics.is_empty(),
        "config diagnostics: {:?}",
        result.diagnostics
    );
    let request = dprint_core::plugins::SyncFormatRequest {
        file_path: Path::new("test.tex"),
        file_bytes: src.as_bytes().to_vec(),
        config_id: dprint_core::plugins::FormatConfigId::from_raw(1),
        config: &result.config,
        range: None,
        token: &dprint_core::plugins::NullCancellationToken,
    };
    let formatted = handler
        .format(request, |_| panic!("unexpected host format call"))
        .unwrap();
    match formatted {
        Some(bytes) => String::from_utf8(bytes).unwrap(),
        None => src.to_string(),
    }
}

fn run_fixture(name: &str) {
    let fixture_dir = Path::new("tests/fixtures").join(name);
    let source_dir = fixture_dir.join("source");
    let target_dir = fixture_dir.join("target");
    let bless = env::var("BLESS").is_ok();
    let config = load_config(&fixture_dir);

    let mut entries: Vec<_> = fs::read_dir(&source_dir)
        .unwrap_or_else(|_| panic!("no source dir for fixture '{name}'"))
        .map(|e| e.unwrap().file_name())
        .collect();
    entries.sort();

    for filename in &entries {
        let source_path = source_dir.join(filename);
        let target_path = target_dir.join(filename);

        let source = fs::read_to_string(&source_path).unwrap();
        let formatted = format_source(&source, config.clone());

        if bless {
            fs::create_dir_all(&target_dir).unwrap();
            fs::write(&target_path, &formatted).unwrap();
        } else {
            let target = fs::read_to_string(&target_path).unwrap_or_else(|_| {
                panic!(
                    "target missing for fixture '{name}/{filename:?}'; \
                     run: BLESS=1 cargo test --test fixtures"
                )
            });
            assert_eq!(
                formatted, target,
                "fixture '{name}/{filename:?}': formatted output differs from target"
            );

            // Idempotency: a second format pass must be a no-op.
            let reformatted = format_source(&formatted, config.clone());
            assert_eq!(
                reformatted, formatted,
                "fixture '{name}/{filename:?}' is not idempotent"
            );
        }
    }
}

#[test]
fn fixture_short_document() {
    run_fixture("short_document");
}

#[test]
fn fixture_tabsize() {
    run_fixture("tabsize");
}

#[test]
fn fixture_wrap() {
    run_fixture("wrap");
}

#[test]
fn fixture_verbatim() {
    run_fixture("verbatim");
}
