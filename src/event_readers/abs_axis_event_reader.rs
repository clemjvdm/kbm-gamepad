use std::sync::mpsc::Sender;
use std::{thread, thread::JoinHandle};
use evdev::{Device, InputEvent};
use crate::MOUSE_PATH;
use super::EventReaderTrait;
use std::any::Any;

pub struct MouseEventReader {
    thread: JoinHandle<()>,
}

impl EventReaderTrait for MouseEventReader {
    fn spawn(tx: Sender<InputEvent>) -> Self {
        let thread = thread::spawn(move || {
            let mut device = Device::open(MOUSE_PATH).unwrap();
            loop {
                for ev in device.fetch_events().unwrap() {
                    tx.send(ev).unwrap();
                }
            }
        });
        Self { thread }
    }

    fn join(self) -> Result<(), Box<dyn Any + Send + 'static>> {
        self.thread.join()
    }
}


 // TODO: get all mice
    /* let mut devices: Vec<Device> = Vec::new();
    for device in read_dir(DEVICES_PATH).unwrap().map(|device| Device::open(device.unwrap().path()).unwrap()) {
        if device.supported_absolute_axes().map_or(false, |axes| axes.contains(AbsoluteAxisType::ABS_X)) {
            devices.push(device);
        }
    } */
    // let mut gamepad = create_gamepad(&config.gamepad_settings).unwrap(); // virtual gamepad

