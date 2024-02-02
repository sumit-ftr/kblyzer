use crate::Mapping;
use ratatui::{style::Stylize, widgets::Paragraph, Frame};

pub fn render(mapping: &Mapping, f: &mut Frame) {
    f.render_widget(
        Paragraph::new(format!("{mapping:?}")).yellow().on_black(),
        f.size(),
    );
}
