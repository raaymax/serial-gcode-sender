use std::io::{self, BufRead, Write};

use std::time::Duration;
use std::sync::mpsc::{channel, Receiver, Sender};
use regex::Regex;
use crate::events::*;

pub struct Serial {
    serial: Box<dyn serialport::SerialPort>,
    input: (Sender<Event>, Receiver<EventCommand>),
}

impl Serial {
    pub fn connect(port_opt: Option<String>, baud: u32, timeout: u64) -> Serial {
        let port = Serial::get_port(port_opt);
        let serial = serialport::new(port.unwrap(), baud)
            .timeout(Duration::from_millis(timeout))
            .open()
            .expect("Cannot open serial port");
        let (etx, erx) = channel();
        let (ctx, crx) = channel();
        let mut inst = Serial { serial, input: (etx.clone(), crx)};
        inst.handle_events(ctx.clone(), erx);
        inst.handle_output(etx);
        inst
    }

    fn get_port(port_opt: Option<String>) -> Option<String> {
        if let None = port_opt {
            let ports = serialport::available_ports().expect("No ports found!");
            let p = ports.first().map(|f| f.port_name.clone());
            return p;
        }
        return port_opt;
    }

    fn handle_events(&mut self, tx: Sender<EventCommand>, rx: Receiver<Event>) {
        let mut serial = self.serial.try_clone().unwrap();
        std::thread::spawn(move || {
            loop {
                let event = rx.recv().unwrap();
                match event {
                    Event::EOF => {
                        println!("End of file - exiting.");
                        tx.send(EventCommand::Quit).unwrap();
                        break;
                    },
                    Event::Data(text) if text.trim() == "start" => {
                        Serial::print(&text);
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
                        Serial::print(&text);
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
                        serial.write(cmd.as_bytes()).unwrap();
                        serial.flush().unwrap();
                        io::stdout().flush().unwrap();
                    }
                }
            }
        });
    }

    fn print(text: &String) {
        print!("< {}", text);
        io::stdout().flush().unwrap();
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

    pub fn send(&self, b: &mut dyn BufRead) {
        let (tx, rx) = &self.input;
        let mut input = b.lines();
       
        loop {
            let ev = rx.recv().unwrap();
            match ev {
                EventCommand::Quit => break, 
                EventCommand::Next => {
                    let next = input.next();
                    if let Some(ev) = next {
                        tx.send(Event::Input(ev.unwrap())).unwrap()
                    } else {
                        tx.send(Event::EOF).unwrap();
                    }
                },
            }
        }
    }
}
