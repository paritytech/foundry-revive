use foundry_compilers::{artifacts::OptimizerDetails, error::SolcError, multi::{MultiCompilerLanguage, MultiCompilerSettings}, ProjectPathsConfig};
use serde::{Deserialize, Serialize};

use crate::{Config, SolcReq};

/// Filename for Revive cache
pub const REVIVE_SOLIDITY_FILES_CACHE_FILENAME: &str = "revive-solidity-files-cache.json";

/// Directory for Revive artifacts
pub const REVIVE_ARTIFACTS_DIR: &str = "revive-out";

pub const CONTRACT_SIZE_LIMIT: usize = 250_000;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
/// Revive Config
pub struct ReviveConfig {
    /// Enable compilation using revive
    pub revive_compile: bool,

    /// The revive compiler
    pub revive: Option<SolcReq>,

    /// Whether to try to recompile with -Oz if the bytecode is too large.
    pub fallback_oz: bool,

    /// Enable optimizer for revive
    pub optimizer: bool,

    /// The optimization mode string for revive
    pub optimizer_mode: char,

    /// revive optimizer details
    pub optimizer_details: Option<OptimizerDetails>,

    // // revive suppressed warnings.
    // #[serde(deserialize_with = "deserialize_warning_set")]
    // pub suppressed_warnings: HashSet<WarningType>,

    // // revive suppressed errors.
    // #[serde(deserialize_with = "deserialize_error_set")]
    // pub suppressed_errors: HashSet<ErrorType>,
}

impl ReviveConfig {
    /// Returns the `ProjectPathsConfig` sub set of the config.
    pub fn project_paths(config: &Config) -> ProjectPathsConfig<MultiCompilerLanguage> {
        let builder = ProjectPathsConfig::builder()
            .cache(config.cache_path.join(REVIVE_SOLIDITY_FILES_CACHE_FILENAME))
            .sources(&config.src)
            .tests(&config.test)
            .scripts(&config.script)
            .artifacts(config.root.join(REVIVE_ARTIFACTS_DIR))
            .libs(config.libs.iter())
            .remappings(config.get_all_remappings())
            .allowed_path(&config.root)
            .allowed_paths(&config.libs)
            .allowed_paths(&config.allow_paths)
            .include_paths(&config.include_paths);

        builder.build_with_root(&config.root)
    }
}

// fn deserialize_warning_set<'de, D>(deserializer: D) -> Result<HashSet<WarningType>, D::Error>
// where
//     D: Deserializer<'de>,
// {
//     let strings: Vec<String> = Vec::deserialize(deserializer)?;
//     Ok(strings
//         .into_iter()
//         .filter_map(|s| match WarningType::from_str(&s) {
//             Ok(warning) => Some(warning),
//             Err(e) => {
//                 error!("Failed to parse warning type: '{}' with error: {}", s, e);
//                 None
//             }
//         })
//         .collect())
// }

// fn deserialize_error_set<'de, D>(deserializer: D) -> Result<HashSet<ErrorType>, D::Error>
// where
//     D: Deserializer<'de>,
// {
//     let strings: Vec<String> = Vec::deserialize(deserializer)?;
//     Ok(strings
//         .into_iter()
//         .filter_map(|s| match ErrorType::from_str(&s) {
//             Ok(error) => Some(error),
//             Err(e) => {
//                 error!("Failed to parse error type: '{}' with error: {}", s, e);
//                 None
//             }
//         })
//         .collect())
// }

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
/// Optimizer settings
pub struct Optimizer {
    // TODO: does this have to be an option?
    /// Enable the optimizer
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// Switch optimizer components on or off in detail.
    /// The "enabled" switch above provides two defaults which can be
    /// tweaked here. If "details" is given, "enabled" can be omitted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<OptimizerDetails>,
    /// Optimizer mode
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<char>,
    /// Whether to try to recompile with -Oz if the bytecode is too large.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fallback_to_optimizing_for_size: Option<bool>,
    /// Whether to disable the system request memoization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_system_request_memoization: Option<bool>,
    /// Set the jump table density threshold.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jump_table_density_threshold: Option<u32>,
}

impl Optimizer {
    /// Disable optimizer
    pub fn disable(&mut self) {
        self.enabled.take();
    }

    /// Enable optimizer
    pub fn enable(&mut self) {
        self.enabled = Some(true)
    }
}

impl Default for Optimizer {
    fn default() -> Self {
        Self {
            enabled: Some(false),
            mode: None,
            fallback_to_optimizing_for_size: None,
            disable_system_request_memoization: None,
            jump_table_density_threshold: None,
            details: None,
        }
    }
}
