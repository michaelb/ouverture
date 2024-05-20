use super::*;

// Chat
pub async fn get_chat_messages(
    State(server): State<&Server>,
    request: Request<chat::GetChatMessages>,
) -> Result<Response> {
    Err(Error)
}
pub async fn add_chat_message(
    State(server): State<&Server>,
    request: Request<chat::AddChatMessage>,
) -> Result<Response> {
    Err(Error)
}

// Jukebox
pub async fn jukebox_control(
    State(server): State<&Server>,
    request: Request<jukebox::JukeboxControl>,
) -> Result<Response> {
    Err(Error)
}
