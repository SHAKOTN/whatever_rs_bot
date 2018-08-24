use serde_json::Error;
use serde_json::from_str;

#[derive (Debug, Serialize, Deserialize)]
pub struct TResponse {
    ok: bool,
    result: Vec<TResult>,
}

#[derive (Debug, Serialize, Deserialize)]
struct TResult {
    message: TMessage,
    update_id: i32,
}


#[derive (Debug, Serialize, Deserialize)]
struct TMessage {
    date: i32,
    message_id: i8,
    text: Option<String>,
}


pub fn parse_response(t_response: &str) -> Result<TResponse, Error> {
    let parsed_response: TResponse = from_str(t_response)?;
    Ok(parsed_response)
}