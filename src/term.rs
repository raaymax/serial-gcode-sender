use std::io::{self, BufRead, Write};

use std::time::Duration;
use std::sync::mpsc::{channel, Receiver, Sender};
use regex::Regex;
use crate::input_handler::InputHandler;
use crate::events::*;
use crate::config::Config;

pub struct Term {
    cfg: Config,
    serial: Box<dyn serialport::SerialPort>,
}

impl Term {
    pub fn init(cfg: Config) -> Term {
        let serial = serialport::new(cfg.port.clone().unwrap(), cfg.baud)
            .timeout(Duration::from_millis(cfg.timeout))
            .open()
            .expect("Cannot open serial port");
        Term { cfg, serial }
    }

    fn handle_events(&mut self, tx: Sender<EventCommand>, rx: Receiver<Event>) {
        loop {
            let event = rx.recv().unwrap();
            match event {
                Event::EOF => {
                    println!("End of file - exiting.");
                    tx.send(EventCommand::Quit).unwrap();
                    break;
                },
                Event::Data(text) if text.trim() == "start" => {
                    Term::print(&text);
                    tx.send(EventCommand::Next).unwrap();
                },
                Event::Data(text) if Regex::new(r".*Error.*").unwrap().is_match(text.as_str()) => {
                    println!("< {}", text);
                    println!("Error occured while processing command");
                    println!("Exiting");
                    tx.send(EventCommand::Quit).unwrap();
                    break;
                },
                Event::Data(text) => {
                    Term::print(&text);
                    if text.trim() == "ok" {
                        tx.send(EventCommand::Next).unwrap();
                    }
                },
                Event::Input(text) if text == ":q" => {
                    tx.send(EventCommand::Quit).unwrap();
                    break;
                },
                Event::Input(text) if text.len() > 1 && &text[..1] == ";" => {
                    tx.send(EventCommand::Next).unwrap();
                },
                Event::Input(text) if text.is_empty() => {
                    tx.send(EventCommand::Next).unwrap();
                },
                Event::Input(text) => {
                    println!("> {}", text);
                    let cmd = text.to_owned() + "\n";
                    self.serial.write(cmd.as_bytes()).unwrap();
                    self.serial.flush().unwrap();
                    io::stdout().flush().unwrap();
                }
            }
        }
    }

    fn print(text: &String) {
        print!("< {}", text);
        io::stdout().flush().unwrap();
    }

    pub fn watch(&mut self) {
        let (etx, erx) = channel();
        let (ctx, crx) = channel();
        self.handle_output(etx.clone());
        self.handle_input(etx.clone(), crx);
        self.handle_events(ctx, erx);
    }

    fn handle_output(&self, tx: Sender<Event>) {
        let serial = self.serial.try_clone().unwrap();
        std::thread::spawn(move || {
            let mut buf = io::BufReader::new(serial); 
            loop {
                let mut serial_buf = String::new();
                buf.read_line(&mut serial_buf).ok();
                let clean = serial_buf.trim_matches(|c| c == 0x0 as char);
                if !clean.is_empty() {
                    tx.send(Event::Data(clean.to_string())).unwrap();
                }
            }
        });
    }

    fn handle_input(&self, tx: Sender<Event>, rx: Receiver<EventCommand>) {
        let input_handler = InputHandler::new(tx, rx, self.cfg.input.clone());
        std::thread::spawn(move || {
            input_handler.handle_input();
        });
    }
}
