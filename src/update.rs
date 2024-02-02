use crate::App;
use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEventKind};
use std::error::Error;

pub fn update(app: &mut App) -> Result<(), Box<dyn Error>> {
    if event::poll(std::time::Duration::from_millis(250))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('Q') => {
                        app.quit();
                        return Ok(());
                    }
                    _ => {}
                }
            }
        }
    }
    Ok(())
}
