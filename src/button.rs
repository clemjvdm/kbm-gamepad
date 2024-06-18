use evdev::Key;
use std::collections::HashMap;
use lazy_static::lazy_static;

// TODO: Maybe macros to not have to write the button options both here and in config struct, in
// which case maybe an enum can be used over a hash map?
lazy_static! {
    pub static ref VALID_BUTTONS: HashMap<&'static str, Key> = HashMap::from([
        ("EAST", evdev::Key::BTN_EAST),
        ("SOUTH", evdev::Key::BTN_SOUTH),
        ("NORTH", evdev::Key::BTN_NORTH),
        ("WEST", evdev::Key::BTN_WEST),
        ("DPAD_UP", evdev::Key::BTN_DPAD_UP),
        ("DPAD_DOWN", evdev::Key::BTN_DPAD_DOWN),
        ("DPAD_LEFT", evdev::Key::BTN_DPAD_LEFT),
        ("DPAD_RIGHT", evdev::Key::BTN_DPAD_RIGHT),
        ("SELECT", evdev::Key::BTN_SELECT),
        ("START", evdev::Key::BTN_START),
        ("L1", evdev::Key::BTN_TL),
        ("R1", evdev::Key::BTN_TR),
        ("L2", evdev::Key::BTN_TL2),
        ("R2", evdev::Key::BTN_TR2),
    ]);
}

trait GamepadButton {
    fn from_str(str: &str) -> Option<Key>;
}

impl GamepadButton for Key {
    fn from_str(str: &str) -> Option<Key> {
        VALID_BUTTONS.get(str).copied()
    }
}
