use std::collections::HashMap;
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
pub struct Settings {

    pub distracting: HashMap<String, i32>,

    pub productive: HashMap<String, i32>,

}


pub fn save_settings(settings: &Settings) {

    let data =
        toml::to_string_pretty(settings)
        .unwrap();


    std::fs::write(
        "config/settings.toml",
        data
    )
    .unwrap();

}


pub fn load_settings() -> Settings {


    let data =
        std::fs::read_to_string(
            "config/settings.toml"
        )
        .unwrap_or(
            String::from(
                "[distracting]\n"
            )
        );


    toml::from_str(&data)
        .unwrap()

}
