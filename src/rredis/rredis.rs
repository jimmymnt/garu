use redis::{Client, Connection, RedisResult};

pub struct RedisManager {
    pub connection: Connection,
}

impl RedisManager {
    // Create a new instance of RedisManager with a connection
    pub fn new() -> RedisResult<Self> {
        let client = Client::open(Self::get_redis_url()).unwrap();
        let connection = client.get_connection().unwrap();
        Ok(Self { connection })
    }

    fn get_redis_url<'a>() -> &'a str {
        "redis://default:Redis@2023@127.0.0.1/"
    }
}
