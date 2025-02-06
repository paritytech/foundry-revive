use crate::opts::build::ReviveConfig;
use clap::Parser;
use foundry_config::SolcReq;
use serde::Serialize;
use std::path::PathBuf;

#[derive(Clone, Debug, Default, Serialize, Parser)]
#[clap(next_help_heading = "Revive configuration")]
/// Compiler options for revive
/// TODO: We need to add more revive specific arguments
pub struct ReviveArgs {
    #[clap(
        value_name = "REVIVE_COMPILE",
        help = "Enable compiling with revive",
        long = "revive-compile",
        action = clap::ArgAction::SetTrue,
        default_value_t = false
    )]
    pub revive_compile: bool,

    #[clap(help = "Solc path to be used by revive", long = "solc-path", value_name = "SOLC_PATH")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solc_path: Option<PathBuf>,

    #[clap(
        long = "revive",
        visible_alias = "revive",
        help = "Specify a custom revive version or path to be used",
        value_name = "REVIVE"
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revive: Option<SolcReq>,
}

impl ReviveArgs {
    pub(crate) fn apply_overrides(&self, mut revive_config: ReviveConfig) -> ReviveConfig {
        macro_rules! set_if_some {
            ($src:expr, $dst:expr) => {
                if let Some(src) = $src {
                    $dst = src.into();
                }
            };
        }

        set_if_some!(self.solc_path.clone(), revive_config.solc_path);
        set_if_some!(self.revive.clone(), revive_config.revive);
        revive_config.revive_compile = self.revive_compile;

        revive_config
    }
}
