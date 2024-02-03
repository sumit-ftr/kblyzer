use ltest::que::Que;
use ltest::App;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = App::new(std::env::args()).unwrap_or_else(|err| {
        eprintln!("Error: {err}");
        std::process::exit(1);
    });
    let mut q = Que::new(&app, 32);

    // startup: Enable raw mode for the terminal, giving us fine control over user input
    crossterm::terminal::enable_raw_mode()?;
    crossterm::execute!(std::io::stderr(), crossterm::terminal::EnterAlternateScreen)?;

    // Initialize the terminal backend using crossterm
    let mut terminal =
        ratatui::Terminal::new(ratatui::prelude::CrosstermBackend::new(std::io::stderr()))?;

    while !q.should_quit() {
        ltest::update::update(&app, &mut q)?;
        terminal.draw(|f| ltest::ui::render(&app, &q, f))?;
    }

    // shutdown down: reset terminal back to original state
    crossterm::execute!(std::io::stderr(), crossterm::terminal::LeaveAlternateScreen)?;
    crossterm::terminal::disable_raw_mode()?;
    Ok(())
}
