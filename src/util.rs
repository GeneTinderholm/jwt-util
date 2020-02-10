//use std::iter::Map;
use serde_json::{Result, Value};
use serde_json::Map;
use serde_json::map::Iter;
use std::fs::read_to_string;


pub fn get_map(key: &str, json_file: &str) -> Result<Map<String, Value>> {
//    let test_str = "{\"things\": \"stuff\", \"others\": \"chonk\"}";
    let test: Value = serde_json::from_str(json_file).unwrap();
    println!("{}", test["things"]);
//    Ok(test.as_object().unwrap().clone())
    for (json_key, val) in test.as_object().unwrap().into_iter() {
        if key == json_key {
            return Ok(val.as_object().unwrap().clone())
        }
    }
    panic!("things");
}

pub fn get_query_params(path: &str, key: &str) -> Result<String> {
    let json = match read_to_string(path) {
        Ok(val) => val,
        Err(e) => panic!(e)
    };

    let map = get_map(key, &json)?;

    let qp = &map.into_iter().fold(String::from(""), |acc, (key, val)| {
        let value = match val.as_str() {
            Some(val) => val,
            None => panic!("JSON incorrect")
        };

        [acc.to_string(), "&".to_string(), key, "=".to_string(), value.to_string()].concat()
    })[1..];
    Ok(String::from(qp))
}


