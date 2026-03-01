use crate::net::packet::PacketDirection;
use crate::tui::app::App;
use ratatui::{layout::{Constraint, Direction, Layout, Rect},style::{Color, Modifier, Style},text::{Line, Span},widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},Frame};
const CLR_INCOMING: Color = Color::Cyan;
const CLR_INJECTED: Color = Color::Magenta;
const CLR_ACK: Color = Color::DarkGray;
const CLR_BORDER: Color = Color::Green;
const CLR_TITLE: Color = Color::LightGreen;
const CLR_STATUS: Color = Color::Yellow;
const CLR_INPUT: Color = Color::White;
const CLR_BANNER: Color = Color::LightGreen;

pub fn draw(frame: &mut Frame, app: &App) {
    let chunks = Layout::default().direction(Direction::Vertical).constraints([
        Constraint::Length(3),
        Constraint::Min(10),
        Constraint::Length(5),
        Constraint::Length(3),
        Constraint::Length(1),
    ]).split(frame.area());
    draw_header(frame, app, chunks[0]);
    draw_packet_list(frame, app, chunks[1]);
    draw_status_log(frame, app, chunks[2]);
    draw_input(frame, app, chunks[3]);
    draw_hotkeys(frame, chunks[4]);
}

fn draw_header(frame: &mut Frame, app: &App, area: Rect) {
    let in_count = app.packet_count_by_dir(PacketDirection::Incoming);
    let out_count = app.packet_count_by_dir(PacketDirection::Injected);
    let header = Paragraph::new(Line::from(vec![
        Span::styled(" 🐺 LYCANTHROPE ", Style::default().fg(CLR_BANNER).add_modifier(Modifier::BOLD)),
        Span::raw("│ "),
        Span::styled(&app.status, Style::default().fg(CLR_STATUS)),
        Span::raw(" │ "),
        Span::styled(format!("IN:{}", in_count), Style::default().fg(CLR_INCOMING)),
        Span::raw(" "),
        Span::styled(format!("OUT:{}", out_count), Style::default().fg(CLR_INJECTED)),
        Span::raw(" │ "),
        Span::styled(format!("Total: {}", app.packets.len()),Style::default().fg(Color::White)),
    ])).block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(CLR_BORDER)).title(" Status "));
    frame.render_widget(header, area);
}

fn draw_packet_list(frame: &mut Frame, app: &App, area: Rect) {
    let items: Vec<ListItem> = app.packets.iter().map(|pkt| {
        let color = match pkt.direction {
            PacketDirection::Incoming => CLR_INCOMING,
            PacketDirection::Injected => CLR_INJECTED,
            PacketDirection::AckReply => CLR_ACK,
        };

        let dir_symbol = match pkt.direction {
            PacketDirection::Incoming => "◄──",
            PacketDirection::Injected => "──►",
            PacketDirection::AckReply => " ⟲ ",
        };

        let line = Line::from(vec![
            Span::styled(pkt.timestamp.format("%H:%M:%S ").to_string(),Style::default().fg(Color::DarkGray)),
            Span::styled(dir_symbol,Style::default().fg(color).add_modifier(Modifier::BOLD)),
            Span::styled(format!(" {} ", pkt.flags),Style::default().fg(color)),
            Span::styled(
            format!("SEQ=0x{:08x} ACK=0x{:08x}",pkt.seq, pkt.ack_num),
            Style::default().fg(Color::White)),
            Span::styled(format!(" [{}B]", pkt.payload.len()),Style::default().fg(Color::DarkGray)),
            Span::styled(format!(" {}", pkt.payload_preview),Style::default().fg(Color::Gray)),
        ]);

            ListItem::new(line)
        }).collect();
    let list = List::new(items).block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(CLR_BORDER)).title(" Packets ").title_style(Style::default().fg(CLR_TITLE))).highlight_style(Style::default().bg(Color::DarkGray).add_modifier(Modifier::BOLD)).highlight_symbol("▶ ");
    frame.render_stateful_widget(list, area, &mut app.list_state.clone());
}

fn draw_status_log(frame: &mut Frame, app: &App, area: Rect) {
    // показываем последние N строк лога
    let visible_lines = area.height.saturating_sub(2) as usize;
    let start = app.status_log.len().saturating_sub(visible_lines);
    let log_text: String = app.status_log[start..].iter().map(|s| s.as_str()).collect::<Vec<_>>().join("\n");
    let log_widget = Paragraph::new(log_text).style(Style::default().fg(CLR_STATUS)).block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(CLR_BORDER)).title(" Log ")).wrap(Wrap { trim: false });
    frame.render_widget(log_widget, area);
}

fn draw_input(frame: &mut Frame, app: &App, area: Rect) {
    let input_text = format!("▏{}", app.input_buffer);
    let input_widget = Paragraph::new(input_text).style(Style::default().fg(CLR_INPUT)).block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(CLR_BORDER)).title(" Inject Command (Enter to send) ").title_style(Style::default().fg(CLR_TITLE)));
    frame.render_widget(input_widget, area);
    frame.set_cursor_position((area.x + 2 + app.input_buffer.len() as u16,area.y + 1));
}

fn draw_hotkeys(frame: &mut Frame, area: Rect) {
    let hotkeys = Line::from(vec![
        Span::styled(" ESC", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
        Span::raw(" Quit  "),
        Span::styled("F1", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
        Span::raw(" RST  "),
        Span::styled("F2", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        Span::raw(" Desync  "),
        Span::styled("↑↓", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::raw(" Scroll  "),
        Span::styled("Enter", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
        Span::raw(" Send  "),
        Span::styled("^C/^D", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
        Span::raw(" Exit"),
    ]);
    let bar = Paragraph::new(hotkeys).style(Style::default().bg(Color::DarkGray));
    frame.render_widget(bar, area);
}