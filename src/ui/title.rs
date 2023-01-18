use tui::{widgets::Paragraph, layout::Alignment, style::{Style, Color}};

const TITLE: &'static str = r"
                       __                           __   .__.__  .__   
_______ __ __  _______/  |_            ____ ______ |  | _|__|  | |  |  
\_  __ \  |  \/  ___/\   __\  ______  /    \\____ \|  |/ /  |  | |  |  
|  | \/  |  /\___ \  |  |   /_____/ |   |  \  |_> >    <|  |  |_|  |__
|__|  |____//____  > |__|           |___|  /   __/|__|_ \__|____/____/
                 \/                      \/|__|        \/             
";

pub fn title<'a>() -> Paragraph<'a> {
    Paragraph::new(TITLE).style(Style::default().fg(Color::White).bg(Color::Black))
    .alignment(Alignment::Center)
}