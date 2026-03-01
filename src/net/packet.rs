use std::fmt;
use std::net::SocketAddr;

#[derive(Debug, Clone, Default)]
pub struct TcpFlagSet {
    pub syn: bool,
    pub ack: bool,
    pub fin: bool,
    pub rst: bool,
    pub psh: bool,
    pub urg: bool,
}

impl TcpFlagSet {
    pub fn from_raw(flags: u8) -> Self {
        Self {
            fin: flags & 0x01 != 0,
            syn: flags & 0x02 != 0,
            rst: flags & 0x04 != 0,
            psh: flags & 0x08 != 0,
            ack: flags & 0x10 != 0,
            urg: flags & 0x20 != 0,
        }
    }

    pub fn to_raw(&self) -> u8 {
        let mut f: u8 = 0;
        if self.fin { f |= 0x01; }
        if self.syn { f |= 0x02; }
        if self.rst { f |= 0x04; }
        if self.psh { f |= 0x08; }
        if self.ack { f |= 0x10; }
        if self.urg { f |= 0x20; }
        f
    }
}

impl fmt::Display for TcpFlagSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut flags = Vec::new();
        if self.syn { flags.push("SYN"); }
        if self.ack { flags.push("ACK"); }
        if self.psh { flags.push("PSH"); }
        if self.fin { flags.push("FIN"); }
        if self.rst { flags.push("RST"); }
        if self.urg { flags.push("URG"); }
        write!(f, "[{}]", flags.join("|"))
    }
}

#[derive(Debug, Clone)]
pub struct ParsedPacket {
    pub src: SocketAddr,
    pub dst: SocketAddr,
    pub seq: u32,
    pub ack_num: u32,
    pub flags: TcpFlagSet,
    pub payload: Vec<u8>,
    pub payload_preview: String,
    pub timestamp: chrono::DateTime<chrono::Local>,
    pub direction: PacketDirection,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PacketDirection {
    Incoming,
    Injected,
    AckReply,
}

impl ParsedPacket {
    pub fn new(src: SocketAddr,dst: SocketAddr,seq: u32,ack_num: u32,flags: TcpFlagSet,payload: Vec<u8>,direction: PacketDirection) -> Self {
        let payload_preview = make_preview(&payload, 60);
        Self {
            src,
            dst,
            seq,
            ack_num,
            flags,
            payload,
            payload_preview,
            timestamp: chrono::Local::now(),
            direction,
        }
    }
}

impl fmt::Display for ParsedPacket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let dir = match self.direction {
            PacketDirection::Incoming => "←",
            PacketDirection::Injected => "→",
            PacketDirection::AckReply => "⟲",
        };
        write!(f,"{} {} {} SEQ=0x{:08x} ACK=0x{:08x} {} len={} {}",self.timestamp.format("%H:%M:%S%.3f"),dir,self.flags,self.seq,self.ack_num,self.src,self.payload.len(), self.payload_preview)
    }
}

impl fmt::Display for PacketDirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Incoming => write!(f, "IN "),
            Self::Injected => write!(f, "OUT"),
            Self::AckReply => write!(f, "ACK"),
        }
    }
}

fn make_preview(data: &[u8], max_len: usize) -> String {
    if data.is_empty() {
        return String::new();
    }

    let preview: String = data.iter().take(max_len).map(|&b| {
        if b.is_ascii_graphic() || b == b' ' {
            b as char
        } else if b == b'\n' {
            '↵'
        } else if b == b'\r' {
            ' '
        } else {
            '·'
        }
    }).collect();

    if data.len() > max_len {
        format!("\"{}...\"", preview)
    } else {
        format!("\"{}\"", preview)
    }
}