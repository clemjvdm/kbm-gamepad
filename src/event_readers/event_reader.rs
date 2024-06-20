use std::sync::mpsc::Sender;
use evdev::InputEvent;
use std::any::Any;

pub trait EventReader {
    fn spawn(tx: Sender<InputEvent>) -> Self;
    fn join(self) -> Result<(), Box<dyn Any + Send + 'static>>; // TODO: change join to stop, to
                                                                // make infine loops stop.
}
