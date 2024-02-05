use crate::que::Que;
use crate::App;
use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEventKind};
use ratatui::prelude::Rect;
use std::{collections::HashMap, error::Error};

pub fn update(app: &App, q: &mut Que) -> Result<(), Box<dyn Error>> {
    if event::poll(std::time::Duration::from_millis(250))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                if let KeyCode::Esc = key.code {
                    q.quit();
                    return Ok(());
                } else if let KeyCode::Char(ch) = key.code {
                }
            }
        }
    }
    Ok(())
}

struct State {
    r: HashMap<u8, Rect>,
    pre_ch: Option<u8>,
    cur_ch: u8,
}
