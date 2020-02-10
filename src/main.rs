mod fs;
mod util;

use serde_json::{Result, Value};
use reqwest::Client;

fn main() {
    let home_dir = match std::env::var("HOME") {
        Ok(val) => val,
        Err(e) => panic!(e)
    };
    let file_path = [home_dir, String::from("/.jwtconfig.json")].concat();
    println!("{}", file_path);
/*
    let stuff = match util::get_map("things") {
        Ok(val) => val,
        Err(e) => panic!(e)
    };
*/

//    println!("{}", stuff.into_iter().fold(String::from(""), |acc, (other_key, value)| {
//        let val2 = match value.as_str() {
//            Some(val) => val,
//            None => ""
//        };
//        [acc, "&".to_string(), other_key, "=".to_string(), val2.to_string()].concat()
//    }));
    let query_params = match util::get_query_params(&file_path, "thing") {
        Ok(val) => val,
        Err(e) => panic!(e)
    };

    println!("{}", query_params);
    let res = Client::new()
        .post("url")
        .body(&query_params)
        .header("Content-Type", "www/url-form-encoded")
        .send()
        .await?
        .text()
        .await?;
    println!("{}", res);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}