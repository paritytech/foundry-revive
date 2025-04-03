use clap::Parser;
use foundry_config::revive::ReviveConfig;
use serde::Serialize;
#[derive(Clone, Debug, Default, Serialize, Parser)]
#[clap(next_help_heading = "Revive configuration")]
/// Compiler options for revive
pub struct ReviveOpts {
    #[clap(
        value_name = "REVIVE_COMPILE",
        help = "Enable compiling with revive",
        long = "revive-compile",
        visible_alias = "revive",
        action = clap::ArgAction::SetTrue,
    )]
    pub revive_compile: Option<bool>,

    /// Try to recompile with -Oz if the bytecode is too large.
    #[clap(
        long = "revive-fallback-oz",
        visible_alias = "fallback-oz",
        value_name = "FALLBACK_OZ",
        default_missing_value = "true"
    )]
    pub fallback_oz: Option<bool>,

    /// Set the LLVM optimization parameter `-O[0 | 1 | 2 | 3 | s | z]`.
    /// Use `3` for best performance and `z` for minimal size.
    #[clap(
        short = 'O',
        long = "revive-optimizer-mode",
        visible_alias = "revive-optimization",
        value_name = "LEVEL"
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optimizer_mode: Option<String>,

    /// Enables optimizations
    #[clap(long = "revive-optimizer")]
    pub optimizer: bool,

    // /// Set the warnings to suppress for revive.
    // #[clap(
    //     long = "revive-suppressed-warnings",
    //     alias = "suppressed-warnings",
    //     visible_alias = "suppress-warnings",
    //     value_delimiter = ',',
    //     help = "Set the warnings to suppress for revive, possible values: [txorigin, assemblycreate]"
    // )]
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub suppressed_warnings: Option<Vec<WarningType>>,

    // /// Set the errors to suppress for revive.
    // #[clap(
    //     long = "revive-suppressed-errors",
    //     alias = "suppressed-errors",
    //     visible_alias = "suppress-errors",
    //     value_delimiter = ',',
    //     help = "Set the errors to suppress for revive, possible values: [sendtransfer]"
    // )]
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub suppressed_errors: Option<Vec<ErrorType>>,
}

impl ReviveOpts {
    pub(crate) fn apply_overrides(&self, mut revive: ReviveConfig) -> ReviveConfig {
        macro_rules! set_if_some {
            ($src:expr, $dst:expr) => {
                if let Some(src) = $src {
                    $dst = src.into();
                }
            };
        }

        set_if_some!(self.revive_compile, revive.revive_compile);
        set_if_some!(self.optimizer.then_some(true), revive.optimizer);
        set_if_some!(
            self.optimizer_mode.as_ref().and_then(|mode| mode.parse::<char>().ok()),
            revive.optimizer_mode
        );
        // let suppressed_warnings = self
        //     .suppressed_warnings
        //     .clone()
        //     .map(|values| values.into_iter().collect::<HashSet<_>>());
        // set_if_some!(suppressed_warnings, revive.suppressed_warnings);
        // let suppressed_errors =
        //     self.suppressed_errors.clone().map(|values| values.into_iter().collect::<HashSet<_>>());
        // set_if_some!(suppressed_errors, revive.suppressed_errors);
        revive
    }
}
