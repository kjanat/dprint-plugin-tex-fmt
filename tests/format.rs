use dprint_core::configuration::{ConfigKeyMap, GlobalConfiguration};
use dprint_core::plugins::{PluginResolveConfigurationResult, SyncPluginHandler};
use dprint_plugin_tex_fmt::{Configuration, PluginHandler};
use std::path::Path;

fn make_handler() -> PluginHandler {
    PluginHandler::new()
}

fn resolve(handler: &mut PluginHandler) -> PluginResolveConfigurationResult<Configuration> {
    handler.resolve_config(ConfigKeyMap::new(), &GlobalConfiguration::default())
}

fn format_str(src: &str) -> Option<String> {
    let mut handler = make_handler();
    let result = resolve(&mut handler);

    let request = dprint_core::plugins::SyncFormatRequest {
        file_path: Path::new("test.tex"),
        file_bytes: src.as_bytes().to_vec(),
        config_id: dprint_core::plugins::FormatConfigId::from_raw(1),
        config: &result.config,
        range: None,
        token: &dprint_core::plugins::NullCancellationToken,
    };
    handler
        .format(request, |_| panic!("unexpected host format call"))
        .unwrap()
        .map(|bytes| String::from_utf8(bytes).unwrap())
}

#[test]
fn formats_unindented_document_environment() {
    let input = "\\begin{document}\nhello\n\\end{document}\n";
    let output = format_str(input);
    // tex-fmt should normalise indentation; here the body stays at indent 0
    // because `document` is in `no_indent_envs` by default.
    assert!(output.is_none() || output.as_deref() == Some(input));
}

#[test]
fn formats_nested_environment() {
    let input = "\\begin{document}\n\\begin{enumerate}\n\\item one\n\\item two\n\\end{enumerate}\n\\end{document}\n";
    let formatted = format_str(input);
    // Result must be well-formed and contain both items.
    let text = formatted.as_deref().unwrap_or(input);
    assert!(text.contains("\\item one"));
    assert!(text.contains("\\item two"));
}

#[test]
fn idempotent_on_already_formatted_file() {
    let input = "\\begin{document}\n  hello\n\\end{document}\n";
    // Run twice; second run must produce no change (None).
    let first = format_str(input);
    let second_input = first.as_deref().unwrap_or(input);
    let second = format_str(second_input);
    assert!(second.is_none(), "formatter is not idempotent");
}

#[test]
fn plugin_info_is_valid() {
    let mut handler = make_handler();
    let info = handler.plugin_info();
    assert_eq!(info.config_key, "texFmt");
    assert!(!info.version.is_empty());
}

#[test]
fn license_text_is_non_empty() {
    let mut handler = make_handler();
    assert!(!handler.license_text().is_empty());
}

#[test]
fn unknown_config_key_produces_diagnostic() {
    let mut handler = make_handler();
    let mut config = ConfigKeyMap::new();
    config.insert(
        "unknownOption".to_string(),
        dprint_core::configuration::ConfigKeyValue::Bool(true),
    );
    let result = handler.resolve_config(config, &GlobalConfiguration::default());
    assert!(
        result.diagnostics.iter().any(|d| d.property_name == "unknownOption"),
        "expected diagnostic for unknown key"
    );
}
