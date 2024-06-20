mod event_readers;
mod translators;
mod event_handler;
mod keys;
mod button;
mod config;

use translators::{KeyTranslator, RelAxisTranslator};
use event_readers::{ EventReaderTrait, KbdEventReader, MouseEventReader };
use config::{ Config, GamepadSettings };
use event_handler::EventHandler;
use button::VALID_BUTTONS;

use evdev::{ uinput::VirtualDeviceBuilder, uinput::VirtualDevice, AbsInfo, AbsoluteAxisType, UinputAbsSetup};
use std::sync::mpsc::channel;

const CONFIG_FILE_PATH: &str = "config.toml";
const MOUSE_PATH: &str = "/dev/input/event19";
const KEYBOARD_PATH: &str = "/dev/input/event7";

fn main() {

    let config = if Config::exists() {
        Config::load().unwrap()
    } else { Config::default() };
       
    let gamepad = create_gamepad(&config.gamepad_settings).unwrap();

    let (tx, rx) = channel(); // tx is for sender(s), rx is for receiver

    let mut event_handler = EventHandler::new(gamepad);
    event_handler.add_translator(Box::new(KeyTranslator::new(config.bindings.to_map()))).unwrap();
    let range = config.gamepad_settings.range/2;
    event_handler.add_translator(Box::new(RelAxisTranslator::new(-range/2, range/2))).unwrap();
    event_handler.start(rx);

    if config.general_settings.read_kbd_events { // TODO: this is a temporary measure
        KbdEventReader::spawn(tx.clone());
    }
    let abs_axis_reader = MouseEventReader::spawn(tx.clone());

    // kbd_reader.join().unwrap();
    abs_axis_reader.join().unwrap();
    event_handler.stop().unwrap();
}


fn create_gamepad(settings: &GamepadSettings) -> std::io::Result<VirtualDevice> {
    let abs_setup = AbsInfo::new(0, -settings.range/2, settings.range/2, settings.fuzz, settings.flat, settings.resolution);
    let abs_x = UinputAbsSetup::new(AbsoluteAxisType::ABS_X, abs_setup);
    let abs_y = UinputAbsSetup::new(AbsoluteAxisType::ABS_Y, abs_setup);
    
    let mut set = evdev::AttributeSet::new();

    for value in VALID_BUTTONS.values() {
        set.insert(value.clone());
    }
    
    let device = VirtualDeviceBuilder::new()?
        .name("kbm-gamepad")
        .with_absolute_axis(&abs_x)?
        .with_absolute_axis(&abs_y)?
        .with_keys(&set)?
        .build()
        .unwrap();

    Ok(device)
}



