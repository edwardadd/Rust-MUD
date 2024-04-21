use std::net::TcpStream;

pub enum Event {
    NewClient(TcpStream),
    NewCommand(String),
    Quit,
}
