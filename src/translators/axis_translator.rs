use crate::event_handler::Translator;
use evdev::{AbsoluteAxisType, EventType, InputEventKind, InputEvent, RelativeAxisType};

pub struct RelAxisTranslator {
    min: i32,
    max: i32,
    mouse_x_pos: i32,
    mouse_y_pos: i32,
}

impl RelAxisTranslator {
    pub fn new(min: i32, max: i32) -> Self {
        RelAxisTranslator {
            min,
            max,
            mouse_x_pos: 0,
            mouse_y_pos: 0,
        }
    }
}

impl Translator for RelAxisTranslator {
    fn translate(&mut self, ev: InputEvent) -> Option<InputEvent> {
        match ev.kind() {
            InputEventKind::RelAxis(RelativeAxisType::REL_X) => {
                self.mouse_x_pos = self.mouse_x_pos + ev.value();
                let mut axis_pos = self.mouse_x_pos;
                if axis_pos < self.min { axis_pos = self.min; }
                if axis_pos > self.max { axis_pos = self.max; }
                return Some(InputEvent::new(EventType::ABSOLUTE, AbsoluteAxisType::ABS_X.0, axis_pos))
            }
            InputEventKind::RelAxis(RelativeAxisType::REL_Y) => {
                self.mouse_y_pos = self.mouse_y_pos + ev.value();
                let mut axis_pos = self.mouse_y_pos;
                if axis_pos < self.min { axis_pos = self.min; }
                if axis_pos > self.max { axis_pos = self.max; }
                return Some(InputEvent::new(EventType::ABSOLUTE, AbsoluteAxisType::ABS_Y.0, axis_pos))
            }
            _ => { None }
        }
    }
}
