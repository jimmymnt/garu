use std::{error::Error, io::BufReader};

use redis::{Commands, Connection};
use serde::Deserialize;

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

#[derive(Debug, Deserialize)]
pub struct Member {
    member: String,
    score: u32,
}

#[derive(Debug, Deserialize)]
pub struct BoardInfo {
    pub info: Vec<Member>,
}

pub trait BoardManager {
    fn board_info(&self) -> BoardInfo;
}

pub struct BoardOLPManager {
    pub key: String,
}

impl BoardManager for BoardOLPManager {
    // Return to add multiple members to the leaderboard.
    fn board_info(&self) -> BoardInfo {
        self.get_data_from_json_file().unwrap()
    }
}

impl BoardOLPManager {
    pub fn set_board(&self, con: &mut Connection) {
        let board_info = self.board_info();
        // Use Sorted Set in Redis to store leaderboard score
        let mut data = vec![];
        for item in board_info.info {
            data.push((item.score, item.member));
        }

        let _: () = con.zadd_multiple(&self.key, &data).unwrap();
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

    pub fn get_data_from_json_file(&self) -> Result<BoardInfo, Box<dyn Error>> {
        let file = std::fs::File::open("../../assets/data.json")?;
        let reader = BufReader::new(file);

        let board = serde_json::from_reader(reader)?;
        Ok(board)
    }
}
