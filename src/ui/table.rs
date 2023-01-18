use tui::{widgets::{Table, Cell, Row, Block, Borders}, style::{Style, Color, Modifier}, layout::Constraint};

use crate::file_helper::size;

use super::{DirData, DirStatus};

const ROW_BOTTOM_MARGIN: u16 = 1u16;

fn map_status_to_str(status: &DirStatus) -> String {
    match status {
        DirStatus::Loading => "LOADING".to_owned(),
        DirStatus::Ready => "READY".to_owned(),
        DirStatus::Deleting => "DELETING".to_owned(),
        DirStatus::Deleted => "DELETED".to_owned(),
        DirStatus::Error => "ERROR".to_owned(),
    }
}

pub fn table<'a>(items: &Vec<DirData>) -> Table<'a> {
    let rows: Vec<Row> = items.iter().map(|item| {
        let cells = vec![
            Cell::from(item.path.clone()),
            match item.size {
                Some(byte) => Cell::from(size(byte)),
                None => Cell::from(".."),
            },
            Cell::from(map_status_to_str(&item.status))
        ];
        Row::new(cells).bottom_margin(ROW_BOTTOM_MARGIN)
    }).collect();

    Table::new(rows)
        .header(Row::new(vec!["Path", "Size", "Status"])
            .style(Style::default().fg(Color::Cyan))
            .bottom_margin(ROW_BOTTOM_MARGIN)
        )
        .block(Block::default().borders(Borders::ALL))
        .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
        .widths(&[
            Constraint::Percentage(70),
            Constraint::Percentage(20),
            Constraint::Percentage(10),
        ])
}