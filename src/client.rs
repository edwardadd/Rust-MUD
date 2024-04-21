use crate::events::Event;
use std::io::Read;
use std::net::TcpStream;
use std::sync::mpsc::Sender;

pub struct Client {
    id: u32,
    stream: TcpStream,
    buffer: [u8; 1024],
    offset: usize,
}

impl Client {
    pub fn new(id: u32, stream: TcpStream, sender: Sender<Event>) -> Self {
        Client {
            id,
            stream,
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

        // Process previous input with new input and send new commands to game

        // for i in offset..self.offset {
        //     if self.buffer[i] == b'\n' || self.buffer[i] == b'\0' {
        //         self.buffer[0] = b'\0';
        //         self.offset = 0;

        //         // complete command and process it
        //         let command = String::from_utf8_lossy(&self.buffer[0..i]);
        //         println!("{}", command);
        //     }
        // }
    }
}
