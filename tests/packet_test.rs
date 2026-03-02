use lycanthrope::ffi;
use lycanthrope::ffi::bindings::*;
use lycanthrope::net::packet::{PacketDirection, TcpFlagSet};
use std::net::Ipv4Addr;

#[test]
fn test_build_packet_syn() {
    let pkt = ffi::build_tcp_packet(
        Ipv4Addr::new(10, 0, 0, 1),
        Ipv4Addr::new(10, 0, 0, 2),
        12345,
        80,
        100,
        0,
        LYC_TH_SYN,
        &[],
    )
    .expect("SYN packet should build");

    assert_eq!(pkt.len(), 40);
    assert_eq!(pkt[0] >> 4, 4);
    assert_eq!(pkt[9], 6);
}

#[test]
fn test_build_packet_with_payload() {
    let payload = b"GET / HTTP/1.1\r\nHost: target\r\n\r\n";
    let pkt = ffi::build_tcp_packet(
        Ipv4Addr::new(192, 168, 1, 100),
        Ipv4Addr::new(192, 168, 1, 1),
        4444,
        80,
        0xDEADBEEF,
        0xCAFEBABE,
        LYC_TH_ACK | LYC_TH_PSH,
        payload,
    )
    .expect("PSH+ACK packet should build");

    assert_eq!(pkt.len(), 40 + payload.len());
}

#[test]
fn test_build_packet_empty_payload() {
    let pkt = ffi::build_tcp_packet(
        Ipv4Addr::new(127, 0, 0, 1),
        Ipv4Addr::new(127, 0, 0, 1),
        9999,
        22,
        555,
        0,
        LYC_TH_RST,
        &[],
    )
    .expect("RST should build");

    assert_eq!(pkt.len(), 40);
}

#[test]
fn test_ip_checksum_verification() {
    let pkt = ffi::build_tcp_packet(
        Ipv4Addr::new(172, 16, 0, 1),
        Ipv4Addr::new(172, 16, 0, 2),
        8080,
        443,
        1000,
        2000,
        LYC_TH_ACK,
        b"test",
    )
    .unwrap();

    let chk = ffi::ip_checksum(&pkt[..20]);
    assert_eq!(chk, 0, "IP checksum verification should be 0");
}

#[test]
fn test_packet_structure_integrity() {
    let pkt = ffi::build_tcp_packet(
        Ipv4Addr::new(192, 168, 1, 100),
        Ipv4Addr::new(192, 168, 1, 200),
        31337,
        23,
        0xAABBCCDD,
        0x11223344,
        LYC_TH_ACK | LYC_TH_PSH,
        b"INJECT_TEST",
    )
    .unwrap();

    assert_eq!(pkt[0], 0x45);
    assert_eq!(&pkt[12..16], &[192, 168, 1, 100]);
    assert_eq!(&pkt[16..20], &[192, 168, 1, 200]);
    assert_eq!(pkt[20], 0x7A);
    assert_eq!(pkt[21], 0x69);
    assert_eq!(&pkt[24..28], &[0xAA, 0xBB, 0xCC, 0xDD]);
    assert_eq!(pkt[33], 0x18);
    assert_eq!(&pkt[40..], b"INJECT_TEST");
}

#[test]
fn test_tcp_flagset_roundtrip() {
    let flags = TcpFlagSet {
        syn: true,
        ack: true,
        fin: false,
        rst: false,
        psh: true,
        urg: false,
    };

    let raw = flags.to_raw();
    assert_eq!(raw, LYC_TH_SYN | LYC_TH_ACK | LYC_TH_PSH);

    let back = TcpFlagSet::from_raw(raw);
    assert!(back.syn && back.ack && back.psh);
    assert!(!back.fin && !back.rst);
}

#[test]
fn test_tcp_flagset_display() {
    let flags = TcpFlagSet::from_raw(LYC_TH_SYN | LYC_TH_ACK);
    let s = format!("{}", flags);
    assert!(s.contains("SYN"));
    assert!(s.contains("ACK"));
}

#[test]
fn test_packet_direction_display() {
    assert_eq!(format!("{}", PacketDirection::Incoming), "IN ");
    assert_eq!(format!("{}", PacketDirection::Injected), "OUT");
    assert_eq!(format!("{}", PacketDirection::AckReply), "ACK");
}

#[test]
fn test_network_init_cleanup() {
    ffi::network_init().expect("init failed");
    ffi::network_cleanup();
    ffi::network_init().expect("second init failed");
    ffi::network_cleanup();
}
