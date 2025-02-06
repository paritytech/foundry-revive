use foundry_compilers::artifacts::Severity;
use foundry_compilers::compile::resolc::resolc_artifact_output::ResolcArtifactOutput;
use foundry_compilers::compilers::resolc::Resolc;
use foundry_compilers::{error::SolcError, solc::SolcLanguage, ProjectPathsConfig};
use foundry_compilers::{Project, ProjectBuilder};
use foundry_config::Config;
use foundry_config::{SkipBuildFilters, SolcReq};
use semver::Version;
use std::path::PathBuf;
use std::result::Result::Ok;
use tracing::trace;
pub const REVIVE_FILES_CACHE_FILENAME: &str = "revive-files-cache.json";
pub const REVIVE_ARTIFACTS_DIR: &str = "revive-out";
pub const REVIVE_DEFAULT_VERSION: &str = "0.1.0-dev.8";
pub struct ReviveCompiler();
impl ReviveCompiler {
    pub fn config_ensure_revive(
        revive: Option<&SolcReq>,
        offline: bool,
    ) -> Result<Option<PathBuf>, SolcError> {
        if let Some(ref revive) = revive {
            let revive = match revive {
                SolcReq::Version(version) => {
                    let mut revive = Resolc::find_installed_version(version)?;
                    if revive.is_none() {
                        if offline {
                            return Err(SolcError::msg(format!(
                                "can't install missing revive {version} in offline mode"
                            )));
                        }
                        Resolc::blocking_install(version)?;
                        revive = Resolc::find_installed_version(version)?;
                    }
                    revive
                }
                SolcReq::Local(revive) => {
                    if !revive.is_file() {
                        return Err(SolcError::msg(format!(
                            "`revive` {} does not exist",
                            revive.display()
                        )));
                    }
                    Some(revive.clone())
                }
            };
            return Ok(revive);
        }

        Ok(None)
    }
    pub fn config_project_paths(config: &Config) -> ProjectPathsConfig<SolcLanguage> {
        let mut builder = ProjectPathsConfig::builder()
            .cache(&config.cache_path.join(REVIVE_FILES_CACHE_FILENAME))
            .sources(&config.src)
            .tests(&config.test)
            .scripts(&config.script)
            .artifacts(&config.root.join(REVIVE_ARTIFACTS_DIR))
            .libs(config.libs.iter())
            .remappings(config.get_all_remappings())
            .allowed_path(&config.root)
            .allowed_paths(&config.libs)
            .allowed_paths(&config.allow_paths)
            .include_paths(&config.include_paths);

        if let Some(build_info_path) = &config.build_info_path {
            builder = builder.build_infos(build_info_path);
        }

        builder.build_with_root(&config.root)
    }

    pub fn create_project(
        config: &Config,
    ) -> Result<Project<Resolc, ResolcArtifactOutput>, SolcError> {
        let revive_settings = config.revive_config.settings(config).map_err(|error| error)?;
        let mut builder = ProjectBuilder::<Resolc>::default()
            .artifacts(ResolcArtifactOutput {})
            .settings(revive_settings)
            .paths(ReviveCompiler::config_project_paths(&config))
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
        let revive = match Self::config_ensure_revive(config.revive_config.revive.as_ref(), config.offline)? {
            Some(path) => path,
            None if config.offline => PathBuf::from("revive"),
            None => {
                let default_version = config.revive_config.revive.as_ref()
                    .and_then(|req| match req {
                        SolcReq::Version(version) => Some(version.clone()),
                        SolcReq::Local(path) => Resolc::get_version_for_path(path).ok()
                    })
                    .unwrap_or_else(|| Version::parse(REVIVE_DEFAULT_VERSION).unwrap());
        
                trace!("Checking for revive compiler");
                let revive = Resolc::find_installed_version(&default_version)?;
                if revive.is_none() {
                    trace!("Installing revive {}", &default_version);
                    Resolc::blocking_install(&default_version)?;
                    Resolc::find_installed_version(&default_version)?
                        .ok_or_else(|| SolcError::msg(format!("Could not install revive v{}", default_version)))?
                } else {
                    revive.unwrap()
                }
            }
        };

        let resolc_compiler = Resolc::new(revive).map_err(|error| {
            SolcError::msg(format!("Error creating revive instance: {:?}", error))
        })?;
        let project = builder.build(resolc_compiler)?;
        if config.force {
            config.cleanup(&project)?;
        }
        Ok(project)
    }
}
