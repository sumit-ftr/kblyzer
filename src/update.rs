use crate::Mapping;
use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEventKind};
use std::error::Error;

pub fn update(mapping: &mut Mapping) -> Result<(), Box<dyn Error>> {
    if event::poll(std::time::Duration::from_millis(250))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('Q') => {
                        mapping.quit();
                        return Ok(());
                    }
                    _ => {}
                }
            }
        }
    }
    Ok(())
}
