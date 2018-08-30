#[macro_use]
extern crate log;
extern crate serde;
extern crate reqwest;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use std::env;
use std::collections::HashMap;
use std::{thread, time};

use serde_json::Error;

mod parse;
mod logger;

#[derive(Serialize, Debug)]
#[serde(untagged)]
enum TValue<'a> {
    String(&'a str),
    Int(&'a i32),
    Int64(&'a i64)
}


pub fn get_bot_token() -> String {
    return match env::var("TELEGRAM_BOT_TOKEN") {
        Ok(token) => token,
        Err(e) => panic!("Couldn't read TELEGRAM_BOT_TOKEN ({})", e),
    };
}

pub struct TBot {
    pub token: String,
    client: reqwest::Client
}

impl TBot {
    pub fn new(token: String) -> TBot {

        logger::init().expect("Cannot setup logger");
        TBot {
            token,
            client: reqwest::Client::new()
        }
    }

    fn api_req(&self, method: &str, req_body: HashMap<&str, TValue>) -> String {

        let url = format!(
            "https://api.telegram.org/bot{}/{}", self.token, method
        );
        let mut response  = self.client.post(
            url.as_str()
        )
            .json(&req_body)
            .send()
            .unwrap();

        response.text().unwrap()
    }

    fn get_updates(&self, offset: &i32) -> Result<parse::TResponse, Error> {
        let mut request_body = HashMap::new();
        request_body.insert("offset", TValue::Int(offset));

        let raw_response = self.api_req("getUpdates", request_body);
        let parsed_response = parse::parse_response(
            raw_response.as_str()
        );

//        info!("{:#?}", parsed_response);

        parsed_response
    }

    pub fn run(&self) {
        info!("TBot is up!");
        let mut offset: i32 = 0;
        loop {
            let parsed_response = self.get_updates(&offset).unwrap();
            let updates = if parsed_response.updates.is_some() {
                parsed_response.updates.unwrap()
            } else {
                // Continue main infinite loop
                continue;
            };
            for update in updates {
                let message = if update.message.is_some() {
                    update.message.unwrap()
                } else {
                    continue;
                };

                let text = if message.text.is_some() {
                    message.text.unwrap()
                } else {
                    continue;
                };
                info!("{:#?}", text);
                let chat = if message.chat.is_some() {
                    message.chat.unwrap()
                } else {
                    continue;
                };
                let chat_id = chat.id;
                match text.as_ref() {
                    "/say_hello" => {
                        let mut request_body = HashMap::new();
                        request_body.insert("chat_id", TValue::Int64(&chat_id));
                        request_body.insert("text", TValue::String("Hello!"));
                        self.api_req("sendMessage", request_body);
                    }
                    _ => {}
                };
                // Telegram API requires to make getUpdates request with offset,
                // To drop old events
                offset = update.update_id + 1;
            }
            thread::sleep(time::Duration::from_millis(100));
        }
    }
}