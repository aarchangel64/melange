use config::{Config, ConfigError, Environment, File};
use ggez::input::keyboard::{KeyCode, KeyMods};
use serde_derive::Deserialize;
use std::{collections::HashMap, env};

#[derive(Debug, Deserialize, SmartDefault)]
#[serde(default)]
pub struct Anim {
    #[default = 1.0]
    pub duration: f32,
    #[default = 0.2]
    pub delay: f32,
}

#[derive(Debug, Deserialize, SmartDefault)]
#[serde(default)]
pub struct ButtonConfig {
    #[default = "I'm a Button! :D"]
    pub label: String,
    pub command: String,
    #[default = 1.5]
    pub thickness: f32,
}

pub struct ButtonData {
    pub label: String,
    pub command: Vec<String>,
    pub thickness: f32,
}

#[derive(Debug, Deserialize, SmartDefault)]
#[serde(default)]
pub struct Font {
    #[default = "sans"]
    pub family: String,
    #[default(None)]
    pub style: Option<String>,
    #[default = 24.0]
    pub size: f32,
}

#[derive(Debug, Deserialize)]
pub enum Modifiers {
    SHIFT,
    CTRL,
    ALT,
    HYPER,
}

impl Modifiers {
    // Map Enum values to the keymods bitflag
    fn value(&self) -> KeyMods {
        match *self {
            Modifiers::CTRL => KeyMods::CTRL,
            Modifiers::SHIFT => KeyMods::SHIFT,
            Modifiers::ALT => KeyMods::ALT,
            Modifiers::HYPER => KeyMods::LOGO,
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Input {
    pub key: KeyCode,
    pub mods: KeyMods,
}

#[derive(Debug, Deserialize)]
pub struct Keymap {
    pub key: KeyCode,
    #[serde(default)]
    pub mods: Vec<Modifiers>,
    pub command: String,
}

#[derive(Debug, Deserialize, SmartDefault)]
#[serde(default)]
pub struct Background {
    #[default([0.11, 0.11, 0.11, 0.7])]
    pub color: [f32; 4],
    #[default([0.0, 0.0, 0.0, 0.0])]
    pub fade_in_color: [f32; 4],
    #[default = 1.0]
    pub fade_duration: f32,
}

#[derive(Debug, Deserialize, SmartDefault)]
#[serde(default)]
pub struct ConfigData {
    #[default = true]
    pub fullscreen: bool,
    pub background: Background,
    pub anim: Anim,
    pub font: Font,
    #[default(_code = "vec![ButtonConfig::default()]")]
    pub buttons: Vec<ButtonConfig>,
    pub keymap: Vec<Keymap>,
}

// TODO: Make a 'general' config struct
pub struct Settings {
    pub fullscreen: bool,
    pub background: Background,
    pub anim: Anim,
    pub font: Font,
    pub keymap: HashMap<Input, Vec<String>>,
}

impl Settings {
    fn make_command(string: &str) -> Vec<String> {
        string.split_whitespace().map(&str::to_string).collect()
    }

    pub fn new() -> (Self, Vec<ButtonData>) {
        // TODO: Handle invalid config error
        let settings = ConfigData::new();

        match settings {
            Ok(s) => {
                let mut map = HashMap::new();
                let mut keymod = KeyMods::empty();

                for keymap in s.keymap {
                    for m in keymap.mods {
                        keymod |= m.value();
                    }

                    map.insert(
                        Input {
                            key: keymap.key,
                            mods: keymod,
                        },
                        Settings::make_command(&keymap.command),
                    );
                }

                (
                    Settings {
                        fullscreen: s.fullscreen,
                        background: s.background,
                        anim: s.anim,
                        font: s.font,
                        keymap: map,
                    },
                    s.buttons
                        .iter()
                        .map(|b| ButtonData {
                            label: b.label.to_owned(),
                            command: Settings::make_command(&b.command),
                            thickness: b.thickness,
                        })
                        .collect(),
                )
            }
            // TODO: Handle error better (maybe an error popup?)
            Err(error) => panic!("{}", error),
        }
    }
}

impl ConfigData {
    pub fn new() -> Result<Self, ConfigError> {
        let config = Config::builder()
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
