use crate::net::packet::{PacketDirection, ParsedPacket};
use ratatui::widgets::ListState;

#[derive(Debug)]
pub struct App {
    pub packets: Vec<ParsedPacket>,
    pub list_state: ListState,
    pub status_log: Vec<String>,
    pub status: String,
    pub input_buffer: String,
    pub is_hijacked: bool,
    pub running: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            packets: Vec::new(),
            list_state: ListState::default(),
            status_log: vec!["Lycanthrope started.".to_string()],
            status: "Initializing...".to_string(),
            input_buffer: String::new(),
            is_hijacked: false,
            running: true,
        }
    }

    pub fn add_packet(&mut self, pkt: ParsedPacket) {
        self.packets.push(pkt);
        let len = self.packets.len();
        self.list_state.select(Some(len.saturating_sub(1)));
    }

    pub fn add_status(&mut self, msg: &str) {
        let ts = chrono::Local::now().format("%H:%M:%S");
        self.status_log.push(format!("[{}] {}", ts, msg));
        if self.status_log.len() > 200 {
            self.status_log.drain(..100);
        }
    }

    pub fn scroll_up(&mut self) {
        let i = self.list_state.selected().unwrap_or(0);
        self.list_state.select(Some(i.saturating_sub(1)));
    }

    pub fn scroll_down(&mut self) {
        let max = self.packets.len().saturating_sub(1);
        let i = self.list_state.selected().unwrap_or(0);
        self.list_state.select(Some((i + 1).min(max)));
    }

    pub fn packet_count_by_dir(&self, dir: PacketDirection) -> usize {
        self.packets.iter().filter(|p| p.direction == dir).count()
    }
}
