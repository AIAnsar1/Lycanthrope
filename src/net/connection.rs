use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Clone)]
pub struct Connection {
    pub src: SocketAddr,
    pub dst: SocketAddr,
    seq: Arc<Mutex<u32>>,
    ack: Arc<Mutex<u32>>,
}

impl Connection {
    pub fn new(src: SocketAddr, dst: SocketAddr, seq: u32, ack: u32) -> Self {
        Self {
            src,
            dst,
            seq: Arc::new(Mutex::new(seq)),
            ack: Arc::new(Mutex::new(ack)),
        }
    }

    pub async fn get_seq(&self) -> u32 {
        *self.seq.lock().await
    }

    pub async fn get_ack(&self) -> u32 {
        *self.ack.lock().await
    }

    pub async fn bump_seq(&self, inc: u32) {
        let mut guard = self.seq.lock().await;
        *guard = guard.wrapping_add(inc);
    }

    pub async fn set_ack(&self, ack: u32) {
        let mut guard = self.ack.lock().await;
        *guard = ack;
    }

    pub async fn is_duplicate(&self, pkt_seq: u32, pkt_len: u32) -> bool {
        let current_ack = self.get_ack().await;
        current_ack >= pkt_seq.wrapping_add(pkt_len)
    }
}
