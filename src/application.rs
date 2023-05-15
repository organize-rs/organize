//! Organize Abscissa Application

use std::fs::File;

use crate::{commands::EntryPoint, config::OrganizeConfig, error::ErrorKind};
use abscissa_core::{
    application::{self, AppCell},
    config::{self, CfgCell},
    status_err, trace, Application, Configurable, FrameworkError, StandardPaths,
};

use serde_yaml;

/// Application state
pub static ORGANIZE_APP: AppCell<OrganizeApp> = AppCell::new();

/// Organize Application
#[derive(Debug)]
pub struct OrganizeApp {
    /// Application configuration.
    config: CfgCell<OrganizeConfig>,

    /// Application state.
    state: application::State<Self>,
}

/// Initialize a new application instance.
///
/// By default no configuration is loaded, and the framework state is
/// initialized to a default, empty state (no components, threads, etc).
impl Default for OrganizeApp {
    fn default() -> Self {
        Self {
            config: CfgCell::default(),
            state: application::State::default(),
        }
    }
}

impl Application for OrganizeApp {
    /// Entrypoint command for this application.
    type Cmd = EntryPoint;

    /// Application configuration.
    type Cfg = OrganizeConfig;

    /// Paths to resources within the application.
    type Paths = StandardPaths;

    /// Accessor for application configuration.
    fn config(&self) -> config::Reader<OrganizeConfig> {
        self.config.read()
    }

    /// Borrow the application state immutably.
    fn state(&self) -> &application::State<Self> {
        &self.state
    }

    /// Register all components used by this application.
    ///
    /// If you would like to add additional components to your application
    /// beyond the default ones provided by the framework, this is the place
    /// to do so.
    fn register_components(&mut self, command: &Self::Cmd) -> Result<(), FrameworkError> {
        let framework_components = self.framework_components(command)?;

        // Load config *after* framework components so that we can
        // report an error to the terminal if it occurs.
        let config = match command.config_path() {
            Some(path) => {
                let file = File::open(path)?;

                let config: OrganizeConfig = match serde_yaml::from_reader(file) {
                    Ok(config) => config,
                    Err(err) => {
                        // TODO: Hard error here
                        status_err!(
                            "organize could not parse the provided config file. Using default. Errored: {}", err
                        );
                        OrganizeConfig::default()
                    }
                };
                config

                // match self.load_config(&path) {
                //     Ok(config) => config,
                //     Err(err) => {
                //         status_err!("organize could not parse the provided config file.");
                //         return Err(err);
                //     }
                // },
            }
            None => OrganizeConfig::default(),
        };

        let config = command.process_config(config)?;

        self.config.set_once(config);

        let mut app_components = self.state.components_mut();
        app_components.register(framework_components)
    }

    /// Post-configuration lifecycle callback.
    ///
    /// Called regardless of whether config is loaded to indicate this is the
    /// time in app lifecycle when configuration would be loaded if
    /// possible.
    fn after_config(&mut self, config: Self::Cfg) -> Result<(), FrameworkError> {
        // Configure components
        self.state.components_mut().after_config(&config)?;
        // TODO: Remove? We handle that in `register_components` now
        // self.config.set_once(config);
        Ok(())
    }

    /// Get tracing configuration from command-line options
    fn tracing_config(&self, command: &EntryPoint) -> trace::Config {
        if command.verbose {
            trace::Config::verbose()
        } else {
            trace::Config::default()
        }
    }
}
