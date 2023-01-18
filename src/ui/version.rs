use tui::{widgets::{Paragraph, Wrap}, style::{Style, Color}, layout::Alignment};

pub fn version_block<'a>() -> Paragraph<'a> {
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    Paragraph::new(VERSION)
        .style(Style::default().fg(Color::LightBlue))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true })
}