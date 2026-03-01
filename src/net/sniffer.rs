use crate::errors::*;
use crate::net::connection::Connection;
use pnet::datalink::{self, Channel::Ethernet, NetworkInterface};
use pktparse::{ethernet, ipv4, tcp};
use pktparse::ip::IPProtocol;
use std::net::SocketAddr;
use tokio::sync::mpsc;
use tokio::task;

#[derive(Debug, Clone)]
pub struct CapturedPacket {
    pub src_ip: std::net::IpAddr,
    pub dst_ip: std::net::IpAddr,
    pub src_port: u16,
    pub dst_port: u16,
    pub seq: u32,
    pub ack: u32,
    pub flags: u8,
    pub payload: Vec<u8>,
    pub is_ack: bool,
    pub is_psh: bool,
}

pub async fn sniff_async(interface: String, src: SocketAddr,dst: SocketAddr,tx: mpsc::Sender<CapturedPacket>) -> Result<()> {
    task::spawn_blocking(move || {
        sniff_blocking(&interface, &src, &dst, &tx)
    }).await??;
    Ok(())
}

fn sniff_blocking(interface_name: &str,src: &SocketAddr,dst: &SocketAddr,tx: &mpsc::Sender<CapturedPacket>) -> Result<()> {
    let interfaces = datalink::interfaces();
    let interface = interfaces.into_iter().find(|iface: &NetworkInterface| iface.name == *interface_name).context("Interface not found")?;

    let (_, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(_tx, rx)) => (_tx, rx),
        Ok(_) => bail!("Unhandled channel type"),
        Err(e) => bail!("Datalink channel error: {}", e),
    };

    while let Ok(packet) = rx.next() {
        if let Ok((remaining, eth_frame)) = ethernet::parse_ethernet_frame(packet) {
            if eth_frame.ethertype != ethernet::EtherType::IPv4 {
                continue;
            }

            if let Ok((remaining, ip_hdr)) = ipv4::parse_ipv4_header(remaining) {
                let pkt_src: std::net::IpAddr = ip_hdr.source_addr.into();
                let pkt_dst: std::net::IpAddr = ip_hdr.dest_addr.into();

                if pkt_src != src.ip() || pkt_dst != dst.ip() {
                    continue;
                }

                if ip_hdr.protocol != IPProtocol::TCP {
                    continue;
                }

                if let Ok((remaining, tcp_hdr)) = tcp::parse_tcp_header(remaining) {
                    if src.port() != 0 && tcp_hdr.source_port != src.port() {
                        continue;
                    }
                    if dst.port() != 0 && tcp_hdr.dest_port != dst.port() {
                        continue;
                    }

                    let captured = CapturedPacket {
                        src_ip: pkt_src,
                        dst_ip: pkt_dst,
                        src_port: tcp_hdr.source_port,
                        dst_port: tcp_hdr.dest_port,
                        seq: tcp_hdr.sequence_no,
                        ack: tcp_hdr.ack_no,
                        flags: 0, // можно собрать из полей
                        payload: remaining.to_vec(),
                        is_ack: tcp_hdr.flag_ack,
                        is_psh: tcp_hdr.flag_psh,
                    };

                    if tx.blocking_send(captured).is_err() {
                        break;
                    }
                }
            }
        }
    }

    Ok(())
}

pub async fn get_seq_ack(interface: String,src: SocketAddr,dst: SocketAddr) -> Result<Connection> {
    let (tx, mut rx) = mpsc::channel::<CapturedPacket>(100);
    let sniffer = tokio::spawn(sniff_async(interface, src, dst, tx));

    while let Some(pkt) = rx.recv().await {
        if pkt.is_ack {
            let seq = pkt.seq + pkt.payload.len() as u32;
            let ack = pkt.ack;
            let real_src = SocketAddr::new(pkt.src_ip, pkt.src_port);
            let real_dst = SocketAddr::new(pkt.dst_ip, pkt.dst_port);
            sniffer.abort();
            return Ok(Connection::new(real_src, real_dst, seq, ack));
        }
    }

    bail!("Sniffer closed without finding SEQ/ACK")
}