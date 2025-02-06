use crate::Remapping;
use crate::{Config, SolcReq};
use alloy_primitives::map::HashMap;
use foundry_compilers::compilers::resolc::ResolcCliSettings;
use foundry_compilers::compilers::resolc::ResolcOptimizer;
use foundry_compilers::compilers::resolc::ResolcSettings;
use foundry_compilers::error::SolcError;
use foundry_compilers::ProjectPathsConfig;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
/// File contains info related to revive config
/// There is missing functionality such as
/// Converting between Foundry settings to Revive settings
/// The requirements for what needs to happen here still need
/// to be worked out this is initial work
/// This here will be part of the larger config so
/// Whenever we want to get any config related to revive
/// We can `config.revive.[anything related to revive]`
/// e.g `config.revive.settings` or `config.revive.revive_compile`

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
/// Revive Config
pub struct ReviveConfig {
    /// The revive bin
    pub revive: Option<SolcReq>,
    /// The solc path that will be used by revive
    pub solc_path: Option<PathBuf>,
    /// Enable compilation using revive
    pub revive_compile: bool,
    /// Revive settings
    pub revive_settings: ResolcSettings,
}

impl Default for ReviveConfig {
    fn default() -> Self {
        Self {
            revive: Default::default(),
            solc_path: Default::default(),
            revive_compile: Default::default(),
            revive_settings: ResolcSettings::default(),
        }
    }
}

impl ReviveConfig {
    /// This function serves as a wrapper for creating any settings
    /// Related to revive i.e. mapping foundry settings to revive
    /// Including any revive specific settings
    /// We still needs to change the Resolc settings to include the `revive_path`
    pub fn settings(&self, config: &Config) -> Result<ResolcSettings, SolcError> {
        let remappings: Vec<Remapping> = config
            .get_all_remappings()
            .map(|r| Remapping {
                name: r.name,
                path: r.path,
                context: Some(r.context.unwrap_or_default()),
            })
            .collect();
        let libraries = match config.parsed_libraries() {
            Ok(libs) => config.project_paths::<ProjectPathsConfig>().apply_lib_remappings(libs),
            Err(e) => return Err(SolcError::msg(format!("Failed to parse libraries: {e}"))),
        };

        let settings = ResolcSettings::new(
            ResolcOptimizer::new(config.optimizer, config.optimizer_runs as u64),
            HashMap::<String, HashMap<String, Vec<String>>>::default(),
            ResolcCliSettings::default(),
            remappings,
            Some(config.evm_version),
            libraries,
        );

        Ok(settings)
    }
}
