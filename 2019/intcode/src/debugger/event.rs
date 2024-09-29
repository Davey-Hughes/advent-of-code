use std::time::Duration;

use crossterm::event::{Event as CrosstermEvent, KeyEvent, MouseEvent};
use futures::{FutureExt, StreamExt};
use tokio::sync::mpsc;

use crate::debugger::DebuggerResult;

/// Terminal events.
#[derive(Clone, Copy, Debug)]
pub enum Event {
    /// Terminal tick.
    Tick,
    /// Key press.
    Key(KeyEvent),
    /// Mouse click/scroll.
    Mouse(MouseEvent),
    /// Terminal resize.
    Resize(u16, u16),
}

/// Terminal event handler.
#[allow(dead_code, clippy::module_name_repetitions)]
#[derive(Debug)]
pub struct EventHandler {
    /// Event sender channel.
    sender: mpsc::UnboundedSender<Event>,
    /// Event receiver channel.
    receiver: mpsc::UnboundedReceiver<Event>,
    /// Event handler thread.
    handler: tokio::task::JoinHandle<()>,
}

impl EventHandler {
    /// Constructs a new instance of [`EventHandler`].
    pub fn new(tick_rate: u64) -> Self {
        let tick_rate = Duration::from_millis(tick_rate);
        let (sender, receiver) = mpsc::unbounded_channel();
        let sender_cloned = sender.clone();
        let handler = tokio::spawn(async move {
            let mut reader = crossterm::event::EventStream::new();
            let mut tick = tokio::time::interval(tick_rate);
            loop {
                let tick_delay = tick.tick();
                let crossterm_event = reader.next().fuse();
                tokio::select! {
                  _ = sender_cloned.closed() => {
                    break;
                  }
                  _ = tick_delay => {
                    sender_cloned.send(Event::Tick).unwrap();
                  }
                  Some(Ok(evt)) = crossterm_event => {
                    match evt {
                      CrosstermEvent::Key(key) => {
                        if key.kind == crossterm::event::KeyEventKind::Press {
                          sender_cloned.send(Event::Key(key)).unwrap();
                        }
                      },
                      CrosstermEvent::Mouse(mouse) => {
                        sender_cloned.send(Event::Mouse(mouse)).unwrap();
                      },
                      CrosstermEvent::Resize(x, y) => {
                        sender_cloned.send(Event::Resize(x, y)).unwrap();
                      },
                      CrosstermEvent::FocusLost => {},
                      CrosstermEvent::FocusGained => {},
                      CrosstermEvent::Paste(_) => {},
                    }
                  }
                };
            }
        });
        Self {
            sender,
            receiver,
            handler,
        }
    }

    /// Receive the next event from the handler thread.
    ///
    /// This function will always block the current thread if
    /// there is no data available and it's possible for more data to be sent.
    pub async fn next(&mut self) -> DebuggerResult<Event> {
        Ok(self.receiver.recv().await.ok_or_else(|| {
            Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "This is an IO error",
            ))
        })?)
    }
}
