use super::*;
// Annotation
pub async fn star(
    State(server): State<&Server>,
    request: Request<annotation::Star>,
) -> Result<Response> {
    Err(Error)
}
pub async fn unstar(
    State(server): State<&Server>,
    request: Request<annotation::Unstar>,
) -> Result<Response> {
    Err(Error)
}
pub async fn set_rating(
    State(server): State<&Server>,
    request: Request<annotation::SetRating>,
) -> Result<Response> {
    Err(Error)
}
pub async fn scrobble(
    State(server): State<&Server>,
    request: Request<annotation::Scrobble>,
) -> Result<Response> {
    Err(Error)
}
