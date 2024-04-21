pub struct Room {
    pub id: u32,
    pub name: String,
    pub players: Vec<u32>,
}

impl Room {
    pub fn new(id: u32, name: String) -> Self {
        Room {
            id,
            name,
            players: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_room() {
        let room = Room::new(1, "Test".to_string());
        assert_eq!(room.id, 1);
        assert_eq!(room.name, "Test");
        assert_eq!(room.players.len(), 0);
    }
}
