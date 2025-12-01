use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct ServerMessage {
    #[serde(rename = "type")]
    pub msg_type: String,
    pub command: Option<String>,
    pub text: Option<String>,
    pub state: Option<String>,
    pub session_id: Option<String>,
}
