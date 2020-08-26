use std::io;
use std::sync::mpsc;
use std::thread;

use termion::event::Key;
use termion::input::TermRead;

#[derive(Clone)]
pub enum Event<I> {
    Input(I),
    Signal(Signal)
}

#[derive(Clone)]
pub enum Signal {
    Quit
}

pub struct Events {
    pub tx: mpsc::Sender<Event<Key>>,
    rx: mpsc::Receiver<Event<Key>>,
    input_handle: thread::JoinHandle<()>,
}

#[derive(Debug, Clone, Copy)]
pub struct Config {}

impl Default for Config {
    fn default() -> Config {
        Config {}
    }
}

impl Events {
    pub fn new() -> Events {
        Events::with_config(Config::default())
    }

    pub fn with_config(_config: Config) -> Events {
        let (tx, rx) = mpsc::channel();
        let input_handle = {
            let tx = tx.clone();
            thread::spawn(move || {
                let stdin = io::stdin();
                for evt in stdin.keys() {
                    if let Ok(key) = evt {
                        if let Err(err) = tx.send(Event::Input(key)) {
                            eprintln!("{}", err);
                            return;
                        }
                    }
                }
            })
        };
        Events { tx, rx, input_handle }
    }

    pub fn next(&self) -> Result<Event<Key>, mpsc::RecvError> {
        self.rx.recv()
    }
}
