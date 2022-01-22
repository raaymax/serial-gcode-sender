# Laser cli tool

Laser gcode streaming tool for laser cutter.

This tool can stream files directly from Google Cloud Storage.

## Setup
```sh
cargo build --release
export SERVICE_ACCOUNT="<your_secret_key>.json" # only if gcs integration needed
target/release/laser --help
```
