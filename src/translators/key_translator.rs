use crate::event_handler::Translator;
use evdev::{Key, InputEvent, EventType, InputEventKind};
use std::collections::HashMap;


pub struct KeyTranslator {
    binds: HashMap<Key, Key>,
}

impl KeyTranslator {
    pub fn new(binds: HashMap<Key, Key>) -> Self {
        KeyTranslator {
            binds,
        }
    }
}

impl Translator for KeyTranslator {
    fn translate(&mut self, ev: InputEvent) -> Option<InputEvent> {
        if let InputEventKind::Key(input_key) = ev.kind() {
            if let Some(output_key) = self.binds.get(&input_key) {
                return Some(InputEvent::new(EventType::KEY, output_key.0, ev.value()))
            }
        }
        None
    }
}
