use net::http::get;

pub fn config_set(key: &str, value: &str) {
    println!("Setting config key: {} to value: {}", key, value);
}

pub fn config_get(key: &str) -> String {
    println!("{}", get(format!("http://example.com/{}", key).as_str()));

    format!("KEY[{}]", key)
}