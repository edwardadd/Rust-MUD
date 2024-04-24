use crate::commands::Command;
use std::net::TcpStream;

pub enum Event {
    NewClient(TcpStream),
    NewCommand(Command),
    Quit,
}
