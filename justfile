export RUSTY_V8_MIRROR :="https://mirror.ghproxy.com/https://github.com/denoland/rusty_v8/releases/download"
set shell :=["powershell"]

default:
    cargo run --bin OvGame


export:
    cargo run --bin OvExport

build:
    cargo build --release --bin OvGame
