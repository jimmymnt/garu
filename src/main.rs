use std::{env, process};

use board::board::{BoardOLPManager, Config};
use redis::RedisResult;
use rredis::rredis::RedisManager;

pub mod board;
pub mod rredis;

fn main() -> RedisResult<()> {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        println!("Error: {}", err);
        process::exit(1);
    });

    // DESC: Descending
    // ASC: Ascending
    // Create a RedisManager instance and obtain the connection
    let con_str = "<redis_url>";
    let mut manager = RedisManager::new(con_str.to_owned()).unwrap();

    let board_olp_manager = BoardOLPManager {
        key: "olp_v1".to_owned(),
    };

    // board_olp_manager.set_board(&mut manager.connection);
    board_olp_manager.get_board(config.query, &mut manager.connection);

    Ok(())
}
