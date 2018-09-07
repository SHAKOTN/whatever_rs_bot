use serde_json::from_str;
use serde_json::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct TResponse {
    pub ok: bool,
    #[serde(rename = "result")]
    pub updates: Option<Vec<TUpdate>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TUpdate {
    pub message: Option<TMessage>,
    pub update_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TMessage {
    pub date: i32,
    pub message_id: i32,
    pub chat: Option<TChat>,
    pub from: Option<TFrom>,
    pub text: Option<String>,
    pub sticker: Option<TSticker>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TFrom {
    pub id: i32,
    pub first_name: String,
    pub is_bot: bool,
    pub last_name: Option<String>,
    pub username: Option<String>,
    pub language_code: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TChat {
    pub id: i64,
    pub title: Option<String>,
    #[serde(rename = "type")]
    pub chat_type: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TSticker {
    pub file_id: String,
    pub emoji: Option<String>,
    pub file_size: i32,
    pub set_name: Option<String>,
    pub height: i16,
    pub width: i16,
}

pub fn parse_response(t_response: &str) -> Result<TResponse, Error> {
    let parsed_response: TResponse = from_str(t_response)?;
    Ok(parsed_response)
}
