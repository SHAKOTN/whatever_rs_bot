#[macro_use]
extern crate log;
extern crate reqwest;
extern crate serde_json;
extern crate serde;

#[macro_use]
extern crate serde_derive;

use std::{thread, time};
use std::collections::HashMap;
use std::env;

mod parse;
mod logger;

fn get_bot_token() -> String {
    return match env::var("TELEGRAM_BOT_TOKEN") {
        Ok(token) => token,
        Err(e) => panic!("Couldn't read TELEGRAM_BOT_TOKEN ({})", e),
    };
}


fn main() {
    logger::init().expect("Cannot setup logger");

    let token: String = get_bot_token();

    let mut offset: i32 = 0;
    loop {
        let client = reqwest::Client::new();
        let mut request_body = HashMap::new();
        let url = format!("https://api.telegram.org/bot{}/getUpdates", token);

        request_body.insert("offset", offset);

        let mut response_test  = client.post(
            url.as_str()
        )
            .json(&request_body)
            .send()
            .unwrap();

        let text = response_test.text().unwrap();

        let parsed_response = parse::parse_response(text.as_str())
            .unwrap();
        for result in parsed_response.result.unwrap() {
            info!("{} {:#?}", parsed_response.ok, result);
            // Telegram API requires to make getUpdates request with offset,
            // To drop old events
            offset = result.update_id + 1;
        }

        thread::sleep(time::Duration::from_millis(100));
    }
}