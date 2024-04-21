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

    pub fn broad_cast(&mut self, from: u32, message: String) {
        // Send the message to each clients TcpStream
    }

    pub fn send(&mut self, from: u32, to: u32, message: String) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_game() {
        let events = mpsc::channel();
        let game = Game::new(events.1);
    }
}
