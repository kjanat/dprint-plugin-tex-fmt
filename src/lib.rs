pub mod config;

use config::resolve_config;
pub use config::Configuration;
use dprint_core::configuration::{ConfigKeyMap, GlobalConfiguration};
use dprint_core::plugins::{
    CheckConfigUpdatesMessage, ConfigChange, PluginInfo,
    PluginResolveConfigurationResult, SyncFormatRequest, SyncHostFormatRequest, SyncPluginHandler,
};
use tex_fmt::args::{Args, TabChar};
use tex_fmt::format::format_file;
use tex_fmt::logging::Log;

pub struct PluginHandler;

impl PluginHandler {
    pub const fn new() -> Self {
        Self
    }
}

impl Default for PluginHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl SyncPluginHandler<Configuration> for PluginHandler {
    fn plugin_info(&mut self) -> PluginInfo {
        PluginInfo {
            name: env!("CARGO_PKG_NAME").to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            config_key: "texFmt".to_string(),
            help_url: env!("CARGO_PKG_REPOSITORY").to_string(),
            config_schema_url: String::new(),
            update_url: None,
        }
    }

    fn license_text(&mut self) -> String {
        include_str!("../LICENSE-MIT").to_string()
    }

    fn resolve_config(
        &mut self,
        config: ConfigKeyMap,
        global_config: &GlobalConfiguration,
    ) -> PluginResolveConfigurationResult<Configuration> {
        resolve_config(config, global_config)
    }

    fn check_config_updates(
        &self,
        _message: CheckConfigUpdatesMessage,
    ) -> anyhow::Result<Vec<ConfigChange>> {
        Ok(Vec::new())
    }

    fn format(
        &mut self,
        request: SyncFormatRequest<Configuration>,
        _format_with_host: impl FnMut(SyncHostFormatRequest) -> dprint_core::plugins::FormatResult,
    ) -> dprint_core::plugins::FormatResult {
        let cfg = request.config;

        let source = std::str::from_utf8(&request.file_bytes)
            .map_err(|e| anyhow::anyhow!("file is not valid UTF-8: {e}"))?;

        let tabchar = if cfg.tabchar == "tab" {
            TabChar::Tab
        } else {
            TabChar::Space
        };

        let args = Args {
            check: false,
            print: true,
            fail_on_change: false,
            wrap: cfg.wrap,
            wraplen: cfg.wraplen,
            wrapmin: cfg.wrapmin,
            tabsize: cfg.tabsize,
            tabchar,
            stdin: false,
            config: None,
            lists: cfg.lists.clone(),
            verbatims: cfg.verbatims.clone(),
            no_indent_envs: cfg.no_indent_envs.clone(),
            wrap_chars: cfg.wrap_chars.clone(),
            verbosity: log::LevelFilter::Off,
            arguments: false,
            files: Vec::new(),
            recursive: false,
            format_tables: cfg.format_tables,
        };

        let mut logs: Vec<Log> = Vec::new();
        let formatted = format_file(source, request.file_path, &args, &mut logs);

        if formatted == source {
            Ok(None)
        } else {
            Ok(Some(formatted.into_bytes()))
        }
    }
}

// Import the macro so its internal recursive call resolves in this crate's scope.
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
use dprint_core::generate_plugin_code;

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
generate_plugin_code!(PluginHandler, PluginHandler::new());
