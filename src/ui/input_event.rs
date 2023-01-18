use std::{sync::mpsc::{Receiver, Sender, channel, RecvError}, time::Duration, thread};

use crossterm::event::{self, KeyCode};

pub enum InputEventType {
    Quit,
    Up,
    Down,
    Select,
    Tick,
}

fn map_input_to_event(input_code: &KeyCode) -> Option<InputEventType> {
    match input_code {
        KeyCode::Char('q') => Some(InputEventType::Quit),
        KeyCode::Up => Some(InputEventType::Up),
        KeyCode::Down => Some(InputEventType::Down),
        KeyCode::Char(' ') => Some(InputEventType::Select),
        _ => None
    }
}

pub struct InputEvent {
    rx: Receiver<InputEventType>,
    _tx: Sender<InputEventType>
}

impl InputEvent {
    pub fn new(tick_rate: Duration) -> InputEvent {
        let ( tx, rx ) = channel();

        let tx_event = tx.clone();

        thread::spawn(move || {
            loop {
                // poll for tick rate duration, if no event, sent tick event.
                if crossterm::event::poll(tick_rate).unwrap() {
                    if let event::Event::Key(key) = event::read().unwrap() {
                        if let Some(event) = map_input_to_event(&key.code) {
                            
                            match tx_event.send(event) {
                                Err(_) => break,
                                _ => continue,
                            }
                        }
                    }
                }
                match tx_event.send(InputEventType::Tick) {
                    Err(_) => break,
                    _ => {}
                }
            }
        });

        InputEvent { rx, _tx: tx }
    }

    pub fn next(&self) -> Result<InputEventType, RecvError> {
        self.rx.recv()
    }
}