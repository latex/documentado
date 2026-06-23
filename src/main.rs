use anyhow::Result;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io;

mod app;
mod docs;
mod ui;

#[tokio::main]
async fn main() -> Result<()> {
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic| {
        let _ = restore_terminal();
        original_hook(panic);
    }));

    let query = std::env::args().nth(1).unwrap_or_default();

    crossterm::terminal::enable_raw_mode()?;
    let mut stderr = io::stderr();
    crossterm::execute!(stderr, crossterm::terminal::EnterAlternateScreen, crossterm::event::EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    let mut app = app::App::new(query)?;
    let res = app.run(&mut terminal).await;

    restore_terminal()?;
    res
}

fn restore_terminal() -> Result<()> {
    crossterm::terminal::disable_raw_mode()?;
    crossterm::execute!(io::stdout(), crossterm::terminal::LeaveAlternateScreen, crossterm::event::DisableMouseCapture)?;
    Ok(())
}
