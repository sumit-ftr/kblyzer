use crate::que::Que;
use crate::App;
use ratatui::{
    layout::{Constraint, Layout, Rect},
    prelude::Direction,
    style::Stylize,
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use std::rc::Rc;

pub fn render(app: &App, q: &Que, frame: &mut Frame) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(50),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
        ])
        .split(frame.size());

    let layout_top = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Percentage(35),
            Constraint::Percentage(30),
            Constraint::Percentage(35),
        ])
        .split(layout[0]);

    render_layout(frame, &layout_top, &app, &q);
    render_lines(frame, &layout, &q);
}

fn render_layout(frame: &mut Frame, layout_top: &Rc<[Rect]>, app: &App, q: &Que) {
    frame.render_widget(
        Paragraph::new(format!("{:?}", app.layout_left))
            .white()
            .on_black()
            .block(Block::new().borders(Borders::ALL)),
        layout_top[0],
    );
    frame.render_widget(
        Paragraph::new(format!("word pair not implemented"))
            .white()
            .on_black()
            .block(Block::new().borders(Borders::ALL)),
        layout_top[1],
    );
    frame.render_widget(
        Paragraph::new(format!("{:?}", app.layout_right))
            .white()
            .on_black()
            .block(Block::new().borders(Borders::ALL)),
        layout_top[2],
    );
}

fn render_lines(frame: &mut Frame, layout: &Rc<[Rect]>, q: &Que) {
    frame.render_widget(
        Paragraph::new(format!("{q:?}"))
            .white()
            .on_black()
            .block(Block::new().borders(Borders::ALL)),
        layout[1],
    );
    frame.render_widget(
        Paragraph::new(format!("{q:?}"))
            .white()
            .on_black()
            .block(Block::new().borders(Borders::ALL)),
        layout[2],
    );
}
