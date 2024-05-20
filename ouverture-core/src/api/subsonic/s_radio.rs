use super::*;
// Radio
pub async fn get_internet_radio_stations(
    State(server): State<&Server>,
    request: Request<radio::GetInternetRadioStations>,
) -> Result<Response> {
    Err(Error)
}
pub async fn create_internet_radio_station(
    State(server): State<&Server>,
    request: Request<radio::CreateInternetRadioStation>,
) -> Result<Response> {
    Err(Error)
}
pub async fn update_internet_radio_station(
    State(server): State<&Server>,
    request: Request<radio::UpdateInternetRadioStation>,
) -> Result<Response> {
    Err(Error)
}
pub async fn delete_internet_radio_station(
    State(server): State<&Server>,
    request: Request<radio::DeleteInternetRadioStation>,
) -> Result<Response> {
    Err(Error)
}
