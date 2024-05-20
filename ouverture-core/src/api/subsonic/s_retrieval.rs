use super::*;
// Retrieval
pub async fn stream(
    State(server): State<&Server>,
    request: Request<retrieval::Stream>,
) -> Result<Bytes> {
    Err(Error)
}
pub async fn download(
    State(server): State<&Server>,
    request: Request<retrieval::Download>,
) -> Result<Bytes> {
    Err(Error)
}
pub async fn hls(
    State(server): State<&Server>,
    request: Request<retrieval::Hls>,
) -> Result<Response> {
    Err(Error)
}
pub async fn get_captions(
    State(server): State<&Server>,
    request: Request<retrieval::GetCaptions>,
) -> Result<Response> {
    Err(Error)
}
pub async fn get_cover_art(
    State(server): State<&Server>,
    request: Request<retrieval::GetCoverArt>,
) -> Result<Bytes> {
    Err(Error)
}
pub async fn get_lyrics(
    State(server): State<&Server>,
    request: Request<retrieval::GetLyrics>,
) -> Result<Response> {
    Err(Error)
}
pub async fn get_avatar(
    State(server): State<&Server>,
    request: Request<retrieval::GetAvatar>,
) -> Result<Bytes> {
    Err(Error)
}
