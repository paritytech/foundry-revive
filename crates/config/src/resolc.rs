use crate::SolcReq;
use foundry_compilers::compilers::resolc::ResolcSettings;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
/// File contains info related to revive/resolc config
/// There is missing functionality such as
/// Converting between Foundry settings to Resolc settings
/// Will implement once i fix the binary issue

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
/// Resolc Config
pub struct ResolcConfig {
    pub settings: ResolcSettings,
    // revive instance if any
    pub resolc: Option<SolcReq>,
    pub solc_path: Option<PathBuf>,
}

impl Default for ResolcConfig {
    fn default() -> Self {
        Self {
            settings: ResolcSettings::default(),
            resolc: Default::default(),
            solc_path: Default::default(),
        }
    }
}
