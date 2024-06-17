use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub bindings: HashMap<String, String>,
    pub gamepad_settings: GamepadSettings,
}


#[derive(Serialize, Deserialize)]
pub struct GamepadSettings {
    pub range: i32,
    pub fuzz: i32,
    pub flat: i32,
    pub resolution: i32,
}

