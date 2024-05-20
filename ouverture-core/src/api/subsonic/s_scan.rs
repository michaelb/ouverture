use super::*;
// Scan
pub async fn get_scan_status(
    State(server): State<&Server>,
    request: Request<scan::GetScanStatus>,
) -> Result<Response> {
    Err(Error)
}
pub async fn start_scan(
    State(server): State<&Server>,
    request: Request<scan::StartScan>,
) -> Result<Response> {
    Err(Error)
}

// Search
pub async fn search(
    State(server): State<&Server>,
    request: Request<search::Search>,
) -> Result<Response> {
    Err(Error)
}
pub async fn search2(
    State(server): State<&Server>,
    request: Request<search::Search2>,
) -> Result<Response> {
    Err(Error)
}
pub async fn search3(
    State(server): State<&Server>,
    request: Request<search::Search3>,
) -> Result<Response> {
    Err(Error)
}
