use crate::que::Que;
use crate::App;
use ratatui::{
    layout::{Constraint, Layout, Rect},
    prelude::Direction,
    style::{Color, Stylize},
    widgets::{Block, Borders, Paragraph, Wrap},
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
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Min(1),
            Constraint::Min(1),
            Constraint::Length(3),
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
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Min(1),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(layout_d[1]);

    frame.render_widget(
        Paragraph::new("")
            .fg(Color::Rgb(255, 211, 0))
            .bg(Color::Rgb(24, 24, 24))
            .block(Block::new().borders(Borders::ALL)),
        layout[0],
    );
    frame.render_widget(
        Paragraph::new("")
            .fg(Color::Rgb(255, 211, 0))
            .bg(Color::Rgb(24, 24, 24))
            .block(Block::new().borders(Borders::ALL)),
        layout[2],
    );

    render_layout(frame, layout_kb[0], &app.layout_left);
    render_layout(frame, layout_kb[2], &app.layout_right);
    render_word_pair(frame, layout_kb[1], &app);
    render_lines(frame, &layout_t_inner, &layout_d_inner, &q, 0);
    render_lines(frame, &layout_t_inner, &layout_d_inner, &q, 1);
    render_lines(frame, &layout_t_inner, &layout_d_inner, &q, 2);
}

fn render_layout(frame: &mut Frame, r: Rect, a: &[[u8; 5]; 3]) {
    frame.render_widget(
        Paragraph::new("")
            .black()
            .on_blue()
            .block(Block::new().borders(Borders::ALL)),
        r,
    );

    let row = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Min(4),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(4),
        ])
        .split(r);

    let row0 = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Min(12),
            Constraint::Length(5),
            Constraint::Length(5),
            Constraint::Length(5),
            Constraint::Length(5),
            Constraint::Length(5),
            Constraint::Min(12),
        ])
        .split(row[1]);

    let row1 = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Min(12),
            Constraint::Length(5),
            Constraint::Length(5),
            Constraint::Length(5),
            Constraint::Length(5),
            Constraint::Length(5),
            Constraint::Min(12),
        ])
        .split(row[2]);

    let row2 = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Min(12),
            Constraint::Length(5),
            Constraint::Length(5),
            Constraint::Length(5),
            Constraint::Length(5),
            Constraint::Length(5),
            Constraint::Min(12),
        ])
        .split(row[3]);

    for i in 0..5 {
        render_char(frame, row0[i + 1], a[0][i]);
    }
    for i in 0..5 {
        render_char(frame, row1[i + 1], a[1][i]);
    }
    for i in 0..5 {
        render_char(frame, row2[i + 1], a[2][i]);
    }
}

fn render_char(frame: &mut Frame, r: Rect, ch: u8) {
    frame.render_widget(
        Paragraph::new(format!(" {}", ch as char))
            .fg(Color::Rgb(64, 64, 64))
            .bg(Color::Rgb(196, 196, 196))
            // .wrap(Wrap { trim: true })
            .block(Block::new().borders(Borders::ALL)),
        r,
    );
}

fn render_word_pair(frame: &mut Frame, l: Rect, app: &App) {
    frame.render_widget(
        Paragraph::new(format!("word pair not implemented"))
            .white()
            .on_black()
            .block(Block::new().borders(Borders::ALL)),
        l,
    );
}

fn render_lines(frame: &mut Frame, lt: &Rc<[Rect]>, ld: &Rc<[Rect]>, q: &Que, lno: usize) {
    frame.render_widget(
        Paragraph::new(&q.tline[lno] as &str)
            .fg(Color::Rgb(255, 211, 0))
            .bg(Color::Rgb(24, 24, 24)),
        lt[lno + 1],
    );
    frame.render_widget(
        Paragraph::new(&q.dline[lno] as &str)
            .fg(Color::Rgb(255, 211, 0))
            .bg(Color::Rgb(24, 24, 24)),
        ld[lno + 1],
    );
}
