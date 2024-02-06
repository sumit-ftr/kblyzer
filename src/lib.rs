// App struct contains all the data about the state of the word queue and lines that are rendered
pub mod app;

// Data struct contains all the data related to the wordlist and layout (that is going to be tested)
pub mod data;

pub mod ui;

pub mod event_handler;

// AppResult type
pub type AppResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;
