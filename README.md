<div align="center">
# 🐺 Lycanthrope

**Async TCP Connection Hijacker --- Rust/C Hybrid**

[![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/) [![C](https://img.shields.io/badge/C-00599C?style=for-the-badge&logo=c&logoColor=white)](https://en.wikipedia.org/wiki/C_(programming_language)) [![Tokio](https://img.shields.io/badge/Tokio-async-blue?style=for-the-badge)](https://tokio.rs/) [![Platform](https://img.shields.io/badge/Platform-Win%20%7C%20Linux%20%7C%20macOS%20%7C%20Android-green?style=for-the-badge)](#-platform-support) [![License: MIT](https://img.shields.io/badge/License-MIT-yellow?style=for-the-badge)](LICENSE)


``` bash


██╗      ██╗   ██╗ ██████╗ █████╗ ███╗   ██╗████████╗██╗  ██╗██████╗  ██████╗ ██████╗ ███████╗
██║      ██║   ██║██╔════╝██╔══██╗████╗  ██║╚══██╔══╝██║  ██║██╔══██╗██╔═══██╗██╔══██╗██╔════╝
██║      ██║   ██║██║     ███████║██╔██╗ ██║   ██║   ███████║██████╔╝██║   ██║██████╔╝█████╗  
██║      ██║   ██║██║     ██╔══██║██║╚██╗██║   ██║   ██╔══██║██╔══██╗██║   ██║██╔═══╝ ██╔══╝  
███████╗ ╚██████╔╝╚██████╗██║  ██║██║ ╚████║   ██║   ██║  ██║██║  ██║╚██████╔╝██║     ███████╗
╚══════╝  ╚═════╝  ╚═════╝╚═╝  ╚═╝╚═╝  ╚═══╝   ╚═╝   ╚═╝  ╚═╝╚═╝  ╚═╝ ╚═════╝ ╚═╝     ╚══════╝
```
*A high‑performance TCP session manipulation tool built with a hybrid
Rust/C architecture.*
</div>

------------------------------------------------------------------------

## 🔥 Features

### ⚡ Performance

-   Async I/O with Tokio runtime
-   Zero‑copy packet parsing
-   Multi‑threaded sniffing & injection
-   Channel‑based architecture

### 🔧 Hybrid Architecture

-   C core for raw packet construction
-   Rust layer for safety & async logic
-   FFI bridge with safe wrappers
-   RAII resource management

### 🖥 Interface

-   Interactive TUI (ratatui)
-   Headless CLI mode
-   Real‑time packet visualization
-   Color‑coded traffic direction

### 🌍 Cross‑Platform

-   Linux (full support)
-   macOS (BPF)
-   Windows (Npcap)
-   Android (Termux + root)

------------------------------------------------------------------------

## 🏗 Architecture Overview

TUI / CLI\
↓\
Injector (Rust)\
↓\
FFI Bridge\
↓\
C Core (packet.c / checksum.c / rawsock.c)\
↓\
Raw Sockets + libpcap

------------------------------------------------------------------------

## 📁 Project Structure

    lycanthrope/
    ├── Cargo.toml
    ├── build.rs
    ├── csrc/
    │   ├── packet.c
    │   ├── checksum.c
    │   └── rawsock.c
    ├── src/
    │   ├── main.rs
    │   ├── net/
    │   ├── ffi/
    │   └── tui/
    └── tests/

------------------------------------------------------------------------

## 📦 Installation

### Linux

``` bash
sudo apt install libpcap-dev build-essential
cargo build --release
```

### macOS

``` bash
sudo chmod o+r /dev/bpf*
cargo build --release
```

### Windows

-   Install Npcap (WinPcap compatible mode)
-   Set NPCAP_SDK env variable

``` powershell
cargo build --release
```

### Android (Termux)

``` bash
pkg install libpcap rust
tsu
cargo build --release
```

------------------------------------------------------------------------

## 🚀 Usage

``` bash
sudo lycanthrope [OPTIONS] <interface> <src_ip:port> <dst_ip:port>
```

### Examples

``` bash
sudo lycanthrope eth0 192.168.1.10:0 192.168.1.20:23
echo 'id' | sudo lycanthrope eth0 10.0.0.5:4444 10.0.0.1:23
sudo lycanthrope -r eth0 10.0.0.5:4444 10.0.0.1:23
sudo lycanthrope --tui eth0 192.168.1.10:0 192.168.1.20:23
```

------------------------------------------------------------------------

## 🧪 Testing

``` bash
cargo test
sudo cargo test -- --ignored
```

Covers: - IP/TCP checksum validation - Packet construction - SEQ/ACK
tracking - Concurrency safety - FFI boundary checks

------------------------------------------------------------------------

## 🛡️ Safety Notes

All unsafe FFI calls are wrapped safely.\
Raw sockets use RAII (Drop trait).\
SEQ numbers handled with wrapping arithmetic.

------------------------------------------------------------------------

## ⚠️ Legal Disclaimer

For educational and authorized security testing only.\
Do not use without explicit permission.

------------------------------------------------------------------------

## 📄 License

MIT License

------------------------------------------------------------------------

<div align="center">
🐺 Rust + C Systems Engineering\
Because some connections change owners.
</div>