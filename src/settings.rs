use config::{Config, ConfigError, Environment, File};
use serde_derive::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Font {
    pub family: String,
    pub style: String,
    pub size: f32,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Line {
    pub thickness: f32,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Settings {
    pub fullscreen: bool,
    pub line: Line,
    pub font: Font,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let s = Config::builder()
            // Defaults - should never panic since the keys are set here, hence the ? operator.
            .set_default("fullscreen", false)?
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
            .add_source(Environment::with_prefix("informant"))
            .build()?;

        // Deserialize (and thus freeze) the entire configuration as
        s.try_deserialize()
    }
}
