use foundry_compilers::{
    error::SolcError, multi::MultiCompilerLanguage, solc::SolcSettings, ProjectPathsConfig,
};
use serde::{Deserialize, Serialize};

use crate::{Config, SolcReq};

/// Filename for resolc cache
pub const RESOLC_SOLIDITY_FILES_CACHE_FILENAME: &str = "resolc-solidity-files-cache.json";

/// Directory for resolc artifacts
pub const RESOLC_ARTIFACTS_DIR: &str = "resolc-out";

pub const CONTRACT_SIZE_LIMIT: usize = 250_000;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Default, Deserialize)]
/// Resolc Config
pub struct ResolcConfig {
    /// Enable compilation using resolc
    pub resolc_compile: bool,

    /// The resolc compiler
    pub resolc: Option<SolcReq>,

    /// The optimization mode string for resolc
    pub optimizer_mode: Option<char>,

    // The emulated EVM linear heap memory static buffer size in bytes
    pub heap_size: Option<u64>,

    // The contracts total stack size in bytes
    pub stack_size: Option<u64>,
}

impl ResolcConfig {
    /// Returns the `ProjectPathsConfig` sub set of the config.
    pub fn project_paths(config: &Config) -> ProjectPathsConfig<MultiCompilerLanguage> {
        let builder = ProjectPathsConfig::builder()
            .cache(config.cache_path.join(RESOLC_SOLIDITY_FILES_CACHE_FILENAME))
            .sources(&config.src)
            .tests(&config.test)
            .scripts(&config.script)
            .artifacts(config.root.join(RESOLC_ARTIFACTS_DIR))
            .libs(config.libs.iter())
            .remappings(config.get_all_remappings())
            .allowed_path(&config.root)
            .allowed_paths(&config.libs)
            .allowed_paths(&config.allow_paths)
            .include_paths(&config.include_paths);

        builder.build_with_root(&config.root)
    }

    pub fn resolc_settings(config: &Config) -> Result<SolcSettings, SolcError> {
        config.solc_settings().map(|mut s| {
            // Add optimizer_mode if present
            if let Some(mode) = config.resolc.optimizer_mode {
                s.cli_settings.extra_args.push(format!("--optimization={mode}"));
            }

            // Add heap_size if present
            if let Some(heap) = config.resolc.heap_size {
                s.cli_settings.extra_args.push(format!("--heap-size={heap}"));
            }

            // Add stack_size if present
            if let Some(stack) = config.resolc.stack_size {
                s.cli_settings.extra_args.push(format!("--stack-size={stack}"));
            }

            s
        })
    }
}
