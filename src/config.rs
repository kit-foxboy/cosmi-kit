// SPDX-License-Identifier: MPL-2.0

use cosmic::cosmic_config::{self, cosmic_config_derive::CosmicConfigEntry, ConfigGet, ConfigSet, CosmicConfigEntry};

use crate::application::app_data::SavedOC;

#[derive(Debug, Default, Clone, CosmicConfigEntry, Eq, PartialEq)]
#[version = 1]
pub struct Config {
    demo: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SavedCharactersConfig {
    pub characters: Vec<SavedOC>,
}

pub const CONFIG_KEY: &str = "com.github.kitfoxboy.cosmi-kit";


impl CosmicConfigEntry for SavedCharactersConfig {
    const VERSION: u64 = 1;
    
    fn write_entry(&self, config: &cosmic_config::Config) -> Result<(), cosmic_config::Error> {
        config.set("characters", self.characters.clone())
    }
    
    fn get_entry(config: &cosmic_config::Config) -> Result<Self, (Vec<cosmic_config::Error>, Self)> {
        config.get("characters").map(|characters| Self { characters }).map_err(|e| (vec![e], Self { characters: vec![] }))
    }
    
    fn update_keys<T: AsRef<str>>(
        &mut self,
        _config: &cosmic_config::Config,
        _changed_keys: &[T],
    ) -> (Vec<cosmic_config::Error>, Vec<&'static str>) {
        todo!("Figure out what this function is for and when it gets called lol");
    }

    
}
