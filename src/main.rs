mod event_readers;
use event_readers::KbdEventReader;
mod translators;
use translators::{ KeyTranslator, RelAxisTranslator};
mod keys;
mod event_handler;
use event_handler::EventHandler;
mod button;
mod config;
use config::Config;
use config::GamepadSettings;
use button::VALID_BUTTONS;
use evdev::{Device, uinput::VirtualDeviceBuilder, uinput::VirtualDevice, AbsInfo, AbsoluteAxisType, UinputAbsSetup};
use toml;
use std::path::Path;
use std::thread;
use std::sync::mpsc::channel;
use event_readers::EventReader;
use std::sync::Arc;

const CONFIG_FILE_PATH: &str = "config.toml";
const MOUSE_PATH: &str = "/dev/input/event13";
const KEYBOARD_PATH: &str = "/dev/input/event9";

fn main() {

    // load config if file is present, else use default config
    let config: Config;
    if Path::new(CONFIG_FILE_PATH).exists() {
        let file_str = std::fs::read_to_string(CONFIG_FILE_PATH).unwrap();
        config = toml::from_str(&file_str).unwrap();
    } else {
        config = Config::default();
    }
    
    // TODO: get all mice
    /* let mut devices: Vec<Device> = Vec::new();
    for device in read_dir(DEVICES_PATH).unwrap().map(|device| Device::open(device.unwrap().path()).unwrap()) {
        if device.supported_absolute_axes().map_or(false, |axes| axes.contains(AbsoluteAxisType::ABS_X)) {
            devices.push(device);
        }
    } */
    // let mut gamepad = create_gamepad(&config.gamepad_settings).unwrap(); // virtual gamepad
    

    // create virtual gamepad
    let gamepad = create_gamepad(&config.gamepad_settings).unwrap(); // virtual gamepad

    // handler thread reads events from rx and handles them
    let (tx, rx) = channel();
    let handler_thread = thread::spawn(move || {
        let mut event_handler = EventHandler::new(gamepad, rx);
        let bind_map = config.bindings.to_map();
        event_handler.add_translator(Box::new(KeyTranslator::new(bind_map)));
        event_handler.add_translator(Box::new(RelAxisTranslator::new(
                                        -(config.gamepad_settings.range/2),
                                        config.gamepad_settings.range/2)));
        event_handler.start();
    });


    // mouse thread reads mouse events and sends them in rs
    let tx1 = tx.clone();
    let mouse_thread = thread::spawn(move || {
        let mut mouse = Device::open(MOUSE_PATH).unwrap();
        loop {
            for ev in mouse.fetch_events().unwrap() {
                tx1.send(ev).unwrap();
            }
        }
    });

    let kbd_reader = KbdEventReader::spawn(Arc::new(tx));

    kbd_reader.join().unwrap();
    mouse_thread.join().unwrap();
    handler_thread.join().unwrap();
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



