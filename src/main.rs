mod keys;
mod button;
mod config;
use config::Config;
use config::GamepadSettings;
use button::VALID_BUTTONS;
use evdev::{
    Device, uinput::VirtualDeviceBuilder, uinput::VirtualDevice, AbsInfo, AbsoluteAxisType, UinputAbsSetup, InputEventKind, RelativeAxisType, InputEvent, EventType, Key
};
use std::collections::HashMap;
use toml;
use std::io::Write;
use std::path::Path;
use std::thread;
use std::sync::{Arc, Mutex, MutexGuard};



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

    // TESTING ONLY CODE SHOULD BE REMOVED 
    let mut file = std::fs::OpenOptions::new()
                                .write(true)
                                .create(true)
                                .truncate(true)
                                .open("config.toml")
                                .unwrap();
    let toml = toml::to_string(&config).unwrap();
    file.write_all(toml.as_bytes()).unwrap();
    // TESTING ONLY CODE SHOULD BE REMOVED 

    // TODO: get all mice
    /* let mut devices: Vec<Device> = Vec::new();
    for device in read_dir(DEVICES_PATH).unwrap().map(|device| Device::open(device.unwrap().path()).unwrap()) {
        if device.supported_absolute_axes().map_or(false, |axes| axes.contains(AbsoluteAxisType::ABS_X)) {
            devices.push(device);
        }
    } */
    // let mut gamepad = create_gamepad(&config.gamepad_settings).unwrap(); // virtual gamepad
    let gamepad = Arc::new(Mutex::new(create_gamepad(&config.gamepad_settings).unwrap())); // virtual gamepad
                                                                         
    let mut keyboard = Device::open(KEYBOARD_PATH).unwrap();
    let bind_map = config.bindings.to_map();

    let gamepad1 = Arc::clone(&gamepad);
    let mouse_thread = thread::spawn(move || { 
        let range = 1024;
        let mut mouse = Device::open(MOUSE_PATH).unwrap(); // mouse
        let mut mouse_x_pos: i32 = 0;
        loop {
            for ev in mouse.fetch_events().unwrap() {
                if ev.kind() == InputEventKind::RelAxis(RelativeAxisType::REL_X) {
                   // handle_mouse_event(ev, &mut mouse_x_pos, &mut g, range);
                    mouse_x_pos += ev.value();
                    let mut mouse_event_x_pos = mouse_x_pos;
                    if mouse_x_pos < -(range/2) { mouse_event_x_pos = -(range/2); }
                    if mouse_x_pos > range/2 { mouse_event_x_pos = range/2; }
                    let mut g = gamepad1.lock().unwrap();
                    g.emit(&[InputEvent::new(EventType::ABSOLUTE, AbsoluteAxisType::ABS_X.0, mouse_event_x_pos)]).unwrap()
                }
            }
        }
    });

    // TODO: leave the option of disabling kbd input
    /*let gamepad2 = Arc::clone(&gamepad);
    let kbd_thread = thread::spawn(move || {
        loop {
            for ev in keyboard.fetch_events().unwrap() {
                if let InputEventKind::Key(input_key) = ev.kind() {
                    if let Some(output_key) = bind_map.get(&input_key) {
                        let mut g = gamepad2.lock().unwrap();
                        g.emit(&[InputEvent::new(EventType::KEY, output_key.0, ev.value())]).unwrap();
                    }
                }
            }
        }
    });*/

    mouse_thread.join().unwrap();
    // kbd_thread.join().unwrap();
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

// TODO: fix these for cleaner code
fn handle_mouse_event(event: InputEvent, mouse_pos: &mut i32, gamepad: &mut MutexGuard<'static, VirtualDevice>, range: i32) {
            *mouse_pos += event.value();
            if *mouse_pos < -(range/2) { *mouse_pos = -(range/2); }
            if *mouse_pos > range/2 { *mouse_pos = range/2; }
            gamepad.emit(&[InputEvent::new(EventType::ABSOLUTE, AbsoluteAxisType::ABS_X.0, *mouse_pos)]).unwrap()
}


fn handle_kbd_event(event: InputEvent, binds: &HashMap<Key, Key>, gamepad: &mut MutexGuard<'static, VirtualDevice>) {
        if let InputEventKind::Key(input_key) = event.kind() {
            if let Some(output_key) = binds.get(&input_key) {
                gamepad.emit(&[InputEvent::new(EventType::KEY, output_key.0, event.value())]).unwrap();
            }
        }
}
