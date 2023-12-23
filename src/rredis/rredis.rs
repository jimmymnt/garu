use redis::{Client, Connection, RedisResult};

pub struct RedisManager {
    pub connection: Connection,
}

impl RedisManager {
    // Create a new instance of RedisManager with a connection
    pub fn new(con_str: String) -> RedisResult<Self> {
        let client = Client::open(con_str).unwrap();
        let connection = client.get_connection().unwrap();
        Ok(Self { connection })
    }
}
