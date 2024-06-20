use evdev::{InputEvent, uinput::VirtualDevice};
use std::sync::mpsc::Receiver;
use std::{thread, thread::JoinHandle};
use std::any::Any;
use std::fmt;
use std::sync::{Arc, Mutex};

pub trait Translator: Send {
    fn translate(&mut self, ev: InputEvent) -> Option<InputEvent>;
}

#[derive(Debug)]
pub enum EventHandlerError {
    ThreadNotStartedError,
    ThreadRunningError,
}

impl fmt::Display for EventHandlerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ThreadNotStartedError => {
                write!(f, "Attemped to stop non-started thread.")
            }
            Self::ThreadRunningError => {
                write!(f, "Attempted to add translator while event handler's thread was already running")
            }
            
        }
    }
}

pub struct EventHandler {
    thread: Option<JoinHandle<()>>,
    gamepad: Arc<Mutex<VirtualDevice>>,
    translators: Arc<Mutex<Vec<Box<dyn Translator>>>>,
}

impl EventHandler {
    pub fn new(gamepad: VirtualDevice) -> Self {
        EventHandler {
            thread: None,
            gamepad: Arc::new(Mutex::new(gamepad)),
            translators: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn start(&mut self, rx: Receiver<InputEvent>) {
        let translators = Arc::clone(&self.translators);
        let gamepad = Arc::clone(&self.gamepad);
        let thread = thread::spawn(move || {
            loop {
                let ev = rx.recv().unwrap();
                let mut translated_ev: Vec<InputEvent> = Vec::new();
                for translator in translators.lock().unwrap().iter_mut() {
                    translator.translate(ev).inspect(|ev| translated_ev.push(*ev));
                }
                gamepad.lock().unwrap().emit(&translated_ev).unwrap();
            }

        });
        self.thread = Some(thread);
    }

    pub fn add_translator(&mut self, translator: Box<dyn Translator>) -> Result<(),EventHandlerError> {
        if let Ok(mut translators) = self.translators.try_lock() {
            translators.push(translator);
            Ok(())
        } else {
            Err(EventHandlerError::ThreadRunningError)
        }
    }

    pub fn stop(self) -> Result<(), Box<dyn Any + Send + 'static>> {
        if let Some(thread) = self.thread {
            thread.join()
        } else {
            Err(Box::new(EventHandlerError::ThreadNotStartedError))
        }
    }
}
