use crate::que::Que;
use crate::App;
use ratatui::{
    layout::{Constraint, Layout, Rect},
    prelude::Direction,
    style::Stylize,
    text::Line,
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use std::rc::Rc;

pub fn render(app: &App, q: &Que, frame: &mut Frame) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(25),
            Constraint::Percentage(50),
            Constraint::Percentage(25),
        ])
        .split(frame.size());

    let layout_t = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Percentage(10),
            Constraint::Percentage(80),
            Constraint::Percentage(10),
        ])
        .split(layout[0]);

    let layout_t_inner = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(33),
            Constraint::Percentage(33),
            Constraint::Percentage(33),
        ])
        .split(layout_t[1]);

    let layout_kb = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Percentage(35),
            Constraint::Percentage(30),
            Constraint::Percentage(35),
        ])
        .split(layout[1]);

    let layout_d = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Percentage(10),
            Constraint::Percentage(80),
            Constraint::Percentage(10),
        ])
        .split(layout[2]);

    let layout_d_inner = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(33),
            Constraint::Percentage(33),
            Constraint::Percentage(33),
        ])
        .split(layout_d[1]);

    render_layout(frame, &layout_kb, &app, &q);
    render_lines(frame, &layout_t_inner, &layout_d_inner, &q, 0);
    render_lines(frame, &layout_t_inner, &layout_d_inner, &q, 1);
    render_lines(frame, &layout_t_inner, &layout_d_inner, &q, 2);
}

fn render_layout(frame: &mut Frame, l: &Rc<[Rect]>, app: &App, q: &Que) {
    frame.render_widget(
        Paragraph::new(format!("{:?}", app.layout_left))
            .white()
            .on_black()
            .block(Block::new().borders(Borders::ALL)),
        l[0],
    );
    frame.render_widget(
        Paragraph::new(format!("word pair not implemented"))
            .white()
            .on_black()
            .block(Block::new().borders(Borders::ALL)),
        l[1],
    );
    frame.render_widget(
        Paragraph::new(format!("{:?}", app.layout_right))
            .white()
            .on_black()
            .block(Block::new().borders(Borders::ALL)),
        l[2],
    );
}

fn render_lines(frame: &mut Frame, lt: &Rc<[Rect]>, ld: &Rc<[Rect]>, q: &Que, lno: usize) {
    frame.render_widget(
        Paragraph::new(&q.tline[lno] as &str).white().on_black(),
        lt[lno],
    );
    frame.render_widget(
        Paragraph::new(&q.dline[lno] as &str).white().on_black(),
        ld[lno],
    );
}
