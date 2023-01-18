use tui::{widgets::Paragraph, layout::Alignment, style::{Style, Color}};

pub fn table_placeholder<'a>() -> Paragraph<'a> {
    Paragraph::new("..Loading..").style(Style::default().fg(Color::White).bg(Color::Black))
    .alignment(Alignment::Center)
}