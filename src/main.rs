use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::sync::{Arc, Mutex};

struct Client {
    stream: TcpStream,
    buffer: [u8; 1024],
    offset: usize,
}

impl Client {
    fn process(&mut self) {
        loop {
            let offset = self.offset;

            match self.stream.read(&mut self.buffer) {
                Ok(size) => self.offset += size,
                Err(e) => println!("Failed to read from stream, {}", e),
            }

            for i in offset..self.offset {
                if self.buffer[i] == b'\n' {
                    // complete command and process it
                    let command = String::from_utf8_lossy(&self.buffer[0..i]);
                    println!("{}", command);
                }
            }
        }
    }
}

fn main() {
    let clients: Arc<Mutex<Vec<Client>>> = Arc::new(Mutex::new(Vec::new()));

    let _clients = clients.clone();
    let _ = thread::spawn(move || loop {
        let mut citer = _clients.lock().unwrap();
        for client in citer.iter_mut() {
            client.process();
        }
    });

    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => clients.lock().unwrap().push(Client {
                stream: stream,
                buffer: [0; 1024],
                offset: 0,
            }),
            Err(e) => println!("couldn't get client: {e:?}"),
        }
    }
}
