use super::*;
// Playlists
pub async fn get_playlists(
    State(server): State<&Server>,
    request: Request<playlists::GetPlaylists>,
) -> Result<Response> {
    Err(Error)
}
pub async fn get_playlist(
    State(server): State<&Server>,
    request: Request<playlists::GetPlaylist>,
) -> Result<Response> {
    Err(Error)
}
pub async fn create_playlist(
    State(server): State<&Server>,
    request: Request<playlists::CreatePlaylist>,
) -> Result<Response> {
    Err(Error)
}
pub async fn update_playlist(
    State(server): State<&Server>,
    request: Request<playlists::UpdatePlaylist>,
) -> Result<Response> {
    Err(Error)
}
pub async fn delete_playlist(
    State(server): State<&Server>,
    request: Request<playlists::DeletePlaylist>,
) -> Result<Response> {
    Err(Error)
}
