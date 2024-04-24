use crate::commands::Command;
use crate::events::Event;
use std::io::Read;
use std::net::TcpStream;
use std::sync::mpsc::Sender;

pub struct Client {
    id: u32,
    stream: TcpStream,
    sender: Sender<Event>,
    buffer: [u8; 1024],
    offset: usize,
}

impl Client {
    pub fn new(id: u32, stream: TcpStream, sender: Sender<Event>) -> Self {
        Client {
            id,
            stream,
            sender,
            buffer: [0; 1024],
            offset: 0,
        }
    }

    pub fn process(&mut self) {
        // Read in input from client
        let mut buffer: [u8; 1024] = [0; 1024];
        let mut bytes_read: usize = 0;

        match self.stream.read(&mut buffer) {
            Ok(size) => bytes_read = size,
            Err(e) => println!("Failed to read from stream, {}", e),
        }

        if bytes_read > 0 {
            println!(
                "Client: {}, Buffer: {}, bytes: {}",
                self.id,
                String::from_utf8_lossy(&buffer),
                bytes_read
            );
        }

        let mut last_offset = 0;

        for i in 0..bytes_read {
            if buffer[i] == b'\n' || buffer[i] == b'\0' {
                self.buffer[0] = b'\0';
                self.offset = 0;

                // complete command and process it
                let command_string = String::from_utf8_lossy(&buffer[last_offset..i]);

                last_offset = i;

                // println!("{}", command_string);
                let command = self.parse_input(&command_string);
                match self.sender.send(command) {
                    Ok(_) => (),
                    Err(e) => println!("Failed to send command: {}", e),
                }
            }
        }
    }

    fn parse_input(&mut self, input: &str) -> Event {
        let tokens: Vec<_> = input.split_ascii_whitespace().collect();

        match tokens[0] {
            "look" => Event::NewCommand(Command::Look { who: self.id }),
            "move" => Event::NewCommand(Command::Move {
                who: self.id,
                x: tokens[1].parse().unwrap(),
                y: tokens[2].parse().unwrap(),
            }),
            "quit" => Event::Quit,
            _ => Event::NewCommand(Command::Say {
                who: self.id,
                what: input.to_string(),
            }),
        }
    }
}
