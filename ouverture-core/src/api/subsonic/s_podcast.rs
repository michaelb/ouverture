use super::*;
// Podcasts
pub async fn get_podcasts(
    State(server): State<&Server>,
    request: Request<podcast::GetPodcasts>,
) -> Result<Response> {
    Err(Error)
}
pub async fn get_newest_podcasts(
    State(server): State<&Server>,
    request: Request<podcast::GetNewestPodcasts>,
) -> Result<Response> {
    Err(Error)
}
pub async fn refresh_podcasts(
    State(server): State<&Server>,
    request: Request<podcast::RefreshPodcasts>,
) -> Result<Response> {
    Err(Error)
}
pub async fn create_podcast_channel(
    State(server): State<&Server>,
    request: Request<podcast::CreatePodcastChannel>,
) -> Result<Response> {
    Err(Error)
}
pub async fn delete_podcast_channel(
    State(server): State<&Server>,
    request: Request<podcast::DeletePodcastChannel>,
) -> Result<Response> {
    Err(Error)
}
pub async fn delete_podcast_episode(
    State(server): State<&Server>,
    request: Request<podcast::DeletePodcastEpisode>,
) -> Result<Response> {
    Err(Error)
}
pub async fn download_podcast_episode(
    State(server): State<&Server>,
    request: Request<podcast::DownloadPodcastEpisode>,
) -> Result<Response> {
    Err(Error)
}
