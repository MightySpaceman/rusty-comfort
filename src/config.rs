use crate::AudioState;
use serde::{Deserialize, Serialize};
use shellexpand;
use std::{
    fs::{create_dir_all, File},
    io::{Read, Write},
    path::Path,
};
use toml::{self, to_string};

#[cfg(target_os = "linux")]
const CONFIG_DIR: &str = "~/.config/rusty-comfort/";
const CONFIG_FILENAME: &str = "Config.toml";

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub volume: f32,
    pub lowpass: f32,
    pub q: f32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            volume: 1.0,
            lowpass: 1000.0,
            q: 1.5,
        }
    }
}

impl Into<Config> for AudioState {
    fn into(self) -> Config {
        Config {
            volume: self.volume.value(),
            lowpass: self.lowpass.value(),
            q: self.q.value(),
        }
    }
}

pub fn write(cfg: Config) {
    let mut config_path = get_config_path();
    if !Path::new(&config_path.as_str()).exists() {
        create_dir_all(&config_path).expect("Could not create config directory!");
    }
    config_path.push_str(CONFIG_FILENAME);
    let mut f = File::create(&config_path).expect("Could not create config file!");
    let cfg_str = to_string(&cfg).expect("Could not Serialize config!");
    f.write_all(cfg_str.as_bytes())
        .expect("Could not write config file!");
}

pub fn read() -> Config {
    let mut config_path = get_config_path();
    config_path.push_str(CONFIG_FILENAME);
    let file = File::open(config_path);

    match file {
        Ok(mut f) => {
            let mut contents = String::new();
            f.read_to_string(&mut contents)
                .expect("Could not read from file!");
            let cfg: Config = toml::from_str(&contents).expect("Could not Deserialize config!");
            cfg
        }
        Err(e) => {
            println!("Could not load config file! E: {e}\nCreating and starting with default settings...");
            write(Config::default());
            Config::default()
        }
    }
}

fn get_config_path() -> String {
    String::from(shellexpand::tilde(CONFIG_DIR))
}
