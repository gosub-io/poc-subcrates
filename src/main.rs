use gosub_config::config::{config_get, config_set};
use gosub_net::http::{get, post, delete};

fn main() {
    config_set("key", "value");
    let value = config_get("key");

    println!("{}", get("http://example.com"));
    println!("{}", post("http://example.com"));
    println!("{}", delete("http://example.com"));

    println!("value: {}", value);
}
