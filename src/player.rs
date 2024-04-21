pub struct Player {
    client_id: u32,
    id: u32,
    name: String,
}

impl Player {
    pub fn new(client_id: u32, id: u32, name: String) -> Self {
        Player {
            client_id,
            id,
            name,
        }
    }
}
