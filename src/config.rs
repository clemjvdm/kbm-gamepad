use evdev::Key;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use super::keys::KbdString;

// TODO: Default values for all settings (serde default)

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub gamepad_settings: GamepadSettings,
    pub bindings: Bindings,
}

impl Config {
    pub fn default() -> Self {
        Self {
            gamepad_settings: GamepadSettings::default(),
            bindings: Bindings::default(),
        }
    }
}


#[derive(Serialize, Deserialize)]
pub struct Bindings {
    north: String,
    east: String,
    south: String,
    west: String,

    dpad_up: String,
    dpad_down: String,
    dpad_left: String,
    dpad_right: String,
    
    select: String,
    start: String,
    l1: String,
    l2: String,
    r1: String,
    r2: String,
}

// TODO: Better option than String to serialize and deserialize? Maybe &str?
// TODO: define enums to be able to rewrite HashMap<Key, Key> to HashMap<GamepadBtn, KbdKey>
// TODO: don't .unwrap()
impl Bindings {
    pub fn to_map(&self) -> HashMap<Key, Key> {
        HashMap::from([
            (Key::from_string(&self.north).unwrap(), Key::BTN_NORTH),
            (Key::from_string(&self.south).unwrap(), Key::BTN_SOUTH),
            (Key::from_string(&self.east).unwrap(), Key::BTN_EAST),
            (Key::from_string(&self.west).unwrap(), Key::BTN_WEST),

            (Key::from_string(&self.dpad_up).unwrap(), Key::BTN_DPAD_UP),
            (Key::from_string(&self.dpad_down).unwrap(), Key::BTN_DPAD_DOWN),
            (Key::from_string(&self.dpad_left).unwrap(), Key::BTN_DPAD_LEFT),
            (Key::from_string(&self.dpad_right).unwrap(), Key::BTN_DPAD_RIGHT),

            
            (Key::from_string(&self.select).unwrap(), Key::BTN_SELECT),
            (Key::from_string(&self.dpad_right).unwrap(), Key::BTN_START),
            (Key::from_string(&self.l1).unwrap(), Key::BTN_TL),
            (Key::from_string(&self.l2).unwrap(), Key::BTN_TL2),
            (Key::from_string(&self.r1).unwrap(), Key::BTN_TR),
            (Key::from_string(&self.r2).unwrap(), Key::BTN_TR2),
        ])
    }

    pub fn default() -> Self {
        Self {
            north : "w".to_string(),
            east : "a".to_string(),
            south : "s".to_string(),
            west : "d".to_string(),
            dpad_up : "up".to_string(),
            dpad_down : "down".to_string(),
            dpad_left : "left".to_string(),
            dpad_right : "right".to_string(),
            select : "x".to_string(),
            start : "c".to_string(),
            l1 : "q".to_string(),
            l2 : "leftshift".to_string(),
            r1 : "e".to_string(),
            r2 : "f".to_string(),
        }
    }
} 

#[derive(Serialize, Deserialize)]
pub struct GamepadSettings {
    pub range: i32,
    pub fuzz: i32,
    pub flat: i32,
    pub resolution: i32,
}

impl GamepadSettings {
    pub fn default() -> Self {
        Self {
            range: 1024,
            fuzz: 0,
            flat: 0,
            resolution: 1,
        }
    }
}
