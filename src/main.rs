use kblyzer::app::App;
use kblyzer::data::Data;
use kblyzer::AppResult;

fn main() -> AppResult<()> {
    let data = Data::new(std::env::args()).unwrap_or_else(|err| {
        eprintln!("Error: {err}");
        std::process::exit(1);
    });
    // this is taking up a bit of execution time (because it is getting reinitialized) but this is the only way I made this work
    let mut app: App = Default::default();

    // startup: Enable raw mode for the terminal, giving us fine control over user input
    crossterm::terminal::enable_raw_mode()?;
    crossterm::execute!(std::io::stderr(), crossterm::terminal::EnterAlternateScreen)?;

    // Initialize the terminal backend using crossterm
    let mut terminal =
        ratatui::Terminal::new(ratatui::prelude::CrosstermBackend::new(std::io::stderr()))?;

    terminal.draw(|frame| {
        app = App::new(&data, frame.size().width);
        kblyzer::ui::render(&data, &app, frame);
    })?;
    while !app.should_quit() {
        kblyzer::event_handler::update(&data, &mut app)?;
    }

    // shutdown down: reset terminal back to original state
    crossterm::execute!(std::io::stderr(), crossterm::terminal::LeaveAlternateScreen)?;
    crossterm::terminal::disable_raw_mode()?;
    Ok(())
}
