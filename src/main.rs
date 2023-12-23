#[allow(dead_code, unused_variables, unreachable_code, unused_must_use)]
use std::{env, process};

use redis::{Client, Commands, Connection, RedisResult};

#[derive(Debug)]
pub struct ScoreInfo<'a> {
    member: &'a str,
    score: u8,
}

pub struct RedisManager {
    connection: Connection,
}

#[derive(Debug)]
pub struct Config {
    query: String,
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
    key: String,
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
        let _: () = con.zadd_multiple("olp_v1", &board_info).unwrap();
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

impl RedisManager {
    // Create a new instance of RedisManager with a connection
    pub fn new(con_str: String) -> RedisResult<Self> {
        let client = Client::open(con_str).unwrap();
        let connection = client.get_connection().unwrap();
        Ok(Self { connection })
    }
}

fn main() -> RedisResult<()> {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        println!("Error: {}", err);
        process::exit(1);
    });

    // DESC: Descending
    // ASC: Ascending
    // Create a RedisManager instance and obtain the connection
    let con_str = "rediss://default:b9dc5e0cc6fd4062aec3bdc287e4d697@apn1-bursting-glowworm-35229.upstash.io:35229";
    let mut manager = RedisManager::new(con_str.to_owned()).unwrap();

    let board_olp_manager = BoardOLPManager {
        key: "olp_v1".to_owned(),
    };

    // board_olp_manager.set_board(&mut manager.connection);
    board_olp_manager.get_board(config.query, &mut manager.connection);

    Ok(())
}
