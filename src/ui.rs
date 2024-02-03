use crate::que::Que;
use crate::App;
use ratatui::{style::Stylize, widgets::Paragraph, Frame};

pub fn render(app: &App, q: &Que, f: &mut Frame) {
    f.render_widget(
        Paragraph::new(format!("{app:?}\n{q:?}"))
            .yellow()
            .on_black(),
        f.size(),
    );
}
