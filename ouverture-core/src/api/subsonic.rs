// Example proxy using subsonic-types
// Check main function for default configuration

use std::net::SocketAddr;

use async_trait::async_trait;
// use axum::body::Bytes;
use subsonic_types::request::{system, Request, SubsonicRequest};

pub struct Subsonic {}

use crate::server::Server;
use log::{debug, info, trace};
use std::sync::Arc;

use axum::middleware::from_extractor_with_state;
use axum::{
    extract::{FromRequestParts, State},
    response::IntoResponse,
    routing::get,
    Router,
};
use bytes::Bytes;
use hyper::StatusCode;
use subsonic_types::{
    request::{
        annotation, bookmark, browsing, chat, jukebox, lists, playlists, podcast, radio, retrieval,
        scan, search, sharing, user,
    },
    response::Response,
};

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug)]
pub struct Error;

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        info!("bad request to subsonic API");
        axum::response::Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(axum::body::Body::empty())
            .unwrap()
    }
}

static SUBSONIC_API_VERSION: &str = "1.16.1";

mod s_annotation;
mod s_bookmark;
mod s_browsing;
mod s_chat;
mod s_list;
mod s_playlist;
mod s_podcast;
mod s_radio;
mod s_retrieval;
mod s_scan;
mod s_sharing;
mod s_system;
mod s_user;

mod auth;
use auth::AuthType;
use auth::RequireAuth;

//actual actions are defined in other files, this only re-export them under one name
mod act {

    pub use super::s_annotation::*;
    pub use super::s_bookmark::*;
    pub use super::s_browsing::*;
    pub use super::s_chat::*;
    pub use super::s_list::*;
    pub use super::s_playlist::*;
    pub use super::s_podcast::*;
    pub use super::s_radio::*;
    pub use super::s_retrieval::*;
    pub use super::s_scan::*;
    pub use super::s_sharing::*;
    pub use super::s_system::*;
    pub use super::s_user::*;
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
            annotation::Star::PATH => star,
            annotation::Unstar::PATH => unstar,
            annotation::SetRating::PATH => set_rating,
            annotation::Scrobble::PATH => scrobble,
            // Bookmarks
            bookmark::GetBookmarks::PATH => get_bookmarks,
            bookmark::CreateBookmark::PATH => create_bookmark,
            bookmark::DeleteBookmark::PATH => delete_bookmark,
            bookmark::GetPlayQueue::PATH => get_play_queue,
            bookmark::SavePlayQueue::PATH => save_play_queue,
            // Browsing
            browsing::GetMusicFolders::PATH => get_music_folders,
            browsing::GetIndexes::PATH => get_indexes,
            browsing::GetMusicDirectory::PATH => get_music_directory,
            browsing::GetGenres::PATH => get_genres,
            browsing::GetArtists::PATH => get_artists,
            browsing::GetArtist::PATH => get_artist,
            browsing::GetAlbum::PATH => get_album,
            browsing::GetSong::PATH => get_song,
            browsing::GetVideos::PATH => get_videos,
            browsing::GetVideoInfo::PATH => get_video_info,
            browsing::GetArtistInfo::PATH => get_artist_info,
            browsing::GetArtistInfo2::PATH => get_artist_info2,
            browsing::GetAlbumInfo::PATH => get_album_info,
            browsing::GetAlbumInfo2::PATH => get_album_info2,
            browsing::GetSimilarSongs::PATH => get_similar_songs,
            browsing::GetSimilarSongs2::PATH => get_similar_songs2,
            browsing::GetTopSongs::PATH => get_top_songs,
            // Chat
            chat::GetChatMessages::PATH => get_chat_messages,
            chat::AddChatMessage::PATH => add_chat_message,
            // Jukebox
            jukebox::JukeboxControl::PATH => jukebox_control,
            // Lists
            lists::GetAlbumList::PATH => get_album_list,
            lists::GetAlbumList2::PATH => get_album_list2,
            lists::GetRandomSongs::PATH => get_random_songs,
            lists::GetSongsByGenre::PATH => get_songs_by_genre,
            lists::GetNowPlaying::PATH => get_now_playing,
            lists::GetStarred::PATH => get_starred,
            lists::GetStarred2::PATH => get_starred2,
            // Playlists
            playlists::GetPlaylists::PATH => get_playlists,
            playlists::GetPlaylist::PATH => get_playlist,
            playlists::CreatePlaylist::PATH => create_playlist,
            playlists::UpdatePlaylist::PATH => update_playlist,
            playlists::DeletePlaylist::PATH => delete_playlist,
            // Podcasts
            podcast::GetPodcasts::PATH => get_podcasts,
            podcast::GetNewestPodcasts::PATH => get_newest_podcasts,
            podcast::RefreshPodcasts::PATH => refresh_podcasts,
            podcast::CreatePodcastChannel::PATH => create_podcast_channel,
            podcast::DeletePodcastChannel::PATH => delete_podcast_channel,
            podcast::DeletePodcastEpisode::PATH => delete_podcast_episode,
            podcast::DownloadPodcastEpisode::PATH => download_podcast_episode,
            // Radio
            radio::GetInternetRadioStations::PATH => get_internet_radio_stations,
            radio::CreateInternetRadioStation::PATH => create_internet_radio_station,
            radio::UpdateInternetRadioStation::PATH => update_internet_radio_station,
            radio::DeleteInternetRadioStation::PATH => delete_internet_radio_station,
            // Retrieval
            retrieval::Stream::PATH => stream,
            retrieval::Download::PATH => download,
            retrieval::Hls::PATH => hls,
            retrieval::GetCaptions::PATH => get_captions,
            retrieval::GetCoverArt::PATH => get_cover_art,
            retrieval::GetLyrics::PATH => get_lyrics,
            retrieval::GetAvatar::PATH => get_avatar,
            // Scan
            scan::GetScanStatus::PATH => get_scan_status,
            scan::StartScan::PATH => start_scan,
            // Search
            search::Search::PATH => search,
            search::Search2::PATH => search2,
            search::Search3::PATH => search3,
            // Sharing
            sharing::GetShares::PATH => get_shares,
            sharing::CreateShare::PATH => create_share,
            sharing::UpdateShare::PATH => update_share,
            sharing::DeleteShare::PATH => delete_share,
            // System
            system::Ping::PATH => ping,
            system::GetLicense::PATH => get_license,
            // User
            user::GetUser::PATH => get_user,
            user::GetUsers::PATH => get_users,
            user::CreateUser::PATH => create_user,
            user::UpdateUser::PATH => update_user,
            user::DeleteUser::PATH => delete_user,
            user::ChangePassword::PATH => change_password
        }
        .route("/", get(root))
        .route_layer(from_extractor_with_state::<RequireAuth, ()>(()))
    }
}
async fn root() -> &'static str {
    "Hello Subsonic"
}

