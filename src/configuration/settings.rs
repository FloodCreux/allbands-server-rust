use crate::configuration::{ApplicationSettings, DatabaseSettings, Environment};
use secrecy::Secret;

#[derive(serde::Deserialize, Clone)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application: ApplicationSettings,
    pub redis_uri: Secret<String>
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let base_path = std::env::current_dir().expect("Failed to locate base path");
    let configuration_directory = base_path.join("configuration");
    
    let environment: Environment = std::env::var("APP_ENVIRONMENT")

        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Failed to parse APP_ENVIRONMENT");

    let environment_filename = format!("{}.yaml", environment.as_str());
    let settings =  config::Config::builder()
        .add_source(config::File::from(
            configuration_directory.join("base.yaml")
        ))
        .add_source(config::File::from(
            configuration_directory.join(environment_filename)
        ))
        .add_source(
            config::Environment::with_prefix("APP")
                .prefix_separator("_")
                .separator("__")
        )
        .build()?;

    settings.try_deserialize::<Settings>()
}
