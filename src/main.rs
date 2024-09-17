use std::io;

use ratatui::{crossterm::event::{self, KeyCode, KeyEventKind}, style::Stylize, widgets::Paragraph, DefaultTerminal};

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    terminal.clear()?;
    let app_result = run(terminal);
    ratatui::restore();
    app_result
}

fn run(mut term: DefaultTerminal) -> io::Result<()> {
    loop {
        term.draw(|frame| {
            let greet = Paragraph::new("Hello Ratatui! (press 'q' to quit)")
                .white()
                .on_blue();
            frame.render_widget(greet, frame.area());
        })?;

        if let event::Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return Ok(());
            }
        }
    }
}
