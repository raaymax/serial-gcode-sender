use std::io;
use structopt::StructOpt;
use std::fs::File;

use gcs::Storage;
use serial::Serial;

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
        #[structopt(short = "p", long = "port", help = "Serial port")]
        port: Option<String>,
        #[structopt(subcommand)]
        cmd: Option<StreamCommands>,
     },
     #[structopt(about = "Google cloud storage")]
     Gcs {
        #[structopt(subcommand, help = "Operation")]
        cmd: StorageCommands,
     }
}

#[derive(StructOpt, Debug)]
enum StreamCommands {
    #[structopt(about = "Stream commands from file")]
    File{
        path: std::path::PathBuf,
    },
    #[structopt(about = "Stream commands from gcs file")]
    Gcs{
        path: String
    },
}

#[derive(StructOpt, Debug)]
enum StorageCommands {
    #[structopt(about = "List all files")]
    Ls,
}

#[tokio::main]
async fn main() -> Result<(), String> {
    let cli = Cli::from_args();
    match cli.cmd {
        Command::Ls => {
            serial::available_ports();
        },
        Command::Stream{port, baud, timeout, cmd} => {
            let connection = Serial::connect(port, baud, timeout);
            match cmd {
                Some(StreamCommands::File{path}) => {
                    let f = File::open(path).map_err(|e|e.to_string())?;
                    connection.send(&mut io::BufReader::new(f))
                },
                Some(StreamCommands::Gcs{path}) => {
                    let mut b = Storage::read(path).await;
                    connection.send(&mut b);
                }
                _ => {
                    connection.send(&mut io::stdin().lock())
                }
            }
        },
        Command::Gcs{cmd} => {
            match cmd {
                StorageCommands::Ls => {
                    Storage::list().await?;
                }
            }
        }
    }
    Ok(())
}

