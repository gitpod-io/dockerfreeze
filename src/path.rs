use std::env::var;

fn get_path() -> String {
    var("PATH").unwrap()
}

pub fn path() -> Vec<String> {
    get_path().split(":").map(|item| item.to_string()).collect()
}
