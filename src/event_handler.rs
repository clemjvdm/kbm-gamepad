use evdev::{InputEvent, uinput::VirtualDevice};
use std::sync::mpsc::Receiver;

pub trait Translator {
    fn translate(&mut self, ev: InputEvent) -> Option<InputEvent>;
}

// TODO: Maybe use 'translators: Vec<Box<Translator>>' over current implementation
pub struct EventHandler {
    gamepad: VirtualDevice,
    translators: Vec<Box<dyn Translator>>,
    event_queue: Receiver<InputEvent>,
    stopped: bool,
}

impl EventHandler {
    pub fn new(gamepad: VirtualDevice, event_queue: Receiver<InputEvent>) -> Self {
        EventHandler {
            gamepad,
            translators: Vec::new(),
            event_queue,
            stopped: true,
        }
    }

    pub fn add_translator(&mut self, translator: Box<dyn Translator>) {
        self.translators.push(translator);
    }

    pub fn start(&mut self) {
        self.stopped = false;
        while !self.stopped {
            let ev = self.event_queue.recv().unwrap();
            let mut translated_ev: Vec<InputEvent> = Vec::new();
            for translator in self.translators.iter_mut() {
                translator.translate(ev).inspect(|ev| translated_ev.push(*ev));
            }
            self.gamepad.emit(&translated_ev).unwrap();
        }
    }
}
