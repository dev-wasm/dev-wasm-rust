use std::env;
use std::collections::HashMap;

fn querystring(str: String) -> HashMap<String, String> {
    let mut result = HashMap::new();
    let parts = str.split("&");
    for part in parts {
        let query: Vec<&str> = part.split("=").collect();
        if query.len() == 0 {
            continue;
        }
        if query.len() == 1 {
            result.insert(query[0].to_string(), "".to_string());
            continue;
        }
        result.insert(query[0].to_string(), query[1].to_string());
    }
    return result
}

fn main() {
    println!("Content-type: text/html");
    println!();

    let mut name = "unknown";
    let q = querystring(env::var("QUERY_STRING").unwrap());
    if q.contains_key("name") {
        name = &q["name"];
    }
    println!("<body><h3>Hello {}!</h3></body>", name);
}