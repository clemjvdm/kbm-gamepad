mod button;
mod config;
use config::Config;
use config::GamepadSettings;
use button::VALID_BUTTONS;
use evdev::{
    Device, uinput::VirtualDeviceBuilder, uinput::VirtualDevice, AbsInfo, AbsoluteAxisType, UinputAbsSetup,
};
use std::collections::HashMap;

// GAMEPAD
const RANGE: i32 = 1024; // min value of controller x axis is -(range/2) and max is (range/2)
const FUZZ: i32 = 0; // changes smaller or equal to fuzz will be ignored
const FLAT: i32 = 0; // joystick dead zone
const RESOLUTION: i32 = 1; // TODO: what exactly is resolution used for?

const MOUSE_PATH: &str = "/dev/input/event13";
// const DEVICES_PATH: &str = "/dev/input/";

fn main() {
    /*let config: Config = Config { bindings: HashMap::new(), gamepad_settings: GamepadSettings {
        range: 1024,
        fuzz: 0,
        flat: 0,
        resolution: 1,
        }  
    };*/

    let mut gamepad = create_gamepad().unwrap(); // virtual gamepad
    let mut mouse = Device::open(MOUSE_PATH).unwrap(); // mouse

    // TODO: get all mice
    /* let mut devices: Vec<Device> = Vec::new();
    for device in read_dir(DEVICES_PATH).unwrap().map(|device| Device::open(device.unwrap().path()).unwrap()) {
        if device.supported_absolute_axes().map_or(false, |axes| axes.contains(AbsoluteAxisType::ABS_X)) {
            devices.push(device);
        }
    } */

    let mut mouse_x_position: i32 = 0;
    loop {
        for ev in mouse.fetch_events().unwrap() {
            println!("mouse event.");
            if ev.kind() == evdev::InputEventKind::RelAxis(evdev::RelativeAxisType::REL_X) {
                println!("relative axis event.");
                let mouse_x_movment = ev.value();
                mouse_x_position += mouse_x_movment;
                let translated_position = if mouse_x_position < -(RANGE/2) {
                    -(RANGE/2)
                } else if mouse_x_position > RANGE/2 {
                    RANGE
                } else {
                    mouse_x_position
                };
                gamepad.emit(&[evdev::InputEvent::new(evdev::EventType::ABSOLUTE, AbsoluteAxisType::ABS_X.0, translated_position)]).unwrap();
            }
        }
    }
}



fn create_gamepad() -> std::io::Result<VirtualDevice> {

    let abs_setup = AbsInfo::new(0, -RANGE/2, RANGE/2, FUZZ, FLAT, RESOLUTION);
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
