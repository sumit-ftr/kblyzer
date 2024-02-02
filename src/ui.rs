use crate::App;
use ratatui::{style::Stylize, widgets::Paragraph, Frame};

pub fn render(app: &App, f: &mut Frame) {
    f.render_widget(
        Paragraph::new(format!("{app:?}")).yellow().on_black(),
        f.size(),
    );
}
