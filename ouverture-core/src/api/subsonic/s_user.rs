use super::*;
// User
pub async fn get_user(
    State(server): State<&Server>,
    request: Request<user::GetUser>,
) -> Result<Response> {
    Err(Error)
}
pub async fn get_users(
    State(server): State<&Server>,
    request: Request<user::GetUsers>,
) -> Result<Response> {
    Err(Error)
}
pub async fn create_user(
    State(server): State<&Server>,
    request: Request<user::CreateUser>,
) -> Result<Response> {
    Err(Error)
}
pub async fn update_user(
    State(server): State<&Server>,
    request: Request<user::UpdateUser>,
) -> Result<Response> {
    Err(Error)
}
pub async fn delete_user(
    State(server): State<&Server>,
    request: Request<user::DeleteUser>,
) -> Result<Response> {
    Err(Error)
}
pub async fn change_password(
    State(server): State<&Server>,
    request: Request<user::ChangePassword>,
) -> Result<Response> {
    Err(Error)
}
