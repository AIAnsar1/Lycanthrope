use lycanthrope::net::connection::Connection;
use std::net::SocketAddr;

#[tokio::test]
async fn test_connection_seq_ack_tracking() {
    let src: SocketAddr = "10.0.0.1:4444".parse().unwrap();
    let dst: SocketAddr = "10.0.0.2:80".parse().unwrap();
    let conn = Connection::new(src, dst, 1000, 2000);

    assert_eq!(conn.get_seq().await, 1000);
    assert_eq!(conn.get_ack().await, 2000);

    conn.bump_seq(100).await;
    assert_eq!(conn.get_seq().await, 1100);

    conn.set_ack(3000).await;
    assert_eq!(conn.get_ack().await, 3000);
}

#[tokio::test]
async fn test_connection_clone_shares_state() {
    let conn1 = Connection::new(
        "10.0.0.1:4444".parse().unwrap(),
        "10.0.0.2:80".parse().unwrap(),
        500,
        600,
    );
    let conn2 = conn1.clone();

    conn1.bump_seq(50).await;
    assert_eq!(conn2.get_seq().await, 550);
}

#[tokio::test]
async fn test_seq_wrapping() {
    let conn = Connection::new(
        "10.0.0.1:1234".parse().unwrap(),
        "10.0.0.2:80".parse().unwrap(),
        0xFFFFFF00,
        0,
    );

    conn.bump_seq(0x200).await;
    assert_eq!(conn.get_seq().await, 0x100);
}

#[tokio::test]
async fn test_is_duplicate() {
    let conn = Connection::new(
        "10.0.0.1:1234".parse().unwrap(),
        "10.0.0.2:80".parse().unwrap(),
        0,
        0x3000,
    );

    assert!(conn.is_duplicate(0x2000, 500).await);
    assert!(!conn.is_duplicate(0x4000, 100).await);
}

#[tokio::test]
async fn test_concurrent_access() {
    let conn = Connection::new(
        "10.0.0.1:1234".parse().unwrap(),
        "10.0.0.2:80".parse().unwrap(),
        0,
        0,
    );

    let mut handles = vec![];
    for _ in 0..10 {
        let c = conn.clone();
        handles.push(tokio::spawn(async move {
            c.bump_seq(1).await;
        }));
    }

    for h in handles {
        h.await.unwrap();
    }

    assert_eq!(conn.get_seq().await, 10);
}
