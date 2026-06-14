use schemars::JsonSchema;
use serde::Serialize;

pub const FILE_EXTENSIONS: &[&str] = &["tex", "sty", "cls", "bib", "def", "ltx"];

/// dprint configuration for tex-fmt.
///
/// All keys use camelCase.  Unrecognised keys produce a diagnostic.
/// Global dprint keys (`lineWidth`, `indentWidth`, `useTabs`) are used as
/// fallback defaults for `wraplen`, `tabsize`, and `tabchar` respectively.
#[derive(Clone, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Configuration {
    /// Wrap long lines (default: `true`).
    #[schemars(default = "default_true")]
    pub wrap: bool,

    /// Maximum line length before wrapping (default: `80`, or `lineWidth`).
    #[schemars(default = "default_wraplen", range(min = 1))]
    pub wraplen: usize,

    /// Lines longer than this will be wrapped.
    /// Defaults to `wraplen - 10` when `wraplen >= 50`, otherwise `wraplen`.
    #[schemars(range(min = 1))]
    pub wrapmin: usize,

    /// Number of spaces per indent level (default: `2`, or `indentWidth`).
    #[schemars(default = "default_tabsize", range(min = 1))]
    pub tabsize: u8,

    /// Character used for indentation.
    /// Must be `"space"` or `"tab"` (default: `"space"`, or `"tab"` when `useTabs` is set).
    #[schemars(default = "default_tabchar", extend("enum" = ["space", "tab"]))]
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
    #[schemars(default)]
    pub format_tables: bool,
}

fn default_true() -> bool { true }
fn default_wraplen() -> usize { 80 }
fn default_tabsize() -> u8 { 2 }
fn default_tabchar() -> String { "space".to_string() }
