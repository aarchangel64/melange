use config::{Config, ConfigError, Environment, File};
use serde_derive::Deserialize;
use std::collections::HashMap;
use wry::application::dpi::{PhysicalPosition, PhysicalSize};

#[derive(Debug, Deserialize, SmartDefault)]
#[serde(default)]
pub struct Window {
    // TODO: add monitor selection
    #[default(FullscreenType::Borderless)]
    pub mode: FullscreenType,
    #[default(None)]
    pub size: Option<PhysicalSize<u32>>,
    #[default(None)]
    pub position: Option<PhysicalPosition<u32>>,
    #[default = true]
    pub always_on_top: bool,
    #[default = true]
    pub transparent: bool,
}

// Wry's fullscreen modes are jank
// 'Exclusive' doesn't work on Linux apparently
// Borderless here is just a window set to take up the full size of a monitor
#[derive(Debug, Deserialize)]
pub enum FullscreenType {
    Windowed,
    Borderless,
    Full,
}

#[derive(Debug, Deserialize)]
pub struct Command {
    name: String,
    action: String,
}

// Struct to specify format for user supplied config data.
#[derive(Debug, Deserialize, SmartDefault)]
#[serde(default)]
pub struct ConfigData {
    pub window: Window,
    pub commands: Vec<Command>,
}

// ConfigData gets mapped to these types for use by the program
pub struct Settings {
    pub window: Window,
    pub commands: HashMap<String, Vec<String>>,
}

impl Settings {
    pub fn new(config_dir: &str) -> Self {
        // TODO: Handle invalid config error
        let config_data = ConfigData::new(config_dir);

        match config_data {
            Ok(data) => {
                let mut map = HashMap::new();

                for command in data.commands {
                    map.insert(
                        command.name,
                        command
                            .action
                            .split_whitespace()
                            .map(&str::to_string)
                            .collect(),
                    );
                }

                Settings {
                    window: data.window,
                    commands: map,
                }
            }
            // TODO: Handle error better (maybe an error popup?)
            Err(error) => panic!("{}", error),
        }
    }
}

impl ConfigData {
    pub fn new(config_dir: &str) -> Result<Self, ConfigError> {
        let config = Config::builder()
            // Merge in the user's config file, if it exists
            .add_source(File::with_name(&format!("{config_dir}/config.toml")).required(false))
            // Add in settings from the environment (with a prefix of INFORMANT)
            // Eg.. `INFORMANT_FULLSCREEN=1` would set the `fullscreen` key
            .add_source(Environment::with_prefix("informant"));

        let s = config.build()?;

        s.try_deserialize()
    }
}
