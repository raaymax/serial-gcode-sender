# Serial gcode sender

Simple tool for sending your gcode files to your CNC

## Setup
```sh
cargo build --release
export SERVICE_ACCOUNT="<your_secret_key>.json" # only if gcs integration needed
target/release/laser --help
```

## Usage
```sh
# laser --help
USAGE:
    laser <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    gcs       Google cloud storage
    help      Prints this message or the help of the given subcommand(s)
    ls        List all serial ports
    stream    Stream commands to serial device
```

## Licence
MIT
