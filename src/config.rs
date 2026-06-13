use dprint_core::configuration::{
    ConfigKeyMap, ConfigKeyValue, ConfigurationDiagnostic, GlobalConfiguration, get_nullable_value,
    get_nullable_vec, get_unknown_property_diagnostics, get_value,
};
use dprint_core::plugins::{FileMatchingInfo, PluginResolveConfigurationResult};
use schemars::JsonSchema;
use serde::Serialize;

const FILE_EXTENSIONS: &[&str] = &["tex", "sty", "cls", "bib", "def", "ltx"];

/// dprint configuration for tex-fmt.
///
/// All keys use camelCase.  Unrecognised keys produce a diagnostic.
/// Global dprint keys (`lineWidth`, `indentWidth`, `useTabs`) are used as
/// fallback defaults for `wraplen`, `tabsize`, and `tabchar` respectively.
#[derive(Clone, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Configuration {
    /// Wrap long lines (default: `true`).
    pub wrap: bool,

    /// Maximum line length before wrapping (default: `80`, or `lineWidth`).
    pub wraplen: usize,

    /// Lines longer than this will be wrapped.
    /// Defaults to `wraplen - 10` when `wraplen >= 50`, otherwise `wraplen`.
    pub wrapmin: usize,

    /// Number of spaces per indent level (default: `2`, or `indentWidth`).
    pub tabsize: u8,

    /// Character used for indentation.
    /// Must be `"space"` or `"tab"` (default: `"space"`, or `"tab"` when `useTabs` is set).
    #[schemars(extend("enum" = ["space", "tab"]))]
    pub tabchar: String,

    /// Extra list environments beyond tex-fmt's built-in defaults.
    pub lists: Vec<String>,

    /// Extra verbatim environments beyond tex-fmt's built-in defaults.
    pub verbatims: Vec<String>,

    /// Environments that are not indented.
    pub no_indent_envs: Vec<String>,

    /// Characters after which lines may be wrapped (each entry must be a single character).
    pub wrap_chars: Vec<char>,

    /// Enable experimental table formatting (default: `false`).
    pub format_tables: bool,
}

pub fn resolve_config(
    mut config: ConfigKeyMap,
    global_config: &GlobalConfiguration,
) -> PluginResolveConfigurationResult<Configuration> {
    let mut diagnostics: Vec<ConfigurationDiagnostic> = Vec::new();

    let wrap = get_value(&mut config, "wrap", true, &mut diagnostics);

    let default_wraplen = global_config.line_width.unwrap_or(80) as usize;
    let wraplen: usize = get_value(&mut config, "wraplen", default_wraplen, &mut diagnostics);

    // Mirror tex-fmt's own wrapmin resolution from Args::from().
    let wrapmin_opt = get_nullable_value::<usize>(&mut config, "wrapmin", &mut diagnostics);
    let wrapmin = wrapmin_opt.unwrap_or_else(|| if wraplen >= 50 { wraplen - 10 } else { wraplen });

    let default_tabsize = global_config.indent_width.unwrap_or(2);
    let tabsize: u8 = get_value(&mut config, "tabsize", default_tabsize, &mut diagnostics);

    let default_tabchar = if global_config.use_tabs.unwrap_or(false) {
        "tab".to_string()
    } else {
        "space".to_string()
    };
    let tabchar: String = get_value(&mut config, "tabchar", default_tabchar, &mut diagnostics);
    if tabchar != "space" && tabchar != "tab" {
        diagnostics.push(ConfigurationDiagnostic {
            property_name: "tabchar".to_string(),
            message: format!("Expected \"space\" or \"tab\", got \"{tabchar}\"."),
        });
    }

    let default_lists: Vec<String> = [
        "itemize",
        "enumerate",
        "description",
        "inlineroman",
        "inventory",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();
    let lists = get_nullable_vec(&mut config, "lists", string_item("lists"), &mut diagnostics)
        .unwrap_or(default_lists);

    let default_verbatims: Vec<String> =
        ["verbatim", "Verbatim", "lstlisting", "minted", "comment"]
            .iter()
            .map(|s| s.to_string())
            .collect();
    let verbatims = get_nullable_vec(
        &mut config,
        "verbatims",
        string_item("verbatims"),
        &mut diagnostics,
    )
    .unwrap_or(default_verbatims);

    let default_no_indent_envs: Vec<String> = vec!["document".to_string()];
    let no_indent_envs = get_nullable_vec(
        &mut config,
        "noIndentEnvs",
        string_item("noIndentEnvs"),
        &mut diagnostics,
    )
    .unwrap_or(default_no_indent_envs);

    let default_wrap_chars: Vec<char> = vec![' '];
    let wrap_chars = get_nullable_vec(
        &mut config,
        "wrapChars",
        char_item("wrapChars"),
        &mut diagnostics,
    )
    .unwrap_or(default_wrap_chars);

    let format_tables = get_value(&mut config, "formatTables", false, &mut diagnostics);

    diagnostics.extend(get_unknown_property_diagnostics(config));

    PluginResolveConfigurationResult {
        file_matching: FileMatchingInfo {
            file_extensions: FILE_EXTENSIONS.iter().map(|s| s.to_string()).collect(),
            file_names: Vec::new(),
        },
        diagnostics,
        config: Configuration {
            wrap,
            wraplen,
            wrapmin,
            tabsize,
            tabchar,
            lists,
            verbatims,
            no_indent_envs,
            wrap_chars,
            format_tables,
        },
    }
}

fn string_item(
    key: &'static str,
) -> impl Fn(ConfigKeyValue, usize, &mut Vec<ConfigurationDiagnostic>) -> Option<String> {
    move |v, _i, diags| match v {
        ConfigKeyValue::String(s) => Some(s),
        _ => {
            diags.push(ConfigurationDiagnostic {
                property_name: key.to_string(),
                message: "Expected a string value.".to_string(),
            });
            None
        }
    }
}

fn char_item(
    key: &'static str,
) -> impl Fn(ConfigKeyValue, usize, &mut Vec<ConfigurationDiagnostic>) -> Option<char> {
    move |v, _i, diags| match v {
        ConfigKeyValue::String(s) => {
            let mut iter = s.chars();
            match (iter.next(), iter.next()) {
                (Some(c), None) => Some(c),
                _ => {
                    diags.push(ConfigurationDiagnostic {
                        property_name: key.to_string(),
                        message: "Expected a single-character string.".to_string(),
                    });
                    None
                }
            }
        }
        _ => {
            diags.push(ConfigurationDiagnostic {
                property_name: key.to_string(),
                message: "Expected a string value.".to_string(),
            });
            None
        }
    }
}
