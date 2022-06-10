use config::{Config, ConfigError, Environment, File};
use serde_derive::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct AnimConfig {
    pub duration: f32,
    pub delay: f32,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct ButtonConfig {
    pub label: String,
    pub command: String,
    #[serde(default = "default_thickness")]
    pub thickness: f32,
}

fn default_thickness() -> f32 {
    1.0
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct FontConfig {
    pub family: String,
    pub style: String,
    pub size: f32,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct ConfigData {
    pub fullscreen: bool,
    pub shell: String,
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
            // Defaults - should never panic since the keys are hardcoded here, hence the ? operator.
            .set_default("fullscreen", false)?
            .set_default("shell", "sh")?
            .set_default("anim.duration", 1.0)?
            .set_default("anim.delay", 0.2)?
            .set_default("line.thickness", 1.5)?
            .set_default("font.size", 24.0)?
            .set_default("font.family", "sans")?
            .set_default("font.style", "")?
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
