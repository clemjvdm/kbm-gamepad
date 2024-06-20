use std::sync::mpsc::Sender;
use evdev::InputEvent;
use std::sync::Arc;
use std::any::Any;

pub trait EventReader {
    fn spawn(tx: Arc<Sender<InputEvent>>) -> Self;
    fn join(self) -> Result<(), Box<dyn Any + Send + 'static>>;
}
