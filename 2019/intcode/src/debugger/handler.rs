use crate::debugger::{Debugger, DebuggerResult};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Handles the key events and updates the state of [`App`].
pub async fn handle_key_events(key_event: KeyEvent, debugger: &mut Debugger) -> DebuggerResult<()> {
    match key_event.code {
        // Exit application on `ESC` or `q`
        KeyCode::Esc | KeyCode::Char('q') => {
            debugger.quit();
        }
        // Exit application on `Ctrl-C`
        KeyCode::Char('c' | 'C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                debugger.quit();
            }
        }
        KeyCode::Char('n') => {
            debugger.next().await;
        }
        KeyCode::Down => {
            debugger.scroll_down();
        }
        KeyCode::Up => {
            debugger.scroll_up();
        }
        // Other handlers you could add here.
        _ => {}
    }
    Ok(())
}
