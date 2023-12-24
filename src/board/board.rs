use std::io::Error;

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
    fn board_info(&self) -> Vec<(f32, String)>;
}

pub struct BoardOLPManager {
    pub key: String,
}

impl BoardManager for BoardOLPManager {
    // Return to add multiple members to the leaderboard.
    fn board_info(&self) -> Vec<(f32, String)> {
        self.get_data_from_json_file().unwrap()
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

    pub fn get_data_from_json_file(&self) -> Result<Vec<(f32, String)>, Error> {
        let mut members = Vec::new();
        let file =
            std::fs::File::open("../../assets/data.json").expect("file should open read only");

        let json: serde_json::Value =
            serde_json::from_reader(file).expect("file should be proper JSON");

        for member in json.as_array().unwrap() {
            // println!(
            //     "{:?} - {:?}",
            //     member.get("name").unwrap().to_string(),
            //     member.get("score").unwrap().as_f64().unwrap(),
            // );
            members.push((
                member.get("score").unwrap().as_f64().unwrap() as f32,
                member.get("name").unwrap().to_string(),
            ));
        }

        Ok(members)
    }
}
