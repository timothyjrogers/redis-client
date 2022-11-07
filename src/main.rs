use redis_client::{Client, connect, ping};

fn main() {
    let mut client = match connect("127.0.0.1", 6379) {
        Ok(c) => c,
        Err(e) => panic!("{}", e)
    };
    ping(&mut client);
}
