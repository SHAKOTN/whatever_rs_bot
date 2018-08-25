use serde_json::Error;
use serde_json::from_str;

#[derive (Debug, Serialize, Deserialize)]
pub struct TResponse {
    pub ok: bool,
    pub result: Option<Vec<TResult>>,
}

#[derive (Debug, Serialize, Deserialize)]
pub struct TResult {
    pub message: TMessage,
    pub update_id: i32,
}


#[derive (Debug, Serialize, Deserialize)]
pub struct TMessage {
    pub date: i32,
    pub message_id: i8,
    pub chat: TChat,
    pub from: TFrom,
    pub text: Option<String>,
    pub sticker: Option<TSticker>,
}

#[derive (Debug, Serialize, Deserialize)]
pub struct TFrom {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub language_code: String,
    pub is_bot: bool,
}

#[derive (Debug, Serialize, Deserialize)]
pub struct TChat {
    pub id: i32,
    pub title: Option<String>,
    #[serde(rename="type")]
    pub chat_type: String,
}

#[derive (Debug, Serialize, Deserialize)]
pub struct TSticker {
    pub emoji: String,
    pub file_id: String,
    pub file_size: i32,
    pub set_name: String,
    pub height: i16,
    pub width: i16
}

pub fn parse_response(t_response: &str) -> Result<TResponse, Error> {
    let parsed_response: TResponse = from_str(t_response)?;
    Ok(parsed_response)
}