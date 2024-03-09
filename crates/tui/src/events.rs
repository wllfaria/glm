use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

use crossterm::event::KeyEvent;
use crossterm::event::{self, Event as CrosstermEvent};

pub enum Event {
    Tick,
    Key(KeyEvent),
}

/// `EventHandler` spawns a new thread to poll for events.
///
/// Events are sent via message passing through a channel, and can only be
/// accessed through the `next()` method.
pub struct EventHandler {
    _tx: mpsc::Sender<Event>,
    rx: mpsc::Receiver<Event>,
}

impl EventHandler {
    /// Produces a new `EventHandler`, spawning a new thread to poll for events.
    ///
    /// Events are polled using `crossterm::event::poll`, and are passed as messages
    /// through channels
    ///
    /// # Arguments
    /// * tick_rate: the interval in milliseconds in between attempts to poll for new events.
    pub fn new(tick_rate: u64) -> Self {
        let tick_rate = Duration::from_millis(tick_rate);
        let (tx, rx) = mpsc::channel();
        {
            let sender = tx.clone();
            thread::spawn(move || {
                let mut last_tick = Instant::now();
                loop {
                    if event::poll(tick_rate).expect("failed to poll events") {
                        match event::read().expect("unable to read events") {
                            CrosstermEvent::Key(e) => sender.send(Event::Key(e)),
                            _ => unimplemented!(),
                        }
                        .expect("failed to send terminal event")
                    }

                    if last_tick.elapsed() >= tick_rate {
                        sender.send(Event::Tick).expect("unable to send tick event");
                        last_tick = Instant::now();
                    }
                }
            });
        }
        Self { _tx: tx, rx }
    }

    /// Returns the next channel on the channel of `Event`s by blocking the thread
    /// until a new `Event` is received
    pub fn next(&self) -> anyhow::Result<Event> {
        Ok(self.rx.recv()?)
    }
}
