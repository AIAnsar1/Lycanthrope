use lycanthrope::ffi::bindings::*;
use lycanthrope::ffi::{self, RawSocket};
use lycanthrope::net::connection::Connection;
use lycanthrope::net::injector::Injector;
use std::net::Ipv4Addr;

#[tokio::test]
async fn test_injector_creation() {
    let conn = Connection::new(
        "10.0.0.1:4444".parse().unwrap(),
        "10.0.0.2:80".parse().unwrap(),
        1000,
        2000,
    );

    match Injector::new(conn) {
        Ok(inj) => {
            assert_eq!(inj.connection().src.port(), 4444);
        }
        Err(e) => {
            let msg = format!("{}", e);
            assert!(
                msg.contains("root")
                    || msg.contains("permission")
                    || msg.contains("denied")
                    || msg.contains("Operation not permitted")
                    || msg.contains("10013"),
                "Unexpected error: {}",
                e
            );
        }
    }
}

#[tokio::test]
#[ignore]
async fn test_raw_socket_send() {
    let socket = RawSocket::new().expect("Need root/admin");

    let packet = ffi::build_tcp_packet(
        Ipv4Addr::new(127, 0, 0, 1),
        Ipv4Addr::new(127, 0, 0, 1),
        11111,
        22222,
        1000,
        0,
        LYC_TH_RST,
        &[],
    )
    .unwrap();

    let sent = socket
        .send_packet(&packet, Ipv4Addr::new(127, 0, 0, 1), 22222)
        .expect("Send failed");

    assert_eq!(sent, packet.len());
}
