use tui::{widgets::Paragraph, style::{Style, Color}};

const GUIDELINE: &'static str = r"Select with CURSORS
Delete with SPACE
Quit with 'q'";

pub fn guideline<'a>() -> Paragraph<'a> {
    Paragraph::new(GUIDELINE).style(Style::default().bg(Color::Yellow).fg(Color::Black))
}