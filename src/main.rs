use config::{config_get, config_set};
use net::http::{get, post};

fn main() {
    config_set("key", "value");
    let value = config_get("key");

    println!("{}", get("http://example.com"));
    println!("{}", post("http://example.com"));

    println!("value: {}", value);
}
