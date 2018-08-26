#[macro_use]
extern crate log;
extern crate serde;
extern crate reqwest;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use std::env;
use std::collections::HashMap;

use serde_json::Error;

mod parse;
mod logger;

pub struct TBot {
    pub token: String
}

pub fn get_bot_token() -> String {
    return match env::var("TELEGRAM_BOT_TOKEN") {
        Ok(token) => token,
        Err(e) => panic!("Couldn't read TELEGRAM_BOT_TOKEN ({})", e),
    };
}
impl TBot {
    pub fn new(token: String) -> TBot {
        logger::init().expect("Cannot setup logger");
        TBot {
            token
        }
    }

    pub fn get_updates(&self, offset: &i32) -> Result<parse::TResponse, Error> {
        let client = reqwest::Client::new();
        let mut request_body = HashMap::new();
        request_body.insert("offset", offset);

        let url = format!("https://api.telegram.org/bot{}/getUpdates", self.token);
        let mut response  = client.post(
            url.as_str()
        )
            .json(&request_body)
            .send()
            .unwrap();

        let text = response.text().unwrap();

        let parsed_response = parse::parse_response(text.as_str());

        info!("{:#?}", parsed_response);

        parsed_response
    }
}