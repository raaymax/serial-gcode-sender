mod input_handler;
mod events;
mod term;
mod config;

use std::io;

use structopt::StructOpt;
use term::Term;
use config::Config;


#[derive(StructOpt, Debug)]
struct Cli {
    #[structopt(subcommand, help = "Operation")]
    cmd: Command,
}

#[derive(StructOpt, Debug)]
enum Command {
    #[structopt(about = "List all serial ports")]
    Ls,
    #[structopt(about = "Stream commands to serial device")]
    Stream {
        #[structopt(short = "b", long = "baud", default_value = "115200", help = "Baud rate")]
        baud: u32,
        #[structopt(short = "t", long = "timeout", default_value = "1000", help = "Serial port timeout")]
        timeout: u64,
        #[structopt(help = "Serial port")]
        port: Option<String>,
        #[structopt(parse(from_os_str), short = "i", long = "input", help = "Input file otherwise stdin")]
        input: Option<std::path::PathBuf>,
     }
}

impl Cli {
    fn get_config(&self) -> Option<Config> {
        if let Command::Stream {ref port, timeout, baud, ref input} = self.cmd {
            if let None = port {
                let ports = serialport::available_ports().expect("No ports found!");
                let p = ports.first().map(|f| f.port_name.clone());
                return Some(Config {port: p, timeout, baud, input: input.clone()});
            }
            return Some(Config {port: port.clone(), timeout, baud, input: input.clone()});
        }
        return None;
    }
}


fn main() -> io::Result<()> {
    let cli = Cli::from_args();
    match cli.cmd {
        Command::Ls => {
            println!("Available ports:");
            for ports in serialport::available_ports().into_iter() {
                for port in ports {
                    println!(" - {}", port.port_name);
                    if let serialport::SerialPortType::UsbPort(info) = port.port_type {
                        println!("\tproduct: {}", info.product.unwrap_or("None".to_string()));
                        println!("\tmanufacturer: {}", info.manufacturer.unwrap_or("None".to_string()));
                        println!("\tserial: {}", info.serial_number.unwrap_or("None".to_string()));
                    }

                }
            }
        },
        Command::Stream{..} => {
            let cfg = cli.get_config().unwrap();
            if cfg.port == None {
                println!("No serial port");
                return Ok(());
            }
            let mut term = Term::init(cfg);
            term.watch();
        },
    }
    Ok(())
}

