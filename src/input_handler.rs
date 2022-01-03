use std::fs::File;
use std::io::{self, BufRead};

use std::sync::mpsc::{Receiver, Sender};
use crate::events::*;

pub struct InputHandler {
    tx: Sender<Event>,
    rx: Receiver<EventCommand>,
    path: Option<std::path::PathBuf>,
}

impl InputHandler {
    pub fn new(tx: Sender<Event>,rx: Receiver<EventCommand>,path: Option<std::path::PathBuf>) -> InputHandler {
        InputHandler { tx, rx, path }
    }
    pub fn handle_input(&self) {
        if let Some(path) = &self.path {
            let f = File::open(path).unwrap();
            self.handle_commands(&mut io::BufReader::new(f));
        } else {
            self.handle_commands(&mut io::stdin().lock());
        };
    }

    fn handle_commands(&self, b: &mut dyn BufRead) {
        let mut input = b.lines();
        
        loop {
            let ev = self.rx.recv().unwrap();
            match ev {
                EventCommand::Quit => break, 
                EventCommand::Next => {
                    let next = input.next();
                    if let Some(ev) = next {
                        self.tx.send(Event::Input(ev.unwrap())).unwrap()
                    } else {
                        self.tx.send(Event::EOF).unwrap();
                    }
                },
            }
        }
    }
}
