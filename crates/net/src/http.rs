pub fn get(url: &str) -> String {
    String::from(format!("GET example page content: {} ", url))
}

pub fn post(url: &str) -> String {
    String::from(format!("POST example page content: {} ", url))
}