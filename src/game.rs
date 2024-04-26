use crate::{client::Client, commands::Command, events::Event};
use std::sync::{mpsc, Arc, Mutex};

pub struct Game {
    clients: Arc<Mutex<Vec<Client>>>,
    events: mpsc::Receiver<Event>,
}

impl Game {
    pub fn new(clients: Arc<Mutex<Vec<Client>>>, events: mpsc::Receiver<Event>) -> Self {
        Game { clients, events }
    }

    pub fn run(&mut self) {
        loop {
            match self.events.recv() {
                Ok(Event::Quit) => break,
                Ok(Event::NewClient(client)) => {
                    println!("New client connected!");
                }
                Ok(Event::NewCommand(command)) => {
                    println!("New command: {:?}", command);
                    self.process_command(command);
                }
                Err(mpsc::RecvError) => break,
            }
        }
    }

    fn process_command(&mut self, command: Command) {
        match command {
            Command::Say { who, what } => self.broad_cast(who, what),
            Command::Look { who } => self.broad_cast(who, "looking!".to_string()),
            Command::Move { who, x, y } => self.broad_cast(who, "moving!".to_string()),
            Command::Quit { who } => self.broad_cast(who, "quiting!".to_string()),
        }
    }

    pub fn broad_cast(&mut self, from: u32, message: String) {
        let mut message = message;
        message.push('\n');

        // Send the message to each clients TcpStream
        let mut clients = self.clients.lock().unwrap();
        let iter = clients.iter_mut();
        for client in iter {
            if client.id == from {
                continue;
            }

            client.send(&message);
        }
    }

    pub fn send(&mut self, from: u32, to: u32, message: String) {
        let mut clients = self.clients.lock().unwrap();
        // let fromClient = clients
        //     .iter()
        //     .find(|&client| client.id == from)
        //     .unwrap()
        //     .clone();
        let to_client = clients.iter_mut().find(|client| client.id == to).unwrap();
        to_client.send(&message);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_game() {
        let clients = Arc::new(Mutex::new(Vec::new()));
        let events = mpsc::channel();
        let game = Game::new(clients, events.1);
    }
}
