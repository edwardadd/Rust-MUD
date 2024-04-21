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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_player() {
        let player = Player::new(1, 1, "Test".to_string());
        assert_eq!(player.client_id, 1);
        assert_eq!(player.id, 1);
        assert_eq!(player.name, "Test");
    }
}
