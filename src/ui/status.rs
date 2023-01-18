use std::time::Duration;

use humantime::format_duration;
use tui::{widgets::{Paragraph, Wrap}, text::{Spans, Span}, style::{Style, Color}, layout::Alignment};

use crate::file_helper::size;

fn info<'a>(field_name: String, value: String) -> Spans<'a> {
    Spans::from(vec![
        Span::raw(field_name),
        Span::raw(": "),
        Span::styled(value, Style::default().fg(Color::Green))
    ])
}

pub fn status_block<'a>(total_size: Option<u128>, time_init: Option<Duration>, free_space: u128) -> Paragraph<'a> {
    let total_size_value = match total_size {
        None => "..".to_owned(),
        Some(byte) => size(byte)
    };

    let duration_value = match time_init {
        None => "..".to_owned(),
        Some(dur) => format_duration(dur).to_string(),
    };

    let free_space_value = size(free_space);

    let info_block = vec![
        info("Total size".to_owned(), total_size_value),
        info("Time".to_owned(), duration_value),
        info("Free space".to_owned(), free_space_value)
    ];
    Paragraph::new(info_block)
        .style(Style::default().bg(Color::Black))
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true })
}