use parse;
use serde_json::Error;
use reqwest;
use std::collections::HashMap;
use TValue;

pub trait AbsTBot {
    fn new(token: String) -> Self;
    fn get_updates(&self, offset: &i32) -> Result<parse::TResponse, Error>;
    fn client(&self) -> &reqwest::Client;
    fn token(&self) -> &str;

    fn api_req(&self, method: &str, req_body: HashMap<&str, TValue>) -> String {

        let url = format!(
            "https://api.telegram.org/bot{}/{}", self.token().to_string(), method
        );
        let mut response  = self.client().post(
            url.as_str()
        )
            .json(&req_body)
            .send()
            .unwrap();

        response.text().unwrap()
    }
    fn run(&self);
}