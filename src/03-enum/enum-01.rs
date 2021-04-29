use crate::Request::*;

#[allow(dead_code)]
enum Request {
    Set { key: String, value: String },
    Get { key: String },
    Delete { key: String }
}

fn main() {
    let req = Set {
        key: String::from("főzőlap"),
        value: String::from("Sencor")
    };

    match req {
        Set { key, value } => handle_set(key, value),
        Get { key } => handle_get(key),
        Delete { key } => handle_delete(key)
    }
}

fn handle_set(key: String, value: String) {
    println!("{} = {}", key, value);
}

fn handle_get(_key: String) { todo!(); }
fn handle_delete(_key: String) { todo!(); }
