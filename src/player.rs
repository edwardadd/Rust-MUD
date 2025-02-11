
use rusqlite::{Connection, Result};
use bcrypt::{hash, verify, DEFAULT_COST};

pub struct Player {
    client_id: u32,
    id: u32,
    name: String,
    authenticated: bool,
}

impl Player {
    pub fn new(client_id: u32, id: u32, name: String) -> Self {
        Player {
            client_id,
            id,
            name,
            authenticated: false,
        }
    }

    pub fn init_db() -> Result<()> {
        let conn = Connection::open("players.db")?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS players (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL UNIQUE,
                password_hash TEXT NOT NULL
            )",
            [],
        )?;
        Ok(())
    }

    pub fn register(username: &str, password: &str) -> Result<bool> {
        let conn = Connection::open("players.db")?;
        let hashed = hash(password.as_bytes(), DEFAULT_COST).unwrap();
        
        match conn.execute(
            "INSERT INTO players (name, password_hash) VALUES (?1, ?2)",
            [username, &hashed],
        ) {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    pub fn authenticate(username: &str, password: &str) -> Result<bool> {
        let conn = Connection::open("players.db")?;
        let mut stmt = conn.prepare("SELECT password_hash FROM players WHERE name = ?")?;
        let mut rows = stmt.query([username])?;
        
        if let Some(row) = rows.next()? {
            let hash: String = row.get(0)?;
            Ok(verify(password.as_bytes(), &hash).unwrap_or(false))
        } else {
            Ok(false)
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
        assert_eq!(player.authenticated, false);
    }
}
