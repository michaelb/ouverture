use super::*;
// Bookmarks
pub async fn get_bookmarks(
    State(server): State<&Server>,
    request: Request<bookmark::GetBookmarks>,
) -> Result<Response> {
    Err(Error)
}
pub async fn create_bookmark(
    State(server): State<&Server>,
    request: Request<bookmark::CreateBookmark>,
) -> Result<Response> {
    Err(Error)
}
pub async fn delete_bookmark(
    State(server): State<&Server>,
    request: Request<bookmark::DeleteBookmark>,
) -> Result<Response> {
    Err(Error)
}
pub async fn get_play_queue(
    State(server): State<&Server>,
    request: Request<bookmark::GetPlayQueue>,
) -> Result<Response> {
    Err(Error)
}
pub async fn save_play_queue(
    State(server): State<&Server>,
    request: Request<bookmark::SavePlayQueue>,
) -> Result<Response> {
    Err(Error)
}
