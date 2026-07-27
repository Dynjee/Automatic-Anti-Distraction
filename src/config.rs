use std::collections::HashMap;
use serde::Deserialize;


#[derive(Deserialize)]
pub struct Settings {

    pub distracting: HashMap<String, i32>,

    pub productive: HashMap<String, i32>,

}


pub fn load_settings() -> Settings {

    let data =
        std::fs::read_to_string(
            "config/settings.toml"
        )
        .expect("Missing settings file");


    toml::from_str(&data)
        .expect("Invalid config")

}