struct FromRequest<R>(Request<R>)
where
    R: SubsonicRequest;

#[async_trait]
impl<S, R> FromRequestParts<S> for FromRequest<R>
where
    R: SubsonicRequest,
{
    type Rejection = Error;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        trace!("uri: {}", parts.uri);
        match parts.uri.query() {
            Some(query) => Request::<R>::from_query(query)
                .map(FromRequest)
                .map_err(|e| {
                    debug!("failed to parse request: {}", e);
                    Error
                }),
            None => {
                debug!("failed to parse request");
                Err(Error)
            }
        }
    }
}

macro_rules! declare_handlers {
        ($(($name:ident $t:path)),*) => {
            $(
                async fn $name(
                    service: State<&'static Server>,
                    request: FromRequest<$t>,
                ) -> axum::response::Response {
                    let request = request.0.clone();
                    debug!("handler $name called");
                    let response = match act::$name(service, request.clone()).await {
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

macro_rules! declare_handlers_binary {
        ($(($name:ident $t:path)),*) => {
            $(
                async fn $name(
                    service: State<&'static Server>,
                    request: FromRequest<$t>,
                ) -> axum::response::Response {
                    debug!("called handler $name");
                    let request = request.0.clone();
                    let response = match act::$name(service,request.clone()).await {
                        Ok(response) => response,
                        Err(err) => return err.into_response(),
                    };
                    let response = axum::response::Response::builder()
                        .status(StatusCode::OK)
                        .body(axum::body::Body::from(response))
                        .unwrap();
                    response
                }
            )*
        }
    }

declare_handlers!(
    // Annotation
    (star annotation::Star),
    (unstar annotation::Unstar),
    (set_rating annotation::SetRating),
    (scrobble annotation::Scrobble),

    // Bookmarks
    (get_bookmarks bookmark::GetBookmarks),
    (create_bookmark bookmark::CreateBookmark),
    (delete_bookmark bookmark::DeleteBookmark),
    (get_play_queue bookmark::GetPlayQueue),
    (save_play_queue bookmark::SavePlayQueue),

    // Browsing
    (get_music_folders browsing::GetMusicFolders),
    (get_indexes browsing::GetIndexes),
    (get_music_directory browsing::GetMusicDirectory),
    (get_genres browsing::GetGenres),
    (get_artists browsing::GetArtists),
    (get_artist browsing::GetArtist),
    (get_album browsing::GetAlbum),
    (get_song browsing::GetSong),
    (get_videos browsing::GetVideos),
    (get_video_info browsing::GetVideoInfo),
    (get_artist_info browsing::GetArtistInfo),
    (get_artist_info2 browsing::GetArtistInfo2),
    (get_album_info browsing::GetAlbumInfo),
    (get_album_info2 browsing::GetAlbumInfo2),
    (get_similar_songs browsing::GetSimilarSongs),
    (get_similar_songs2 browsing::GetSimilarSongs2),
    (get_top_songs browsing::GetTopSongs),

    // Chat
    (get_chat_messages chat::GetChatMessages),
    (add_chat_message chat::AddChatMessage),

    // Jukebox
    (jukebox_control jukebox::JukeboxControl),

    // Lists
    (get_album_list lists::GetAlbumList),
    (get_album_list2 lists::GetAlbumList2),
    (get_random_songs lists::GetRandomSongs),
    (get_songs_by_genre lists::GetSongsByGenre),
    (get_now_playing lists::GetNowPlaying),
    (get_starred lists::GetStarred),
    (get_starred2 lists::GetStarred2),

    // Playlists
    (get_playlists playlists::GetPlaylists),
    (get_playlist playlists::GetPlaylist),
    (create_playlist playlists::CreatePlaylist),
    (update_playlist playlists::UpdatePlaylist),
    (delete_playlist playlists::DeletePlaylist),

    // Podcasts
    (get_podcasts podcast::GetPodcasts),
    (get_newest_podcasts podcast::GetNewestPodcasts),
    (refresh_podcasts podcast::RefreshPodcasts),
    (create_podcast_channel podcast::CreatePodcastChannel),
    (delete_podcast_channel podcast::DeletePodcastChannel),
    (delete_podcast_episode podcast::DeletePodcastEpisode),
    (download_podcast_episode podcast::DownloadPodcastEpisode),

    // Radio
    (get_internet_radio_stations radio::GetInternetRadioStations),
    (create_internet_radio_station radio::CreateInternetRadioStation),
    (update_internet_radio_station radio::UpdateInternetRadioStation),
    (delete_internet_radio_station radio::DeleteInternetRadioStation),

    // Retrieval
    (hls retrieval::Hls),
    (get_captions retrieval::GetCaptions),
    (get_lyrics retrieval::GetLyrics),

    // Scan
    (get_scan_status scan::GetScanStatus),
    (start_scan scan::StartScan),

    // Search
    (search search::Search),
    (search2 search::Search2),
    (search3 search::Search3),

    // Sharing
    (get_shares sharing::GetShares),
    (create_share sharing::CreateShare),
    (update_share sharing::UpdateShare),
    (delete_share sharing::DeleteShare),

    // System
    (ping system::Ping),
    (get_license system::GetLicense),

    // User
    (get_user user::GetUser),
    (get_users user::GetUsers),
    (create_user user::CreateUser),
    (update_user user::UpdateUser),
    (delete_user user::DeleteUser),
    (change_password user::ChangePassword)
);

declare_handlers_binary! {
    // Retrieval
    (stream retrieval::Stream),
    (download retrieval::Download),
    (get_cover_art retrieval::GetCoverArt),
    (get_avatar retrieval::GetAvatar)
}
