use crate::utils::StringExt;
use std::error;

use std::io;

use ratatui::{backend::CrosstermBackend, Terminal};

use crate::debugger::{
    event::{Event, EventHandler},
    handler::handle_key_events,
    tui::Tui,
};

pub mod event;
pub mod handler;
pub mod tui;
pub mod ui;

/// Application result type.
#[allow(clippy::module_name_repetitions)]
pub type DebuggerResult<T> = std::result::Result<T, Box<dyn error::Error + Sync + Send>>;

/// Application.
#[derive(Debug)]
pub struct Debugger {
    /// Is the application running?
    pub running: bool,

    pub scrolling: bool,
    pub scroll_offset: (u16, u16),

    pub interpreter: intcode::Interpreter,
    pub text: String,
}

impl Debugger {
    pub async fn from_file(file: &str, input: Vec<i64>) -> DebuggerResult<Self> {
        let interpreter = intcode::Interpreter::from_file(file, input).await?;
        let text = interpreter.to_string().expand_tabs(8);
        Ok(Self {
            running: true,
            scrolling: false,
            scroll_offset: (0, 0),
            interpreter,
            text,
        })
    }

    /// Handles the tick event of the terminal.
    pub const fn tick(&self) {
        let _ = self;
    }

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn scroll_down(&mut self) {
        self.scroll_offset.0 += 1;
    }

    pub fn scroll_up(&mut self) {
        if self.scroll_offset.0 > 0 {
            self.scroll_offset.0 -= 1;
        }
    }

    pub async fn next(&mut self) {
        self.update_text();

        let pc_index = self.text.find(">").unwrap();
        self.scroll_offset.0 = self.text[0..pc_index]
            .chars()
            .filter(|x| *x == '\n')
            .count()
            .saturating_sub(7)
            .try_into()
            .expect("Could not convert line number to offset for pc marker");

        self.interpreter
            .exec_one()
            .await
            .expect("Could not execute instruction");
    }

    fn update_text(&mut self) {
        self.text = self.interpreter.to_string().expand_tabs(8);
    }
}

pub async fn start(file: &str, input: Vec<i64>) -> DebuggerResult<()> {
    // Create an application.
    let mut debugger = Debugger::from_file(file, input).await?;

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stdout());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    // Start the main loop.
    while debugger.running {
        // Render the user interface.
        tui.draw(&debugger)?;
        // Handle events.
        match tui.events.next().await? {
            Event::Tick => debugger.tick(),
            Event::Key(key_event) => handle_key_events(key_event, &mut debugger).await?,
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
