use crate::app::App;
use crate::data::Data;
use crate::AppResult;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};

pub fn update(data: &Data, app: &mut App) -> AppResult<()> {
    if event::poll(std::time::Duration::from_millis(250))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                if let KeyCode::Esc = key.code {
                    app.quit();
                    return Ok(());
                } else if let KeyCode::Char(ch) = key.code {
                    if !ch.is_control() && ch.is_ascii() {
                        // if let Some(r) = state.pos.get(&(ch as u8)) {
                        //     render_char();
                        //     render_lines();
                        //     render_word_pair();
                        // } else if ch as u8 == 32 {
                        //     render_char();
                        //     render_lines();
                        //     render_word_pair();
                        // } else {
                        //     render_char();
                        //     render_lines();
                        //     render_word_pair();
                        // }
                    } else {
                    }
                }
            }
        }
    }
    Ok(())
}
