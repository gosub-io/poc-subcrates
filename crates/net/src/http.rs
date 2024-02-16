use gosub_shared::shared;

pub fn delete(url: &str) -> String {
    String::from(format!("DELETE: {} ", url))
}

pub fn get(url: &str) -> String {
    shared();
    String::from(format!("GET example page content: {} ", url))
}

pub fn post(url: &str) -> String {
    String::from(format!("POST example page content: {} ", url))
}