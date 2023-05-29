use redis::{Client, Commands, Connection};

pub fn redis_connect() -> Connection {
    let client = Client::open("redis://127.0.0.1:6379/")
        .unwrap()
        .get_connection()
        .unwrap();
    client
}

pub fn set_value(key: String, value: String) {
    let mut connection = redis_connect();
    connection
        .set_ex::<String, String, String>(key, value, 60 * 60 * 24)
        .unwrap();
}
pub fn get_value(key: String) -> String {
    let mut connection = redis_connect();
    let res = connection.get::<String, String>(key).unwrap();
    res
}
