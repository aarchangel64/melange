use config::{Config, ConfigError, Environment, File};
use serde_derive::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
#[allow(unused)]
#[serde(default)]
pub struct AnimConfig {
    pub duration: f32,
    pub delay: f32,
}

impl Default for AnimConfig {
    fn default() -> Self {
        Self {
            duration: 1.0,
            delay: 0.2,
        }
    }
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
#[serde(default)]
pub struct ButtonConfig {
    pub label: String,
    pub command: String,
    pub thickness: f32,
}

impl Default for ButtonConfig {
    fn default() -> Self {
        Self {
            label: "I'm a Button! :D".to_string(),
            command: String::new(),
            thickness: 1.5,
        }
    }
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
#[serde(default)]
pub struct FontConfig {
    pub family: String,
    pub style: Option<String>,
    pub size: f32,
}

impl Default for FontConfig {
    fn default() -> Self {
        Self {
            family: String::from("sans"),
            style: None,
            size: 24.0,
        }
    }
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
#[serde(default)]
pub struct ConfigData {
    pub fullscreen: bool,
    pub shell: String,
    pub buttons: Vec<ButtonConfig>,
    pub anim: AnimConfig,
    pub font: FontConfig,
}

impl Default for ConfigData {
    fn default() -> Self {
        Self {
            fullscreen: true,
            shell: "sh".to_string(),
            buttons: vec![ButtonConfig::default()],
            anim: AnimConfig::default(),
            font: FontConfig::default(),
        }
    }
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
