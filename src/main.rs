extern crate reqwest;

extern crate serde_json;
extern crate serde;

#[macro_use]
extern crate serde_derive;

//use serde_json::Error;
use serde_json::Value;

use std::{thread, time};
use std::collections::HashMap;
use std::env;

#[derive (Debug, Serialize, Deserialize)]
struct TResponse {
    ok: bool,
    result: Vec<Value>,
}

fn parse_response(t_response: &str) -> TResponse {
    let parsed_response: TResponse = serde_json::from_str(t_response).unwrap();
    return parsed_response;
}

fn get_bot_token() -> String {
    return match env::var("TELEGRAM_BOT_TOKEN") {
        Ok(token) => token,
        Err(e) => panic!("Couldn't read TELEGRAM_BOT_TOKEN ({})", e),
    };
}

fn main() {
    let token: String = get_bot_token();
    loop {
        let client = reqwest::Client::new();
        let mut map = HashMap::new();
        let url = format!("https://api.telegram.org/bot{}/getUpdates", token);

        map.insert("offset", 267956369);

        let mut response_test  = client.post(
            url.as_str()
        )
//            .json(&map)
            .send()
            .unwrap();

        let text = response_test.text().unwrap();

        let parsed_response: TResponse = parse_response(text.as_str());
        println!("{:#?}", parsed_response);

        thread::sleep(time::Duration::from_millis(1000));
    }
}