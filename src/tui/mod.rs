pub mod app;
pub mod widgets;

use crate::errors::*;
use crate::net::injector::Injector;
use crate::net::packet::ParsedPacket;
use app::App;
use crossterm::{event::{Event, KeyCode, KeyModifiers}, execute,terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}};
use futures::StreamExt;
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io::Stdout;
use tokio::sync::mpsc;

type Term = Terminal<CrosstermBackend<Stdout>>;

pub fn setup_terminal() -> Result<Term> {
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

pub fn restore_terminal(mut terminal: Term) -> Result<()> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}

pub async fn run_tui(injector: &Injector,mut packet_rx: mpsc::Receiver<ParsedPacket>) -> Result<()> {
    let mut terminal = setup_terminal()?;
    let mut app = App::new();

    app.status = format!("Hijacking {} → {}", injector.connection().src,injector.connection().dst);
    app.is_hijacked = true;

    let mut event_stream = crossterm::event::EventStream::new();

    loop {
        terminal.draw(|frame| {
            widgets::draw(frame, &app);
        })?;

        if !app.running {
            break;
        }

        tokio::select! {
            Some(pkt) = packet_rx.recv() => {
                app.add_packet(pkt);
            }

            Some(Ok(evt)) = event_stream.next() => {
                match evt {
                    Event::Key(key) => {
                        handle_key(&mut app, injector, key.code, key.modifiers).await?;
                    }
                    Event::Resize(_, _) => {

                    }
                    _ => {}
                }
            }
        }
    }

    restore_terminal(terminal)?;
    Ok(())
}

async fn handle_key(app: &mut App,injector: &Injector,key: KeyCode,modifiers: KeyModifiers) -> Result<()> {
    if modifiers.contains(KeyModifiers::CONTROL) {
        match key {
            KeyCode::Char('c') | KeyCode::Char('d') => {
                app.running = false;
                return Ok(());
            }
            _ => {}
        }
    }

    match key {
        KeyCode::Esc => {
            app.running = false;
        }

        KeyCode::Enter => {
            if !app.input_buffer.is_empty() {
                let data = format!("{}\n", app.input_buffer);
                app.add_status(&format!("Injecting: {:?}", app.input_buffer));

                if let Err(e) = injector.inject_data(data.as_bytes()).await {
                    app.add_status(&format!("Send error: {}", e));
                }
                app.input_buffer.clear();
            }
        }

        KeyCode::Backspace => {
            app.input_buffer.pop();
        }

        KeyCode::Char(c) => {
            app.input_buffer.push(c);
        }

        KeyCode::Up => {
            app.scroll_up();
        }

        KeyCode::Down => {
            app.scroll_down();
        }

        KeyCode::F(1) => {
            app.add_status("Sending RST...");
            injector.reset().await?;
            app.add_status("RST sent. Connection closed.");
        }

        KeyCode::F(2) => {
            app.add_status("Sending 1KB null desync...");
            injector.desync().await?;
            app.add_status("Desync complete.");
        }

        _ => {}
    }

    Ok(())
}