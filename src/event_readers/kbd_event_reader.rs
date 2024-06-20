use std::sync::mpsc::Sender;
use std::{thread, thread::JoinHandle};
use evdev::{Device, InputEvent};
use crate::KEYBOARD_PATH;
use std::sync::Arc;
use super::EventReader;
use std::any::Any;

pub struct KbdEventReader {
    thread: JoinHandle<()>,
}

impl EventReader for KbdEventReader {
    fn spawn(tx: Arc<Sender<InputEvent>>) -> Self {
        let thread = thread::spawn(move || {
            let mut device = Device::open(KEYBOARD_PATH).unwrap();
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
