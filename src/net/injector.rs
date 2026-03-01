use crate::errors::*;
use crate::ffi::{self, bindings::*, RawSocket};
use crate::net::connection::Connection;
use crate::net::packet::{PacketDirection, ParsedPacket, TcpFlagSet};
use std::net::{Ipv4Addr, SocketAddr};
use std::sync::Arc;
use tokio::sync::mpsc;

pub struct Injector {
    socket: Arc<RawSocket>,
    connection: Connection,
    event_tx: Option<mpsc::Sender<ParsedPacket>>,
}

impl Injector {
    pub fn new(connection: Connection) -> Result<Self> {
        let socket = RawSocket::new()?;
        Ok(Self {
            socket: Arc::new(socket),
            connection,
            event_tx: None,
        })
    }

    pub fn with_event_channel(mut self, tx: mpsc::Sender<ParsedPacket>) -> Self {
        self.event_tx = Some(tx);
        self
    }

    pub fn connection(&self) -> &Connection {
        &self.connection
    }

    pub fn connection_mut(&mut self) -> &mut Connection {
        &mut self.connection
    }

    pub fn socket(&self) -> Arc<RawSocket> {
        Arc::clone(&self.socket)
    }

    pub async fn inject_data(&self, data: &[u8]) -> Result<()> {
        let flags = LYC_TH_ACK | LYC_TH_PSH;
        self.send_tcp(flags, data).await?;

        if let Some(tx) = &self.event_tx {
            let pkt = ParsedPacket::new(self.connection.src,self.connection.dst,self.connection.get_seq().await,self.connection.get_ack().await,TcpFlagSet::from_raw(flags),data.to_vec(),PacketDirection::Injected);
            let _ = tx.send(pkt).await;
        }
        Ok(())
    }

    pub async fn reset(&self) -> Result<()> {
        self.send_tcp(LYC_TH_RST, &[]).await
    }

    pub async fn finish(&self) -> Result<()> {
        self.send_tcp(LYC_TH_ACK | LYC_TH_FIN, &[]).await
    }

    pub async fn desync(&self) -> Result<()> {
        info!("Sending 1KB null desync");
        let data = [0u8; 1024];
        self.send_tcp(LYC_TH_ACK | LYC_TH_PSH, &data).await
    }

    pub async fn ack_packet(&self, their_seq: u32, data_len: u32) -> Result<()> {
        let new_ack = their_seq + data_len;
        self.connection.set_ack(new_ack).await;
        self.send_tcp(LYC_TH_ACK, &[]).await
    }

    async fn send_tcp(&self, flags: u8, data: &[u8]) -> Result<()> {
        let (src_ip, dst_ip) = self.extract_ipv4()?;
        let seq = self.connection.get_seq().await;
        let ack = self.connection.get_ack().await;
        let packet = ffi::build_tcp_packet(src_ip,dst_ip,self.connection.src.port(),self.connection.dst.port(),seq, ack,flags, data)?;
        let socket = Arc::clone(&self.socket);
        let dst_ip_copy = dst_ip;
        let dst_port = self.connection.dst.port();

        tokio::task::spawn_blocking(move || {
            socket.send_packet(&packet, dst_ip_copy, dst_port)
        })
        .await??;

        if !data.is_empty() {
            self.connection.bump_seq(data.len() as u32).await;
        }
        debug!("Sent TCP: flags=0x{:02x} seq=0x{:x} ack=0x{:x} len={}",flags, seq, ack, data.len());
        Ok(())
    }

    fn extract_ipv4(&self) -> Result<(Ipv4Addr, Ipv4Addr)> {
        match (self.connection.src, self.connection.dst) {
            (SocketAddr::V4(s), SocketAddr::V4(d)) => Ok((*s.ip(), *d.ip())),
            _ => bail!(LycError::AddrFamilyMismatch(
                self.connection.src.to_string(),
                self.connection.dst.to_string(),
            )),
        }
    }
}