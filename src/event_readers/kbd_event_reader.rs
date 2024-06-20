use std::sync::mpsc::Sender;
use std::{thread, thread::JoinHandle};
use evdev::{Device, InputEvent};
use crate::KEYBOARD_PATH;
use super::EventReaderTrait;
use std::any::Any;

pub struct KbdEventReader {
    thread: JoinHandle<()>,
}

impl EventReaderTrait for KbdEventReader {
    fn spawn(tx: Sender<InputEvent>) -> Self {
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
