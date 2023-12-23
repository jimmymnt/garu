use redis::{Commands, Connection};

#[derive(Debug)]
pub struct Config {
    pub query: String,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();
        let query = match args.next() {
            Some(query) => query,
            None => return Err("Did not get a query string."),
        };

        Ok(Self { query })
    }
}

pub trait BoardManager {
    fn board_info(&self) -> Vec<(f32, &str)>;
}

pub struct BoardOLPManager {
    pub key: String,
}

impl BoardManager for BoardOLPManager {
    // Return to add multiple members to the leaderboard.
    fn board_info(&self) -> Vec<(f32, &str)> {
        vec![
            (90.0, "Jimmy"),
            (100.0, "Robert"),
            (60.0, "Lee"),
            (20.0, "Andrew"),
            (50.0, "Ander"),
        ]
    }
}

impl BoardOLPManager {
    pub fn set_board(&self, con: &mut Connection) {
        let board_info = self.board_info();
        // Use Sorted Set in Redis to store leaderboard score
        let _: () = con.zadd_multiple(&self.key, &board_info).unwrap();
    }

    pub fn get_board(&self, query: String, con: &mut Connection) {
        let resp: Vec<(String, u32)>;
        if query.to_lowercase() == "asc" {
            resp = con.zrange_withscores(&self.key, 0, -1).unwrap();
        } else {
            resp = con.zrevrange_withscores(&self.key, 0, -1).unwrap();
        }
        dbg!(resp);
    }
}
