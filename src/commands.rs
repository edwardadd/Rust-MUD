#[derive(Debug)]
pub enum Command {
    Say { who: u32, what: String },
    Look { who: u32 },
    Move { who: u32, x: i32, y: i32 },
    Quit { who: u32 },
    Login { who: u32, username: String, password: String },
}
