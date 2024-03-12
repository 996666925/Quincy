export RUSTY_V8_MIRROR :="https://mirror.ghproxy.com/https://github.com/denoland/rusty_v8/releases/download"
set shell :=["powershell"]
alias e := editor
alias t := tokei

default:
    cargo run --bin QcGame

editor:
    cargo run --bin QcEditor

export:
    cargo run --bin QcExport

build:
    cargo build --release --bin QcGame

@tokei:
    tokei -t Rust,TypeScript