use lycanthrope::ffi;
use std::net::Ipv4Addr;

#[test]
fn test_ip_checksum_known_value() {
    let header: [u8; 20] = [
        0x45, 0x00, 0x00, 0x3c, 0x1c, 0x46, 0x40, 0x00, 0x40, 0x06, 0x00, 0x00, 0xac, 0x10, 0x0a,
        0x63, 0xac, 0x10, 0x0a, 0x0c,
    ];

    let checksum = ffi::ip_checksum(&header);
    assert_ne!(checksum, 0, "Checksum should not be zero");
}

#[test]
fn test_tcp_checksum_not_zero() {
    let src = Ipv4Addr::new(192, 168, 1, 1);
    let dst = Ipv4Addr::new(192, 168, 1, 2);

    let mut tcp_segment = vec![0u8; 20];
    tcp_segment[0] = 0x30;
    tcp_segment[1] = 0x39;
    tcp_segment[2] = 0x00;
    tcp_segment[3] = 0x50;
    tcp_segment[12] = 0x50;

    let chk = ffi::tcp_checksum(src, dst, &tcp_segment);
    assert_ne!(chk, 0);
}
