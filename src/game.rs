use crate::events::Event;
use std::sync::mpsc;

pub struct Game {
    events: mpsc::Receiver<Event>,
}

impl Game {
    pub fn new(events: mpsc::Receiver<Event>) -> Self {
        Game { events }
    }

    pub fn run(&mut self) {
        loop {
            match self.events.recv() {
                Ok(Event::Quit) => break,
                Ok(Event::NewClient(client)) => {
                    println!("New client connected!");
                }
                Ok(Event::NewCommand(command)) => {
                    println!("New command: {}", command);
                }
                Err(mpsc::RecvError) => break,
            }
        }
    }
}
