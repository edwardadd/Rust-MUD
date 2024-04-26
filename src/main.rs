use std::net::TcpListener;
use std::sync::mpsc::Sender;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

mod client;
use crate::client::Client;

mod events;
use crate::events::Event;

mod game;
use crate::game::Game;

mod player;
mod room;
// use crate::room::Room;

mod commands;

static SERVER_ADDRESS: &str = "127.0.0.1:8080";

fn init_process_thread(clients: Arc<Mutex<Vec<Client>>>) -> thread::JoinHandle<()> {
    thread::spawn(move || loop {
        // Park the thread if there are no clients
        // if clients.lock().unwrap().len() == 0 {
        //     // TODO: Check if there is a race condition here due to thread unparking when clients are added
        //     println!("Parking process_thread - no clients");
        //     thread::park();
        //     continue;
        // }

        // println!("init_process_thread - REPEAT!");

        let mut clients = clients.lock().unwrap();

        for client in clients.iter_mut() {
            // println!("init_process_thread - client process!");
            client.process();
        }

        // thread::sleep(Duration::from_millis(100));
        thread::yield_now();
    })
}

fn listen_for_connections(
    clients: Arc<Mutex<Vec<Client>>>,
    process_thread: thread::JoinHandle<()>,
    sender: Sender<Event>,
) {
    thread::spawn(move || loop {
        let listener = TcpListener::bind(SERVER_ADDRESS).unwrap();

        let mut id = 0; // Note: This is not a player but just  a client id

        println!("Server listening on {SERVER_ADDRESS}");

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    stream
                        .set_nonblocking(true)
                        .expect("Cannot set stream unblocking");
                    println!("New client connected!");
                    id += 1;
                    clients
                        .lock()
                        .unwrap()
                        .push(Client::new(id, stream, sender.clone()));

                    // Unpark the thread if it isn't already
                    // process_thread.thread().unpark();
                    // println!("Unparking process_thread - clients available");
                }
                Err(e) => println!("couldn't get client: {e:?}"),
            }
        }
    });
}

fn main() {
    let clients: Arc<Mutex<Vec<Client>>> = Arc::new(Mutex::new(Vec::new()));

    let (sender, receiver) = mpsc::channel();

    let process_thread = init_process_thread(clients.clone());
    listen_for_connections(clients.clone(), process_thread, sender);

    let mut game = Game::new(clients, receiver);
    game.run();
}
