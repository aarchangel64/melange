use config::{Config, ConfigError, Environment, File};
use serde_derive::Deserialize;
use std::env;

#[derive(Debug, Deserialize, SmartDefault)]
#[allow(unused)]
#[serde(default)]
pub struct AnimConfig {
    #[default = 1.0]
    pub duration: f32,
    #[default = 0.2]
    pub delay: f32,
}

#[derive(Debug, Deserialize, SmartDefault)]
#[allow(unused)]
#[serde(default)]
pub struct ButtonConfig {
    #[default = "I'm a Button! :D"]
    pub label: String,
    pub command: String,
    #[default = 1.5]
    pub thickness: f32,
}

#[derive(Debug, Deserialize, SmartDefault)]
#[allow(unused)]
#[serde(default)]
pub struct FontConfig {
    #[default = "sans"]
    pub family: String,
    #[default(None)]
    pub style: Option<String>,
    #[default = 24.0]
    pub size: f32,
}

#[derive(Debug, Deserialize, SmartDefault)]
#[allow(unused)]
#[serde(default)]
pub struct ConfigData {
    #[default = true]
    pub fullscreen: bool,
    #[default = "sh"]
    pub shell: String,
    #[default(_code = "vec![ButtonConfig::default()]")]
    pub buttons: Vec<ButtonConfig>,
    pub anim: AnimConfig,
    pub font: FontConfig,
}

pub struct Settings {
    pub fullscreen: bool,
    pub shell: String,
    pub anim: AnimConfig,
    pub font: FontConfig,
}

impl ConfigData {
    pub fn new() -> Result<Self, ConfigError> {
        let mut config = Config::builder()
            // Merge in the user's config file, if it exists
            .add_source(
                File::with_name(&format!(
                    "{}/informant",
                    env::var("XDG_CONFIG_HOME").unwrap_or_else(|_| format!(
                        "{}/.config",
                        env::var("HOME").expect("Your $HOME variable isn't set, I think you have bigger problems than this panic.")
                    ))
                ))
                .required(false),
            )
            // Add in settings from the environment (with a prefix of INFORMANT)
            // Eg.. `INFORMANT_FULLSCREEN=1` would set the `fullscreen` key
            .add_source(Environment::with_prefix("informant"));

        let s = config.build()?;

        s.try_deserialize()
    }
}
