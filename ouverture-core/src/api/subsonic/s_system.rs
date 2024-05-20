use subsonic_types::{common::Version, response::{ResponseBody, License}};

use super::*;
// System
pub async fn ping(
    State(server): State<&Server>,
    request: Request<system::Ping>,
) -> Result<Response> {
    info!("ping");
    Ok(Response::ok(
        Version::V1_16_1,
        ResponseBody::Empty
    ))
}
pub async fn get_license(
    State(server): State<&Server>,
    request: Request<system::GetLicense>,
) -> Result<Response> {

    Ok(Response::ok(
        Version::V1_16_1,
        ResponseBody::License(License {valid:true, ..Default::default()})
    ))
}
