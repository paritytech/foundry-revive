use std::path::PathBuf;

use alloy_primitives::map::HashMap;
use foundry_compilers::artifacts::{remappings, Libraries, Remapping, Severity};
use foundry_compilers::compile::resolc::resolc_artifact_output::ResolcArtifactOutput;
use foundry_compilers::compilers::resolc::ResolcCliSettings;
use foundry_compilers::compilers::resolc::{Resolc, ResolcOptimizer, ResolcSettings};

use foundry_compilers::{cache, Project, ProjectBuilder};
use foundry_compilers::{error::SolcError, solc::SolcLanguage, ProjectPathsConfig};
use foundry_config::Config;
use foundry_config::{SkipBuildFilters, SolcReq};
use semver::Version;
use tracing::trace;
pub const RESOLC_FILES_CACHE_FILENAME: &str = "resolc-files-cache.json";
pub const RESOLC_ARTIFACTS_DIR: &str = "resolc-out";

pub struct ResolcCompiler();
impl ResolcCompiler {
    pub fn config_ensure_resolc(
        resolc: Option<&SolcReq>,
        offline: bool,
    ) -> Result<Option<PathBuf>, SolcError> {
        if let Some(ref resolc) = resolc {
            let resolc = match resolc {
                SolcReq::Version(version) => {
                    let mut resolc = Resolc::find_installed_version(version)?;
                    if resolc.is_none() {
                        if offline {
                            return Err(SolcError::msg(format!(
                                "can't install missing resolc {version} in offline mode"
                            )));
                        }
                        Resolc::blocking_install(version)?;
                        resolc = Resolc::find_installed_version(version)?;
                    }
                    resolc
                }
                SolcReq::Local(resolc) => {
                    if !resolc.is_file() {
                        return Err(SolcError::msg(format!(
                            "`resolc` {} does not exist",
                            resolc.display()
                        )));
                    }
                    Some(resolc.clone())
                }
            };
            return Ok(resolc);
        }

        Ok(None)
    }
    pub fn config_project_paths(config: &Config) -> ProjectPathsConfig<SolcLanguage> {
        let builder = ProjectPathsConfig::builder()
            .cache(&config.cache_path.join(RESOLC_FILES_CACHE_FILENAME))
            .sources(&config.src)
            .tests(&config.test)
            .scripts(&config.script)
            .artifacts(&config.root.join(RESOLC_ARTIFACTS_DIR))
            .libs(config.libs.iter())
            .remappings(config.get_all_remappings())
            .allowed_path(&config.root)
            .allowed_paths(&config.libs)
            .allowed_paths(&config.allow_paths)
            .include_paths(&config.include_paths);

        /*if let Some(build_info_path) = &config.build_info_path {
            builder = builder.build_infos(build_info_path);
        }*/

        builder.build_with_root(&config.root)
    }

    pub fn solc_to_resolc_settings(config: &Config) -> Result<ResolcSettings, SolcError> {
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

        trace!("Final settings: {:?}", settings);

        Ok(settings)
    }

    pub fn create_project(
        config: &Config,
    ) -> Result<Project<Resolc, ResolcArtifactOutput>, SolcError> {
        let mut builder = ProjectBuilder::<Resolc>::default()
            .artifacts(ResolcArtifactOutput {})
            .settings(Self::solc_to_resolc_settings(&config)?)
            .paths(ResolcCompiler::config_project_paths(&config))
            .ignore_error_codes(config.ignored_error_codes.iter().copied().map(Into::into))
            .ignore_paths(config.ignored_file_paths.clone())
            .set_compiler_severity_filter(if config.deny_warnings {
                Severity::Warning
            } else {
                Severity::Error
            })
            .set_offline(config.offline)
            .set_cached(config.cache)
            .set_build_info(config.build_info)
            .set_no_artifacts(false);
        if !config.skip.is_empty() {
            let filter = SkipBuildFilters::new(config.skip.clone(), config.root.clone());
            builder = builder.sparse_output(filter);
        }
        let resolc = if let Some(resolc) =
            Self::config_ensure_resolc(config.resolc_config.resolc.as_ref(), config.offline)?
        {
            resolc
        } else if !config.offline {
            // ideally here we want to fetch the latest version from github but
            // for now we can hardcode the latest version
            let default_version = Version::parse("0.1.0-dev.8").unwrap();
            trace!("Checking for resolc compiler");
            let mut resolc = Resolc::find_installed_version(&default_version)?;
            trace!("{:?}", format!("Installing revive {:?}", &default_version));
            if resolc.is_none() {
                Resolc::blocking_install(&default_version)?;
                resolc = Resolc::find_installed_version(&default_version)?;
            }
            resolc.unwrap_or_else(|| panic!("Could not install resolc v{}", default_version))
        } else {
            "resolc".into()
        };
        // the below will never panic so calling unwrap is ok
        // We might need to revise the code for calling ::new
        let resolc_compiler = Resolc::new(resolc).unwrap();
        let project = builder.build(resolc_compiler)?;
        if config.force {
            config.cleanup(&project)?;
        }
        Ok(project)
    }
}
