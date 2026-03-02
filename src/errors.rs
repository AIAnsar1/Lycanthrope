pub use anyhow::{Context, Error, Result, anyhow, bail};
pub use tracing::{debug, error, info, trace, warn};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum LycError {
    #[error("Raw socket creation failed: {0}")]
    SocketCreate(String),

    #[error("Packet send failed: {0}")]
    SendFailed(String),

    #[error("Network init failed: {0}")]
    NetworkInit(String),

    #[error("Packet build failed (code {0})")]
    PacketBuild(i32),

    #[error("Interface '{0}' not found")]
    InterfaceNotFound(String),

    #[error("Sniffer closed without result")]
    SnifferClosed,

    #[error("Address family mismatch: src={0}, dst={1}")]
    AddrFamilyMismatch(String, String),

    #[error("Platform not supported for this operation: {0}")]
    PlatformUnsupported(String),
}
