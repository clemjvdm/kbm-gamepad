use std::sync::mpsc::Sender;
use evdev::InputEvent;
use std::any::Any;
use super::kbd_event_reader::KbdEventReader;

pub enum EventReader {
    KeyEventReader(KbdEventReader),
    RelAxisEventReader(super::MouseEventReader),
}

pub trait EventReaderTrait {
    fn spawn(tx: Sender<InputEvent>) -> Self;
    fn join(self) -> Result<(), Box<dyn Any + Send + 'static>>; // TODO: change join to stop, to
                                                                // make infine loops stop.
}
