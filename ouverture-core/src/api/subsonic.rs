// subsonic API compartemented as decribed in their ref: http://www.subsonic.org/pages/api.jsp
mod auth;
// mod browsing;
// mod album_songlist;
// mod searching;
// mod playlists;
// mod media_retrieval;
// mod media_annotation;
// mod sharing;
// mod podcast;
// mod jukebox;
// mod internet_radio;
// mod chat;
// mod user_management;
// mod bookmark;
// mod scanning;
use crate::server::Server;
use async_trait::async_trait;
use axum::routing::{get, post};
use axum::{response::IntoResponse, Router};
use bytes::Bytes;
use hyper::StatusCode;
use std::sync::Arc;
use subsonic_types::request::*;
use subsonic_types::response::Response;
pub struct Subsonic {}

#[derive(Debug)]
pub struct Error;

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        axum::response::Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .unwrap()
    }
}

impl Subsonic {
    pub fn route() -> Router<&'static Server> {
        macro_rules! make_router {
            ($($req:path => $handler:ident),*) => {
                Router::new()
                $(
                    .route($req, get($handler))
                    .route(&format!("{}.view", $req), get($handler))
                )*
            }
        }
        make_router! {
            // Annotation
            // annotation::Star::PATH => star,
            // annotation::Unstar::PATH => unstar,
            // annotation::SetRating::PATH => set_rating,
            // annotation::Scrobble::PATH => scrobble,
            // // Bookmarks
            // bookmark::GetBookmarks::PATH => get_bookmarks,
            // bookmark::CreateBookmark::PATH => create_bookmark,
            // bookmark::DeleteBookmark::PATH => delete_bookmark,
            // bookmark::GetPlayQueue::PATH => get_play_queue,
            // bookmark::SavePlayQueue::PATH => save_play_queue,
            // // Browsing
            // browsing::GetMusicFolders::PATH => get_music_folders,
            // browsing::GetIndexes::PATH => get_indexes,
            // browsing::GetMusicDirectory::PATH => get_music_directory,
            // browsing::GetGenres::PATH => get_genres,
            // browsing::GetArtists::PATH => get_artists,
            // browsing::GetArtist::PATH => get_artist,
            // browsing::GetAlbum::PATH => get_album,
            // browsing::GetSong::PATH => get_song,
            // browsing::GetVideos::PATH => get_videos,
            // browsing::GetVideoInfo::PATH => get_video_info,
            // browsing::GetArtistInfo::PATH => get_artist_info,
            // browsing::GetArtistInfo2::PATH => get_artist_info2,
            // browsing::GetAlbumInfo::PATH => get_album_info,
            // browsing::GetAlbumInfo2::PATH => get_album_info2,
            // browsing::GetSimilarSongs::PATH => get_similar_songs,
            // browsing::GetSimilarSongs2::PATH => get_similar_songs2,
            // browsing::GetTopSongs::PATH => get_top_songs,
            // // Chat
            // chat::GetChatMessages::PATH => get_chat_messages,
            // chat::AddChatMessage::PATH => add_chat_message,
            // // Jukebox
            // jukebox::JukeboxControl::PATH => jukebox_control,
            // // Lists
            // lists::GetAlbumList::PATH => get_album_list,
            // lists::GetAlbumList2::PATH => get_album_list2,
            // lists::GetRandomSongs::PATH => get_random_songs,
            // lists::GetSongsByGenre::PATH => get_songs_by_genre,
            // lists::GetNowPlaying::PATH => get_now_playing,
            // lists::GetStarred::PATH => get_starred,
            // lists::GetStarred2::PATH => get_starred2,
            // // Playlists
            // playlists::GetPlaylists::PATH => get_playlists,
            // playlists::GetPlaylist::PATH => get_playlist,
            // playlists::CreatePlaylist::PATH => create_playlist,
            // playlists::UpdatePlaylist::PATH => update_playlist,
            // playlists::DeletePlaylist::PATH => delete_playlist,
            // // Podcasts
            // podcast::GetPodcasts::PATH => get_podcasts,
            // podcast::GetNewestPodcasts::PATH => get_newest_podcasts,
            // podcast::RefreshPodcasts::PATH => refresh_podcasts,
            // podcast::CreatePodcastChannel::PATH => create_podcast_channel,
            // podcast::DeletePodcastChannel::PATH => delete_podcast_channel,
            // podcast::DeletePodcastEpisode::PATH => delete_podcast_episode,
            // podcast::DownloadPodcastEpisode::PATH => download_podcast_episode,
            // // Radio
            // radio::GetInternetRadioStations::PATH => get_internet_radio_stations,
            // radio::CreateInternetRadioStation::PATH => create_internet_radio_station,
            // radio::UpdateInternetRadioStation::PATH => update_internet_radio_station,
            // radio::DeleteInternetRadioStation::PATH => delete_internet_radio_station,
            // // Retrieval
            // retrieval::Stream::PATH => stream,
            // retrieval::Download::PATH => download,
            // retrieval::Hls::PATH => hls,
            // retrieval::GetCaptions::PATH => get_captions,
            // retrieval::GetCoverArt::PATH => get_cover_art,
            // retrieval::GetLyrics::PATH => get_lyrics,
            // retrieval::GetAvatar::PATH => get_avatar,
            // // Scan
            // scan::GetScanStatus::PATH => get_scan_status,
            // scan::StartScan::PATH => start_scan,
            // Search
            // search::Search::PATH => search,
            // search::Search2::PATH => search2,
            // search::Search3::PATH => search3,
            // // Sharing
            // sharing::GetShares::PATH => get_shares,
            // sharing::CreateShare::PATH => create_share,
            // sharing::UpdateShare::PATH => update_share,
            // sharing::DeleteShare::PATH => delete_share,
            // // User
            // user::GetUser::PATH => get_user,
            // user::GetUsers::PATH => get_users,
            // user::CreateUser::PATH => create_user,
            // user::UpdateUser::PATH => update_user,
            // user::DeleteUser::PATH => delete_user,
            // user::ChangePassword::PATH => change_password
            // System
            // system::GetLicense::PATH => get_license,
            system::Ping::PATH => ping
        }
    }
}

#[async_trait]
#[allow(unused_variables)]
pub trait SubsonicService: Send + Sync + 'static {
    async fn ping(&self, request: Request<system::Ping>) -> Result<Response, Error> {
        Err(Error)
    }
}
struct FromRequest<R>(Request<R>)
where
    R: SubsonicRequest;

#[async_trait]
impl SubsonicService for Subsonic {
    async fn ping(&self, request: Request<system::Ping>) -> Result<Response, Error> {
        Err(Error)
    }
}

macro_rules! declare_handlers {
        ($(($name:ident $t:path)),*) => {
            $(
                async fn $name(
                    service: State<Arc<dyn SubsonicService>>,
                    request: FromRequest<$t>,
                ) -> axum::response::Response {
                    println!("yhea");
                    let request = request.0.clone();
                    let response = match service.$name(request.clone()).await {
                        Ok(response) => response,
                        Err(err) => return err.into_response(),
                    };
                    let response = match request.format.as_ref().map(|s| s.as_str()) {
                        Some("json") => response.to_json().unwrap(),
                        _ => response.to_xml().unwrap()
                    };
                    let response = axum::response::Response::builder()
                        .status(StatusCode::OK)
                        .body(axum::body::Body::from(Bytes::from(response.into_bytes())))
                        .unwrap();
                    response
                }
            )*
        }
    }
declare_handlers!(
(ping system::Ping)
    );
