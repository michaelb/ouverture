use super::*;

// Lists
pub async fn get_album_list(
    State(server): State<&Server>,
    request: Request<lists::GetAlbumList>,
) -> Result<Response> {
    Err(Error)
}
pub async fn get_album_list2(
    State(server): State<&Server>,
    request: Request<lists::GetAlbumList2>,
) -> Result<Response> {
    Err(Error)
}
pub async fn get_random_songs(
    State(server): State<&Server>,
    request: Request<lists::GetRandomSongs>,
) -> Result<Response> {
    Err(Error)
}
pub async fn get_songs_by_genre(
    State(server): State<&Server>,
    request: Request<lists::GetSongsByGenre>,
) -> Result<Response> {
    Err(Error)
}
pub async fn get_now_playing(
    State(server): State<&Server>,
    request: Request<lists::GetNowPlaying>,
) -> Result<Response> {
    Err(Error)
}
pub async fn get_starred(
    State(server): State<&Server>,
    request: Request<lists::GetStarred>,
) -> Result<Response> {
    Err(Error)
}
pub async fn get_starred2(
    State(server): State<&Server>,
    request: Request<lists::GetStarred2>,
) -> Result<Response> {
    Err(Error)
}
