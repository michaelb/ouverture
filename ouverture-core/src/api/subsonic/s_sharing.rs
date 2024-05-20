use super::*;

// Sharing
pub async fn get_shares(
    State(server): State<&Server>,
    request: Request<sharing::GetShares>,
) -> Result<Response> {
    Err(Error)
}
pub async fn create_share(
    State(server): State<&Server>,
    request: Request<sharing::CreateShare>,
) -> Result<Response> {
    Err(Error)
}
pub async fn update_share(
    State(server): State<&Server>,
    request: Request<sharing::UpdateShare>,
) -> Result<Response> {
    Err(Error)
}
pub async fn delete_share(
    State(server): State<&Server>,
    request: Request<sharing::DeleteShare>,
) -> Result<Response> {
    Err(Error)
}
