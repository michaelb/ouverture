use subsonic_types::{
    common::{Milliseconds, Version},
    response::{
        AlbumID3, AlbumWithSongsID3, Artist, ArtistID3, ArtistWithAlbumsID3, ArtistsID3, Directory,
        Genres, Index, IndexID3, Indexes, MusicFolder, MusicFolders, ResponseBody, Child,
    },
};

use super::*;
// Browsing
pub async fn get_music_folders(
    State(server): State<&Server>,
    request: Request<browsing::GetMusicFolders>,
) -> Result<Response> {
    let folders = server.music_folders();

    let unreadable_name = String::from("?????");

    Ok(Response::ok(
        Version::V1_16_1,
        ResponseBody::MusicFolders(MusicFolders {
            music_folder: folders
                .iter()
                .enumerate()
                .map(|(i, f)| MusicFolder {
                    id: (i as u32),
                    name: Some(
                        f.file_name()
                            .unwrap()
                            .to_os_string()
                            .into_string()
                            .unwrap_or(unreadable_name.clone()),
                    ),
                })
                .collect(),
        }),
    ))
}
pub async fn get_indexes(
    State(server): State<&Server>,
    request: Request<browsing::GetIndexes>,
) -> Result<Response> {
    if let Some(folder_id) = request.body.music_folder_id {
        todo!("implement per-directory search");
    }

    if let Some(modified_since) = request.body.if_modified_since {
        todo!("implement modif time search");
    }

    let artists: Vec<_> = server
        .list_artists()
        .await
        .into_iter()
        .map(|s| Artist {
            id: "0".to_string(),
            name: s,
            ..Default::default()
        })
        .collect();
    let index = Index {
        name: "Library".to_string(),
        artist: artists,
    };
    let indexes = Indexes {
        last_modified: Milliseconds::new(0),
        ignored_articles: String::new(),
        shortcut: vec![],
        index: vec![index],
        child: vec![],
    };
    Ok(Response::ok(
        Version::V1_16_1,
        ResponseBody::Indexes(indexes),
    ))
}
pub async fn get_music_directory(
    State(server): State<&Server>,
    request: Request<browsing::GetMusicDirectory>,
) -> Result<Response> {
    let directory = Directory {
        id: "0".to_string(),
        parent: None,
        name: "Library".to_string(),
        ..Default::default()
    };

    Ok(Response::ok(
        Version::V1_16_1,
        ResponseBody::Directory(directory),
    ))
}
pub async fn get_genres(
    State(server): State<&Server>,
    request: Request<browsing::GetGenres>,
) -> Result<Response> {
    Ok(Response::ok(
        Version::V1_16_1,
        ResponseBody::Genres(Genres { genre: vec![] }),
    ))
}
pub async fn get_artists(
    State(server): State<&Server>,
    request: Request<browsing::GetArtists>,
) -> Result<Response> {
    //TODO temporary, change it bc it's different from get indexes
    if let Some(folder_id) = request.body.music_folder_id {
        todo!("implement per-directory search");
    }

    let artists: Vec<_> = server
        .list_artists()
        .await
        .into_iter()
        .map(|s| ArtistID3 {
            id: "0".to_string(),
            name: s,
            ..Default::default()
        })
        .collect();
    let index = IndexID3 {
        name: "Library".to_string(),
        artist: artists,
    };
    let artists = ArtistsID3 {
        ignored_articles: String::new(),
        index: vec![index],
    };
    Ok(Response::ok(
        Version::V1_16_1,
        ResponseBody::Artists(artists),
    ))
}
pub async fn get_artist(
    State(server): State<&Server>,
    request: Request<browsing::GetArtist>,
) -> Result<Response> {
    let artist = ArtistID3 {
        id: request.body.id,
        name: String::from("todoname"),
        ..Default::default()
    };
    let artist_w = ArtistWithAlbumsID3 {
        artist: artist,
        album: vec![],
    };
    Ok(Response::ok(
        Version::V1_16_1,
        ResponseBody::Artist(artist_w),
    ))
}
pub async fn get_album(
    State(server): State<&Server>,
    request: Request<browsing::GetAlbum>,
) -> Result<Response> {
    let album = AlbumID3 {
        id: String::from("albumTODO"),
        name: String::from("albumtodoname"),
        ..Default::default()
    };
    let album_w = AlbumWithSongsID3 {
        album,
        song: vec![],
    };
    Ok(Response::ok(Version::V1_16_1, ResponseBody::Album(album_w)))
}
pub async fn get_song(
    State(server): State<&Server>,
    request: Request<browsing::GetSong>,
) -> Result<Response> {
    let id = request.body.id;
    let child= Child {id, ..Default::default()};
    Ok(Response::ok(Version::V1_16_1, ResponseBody::Song(child)))
}
pub async fn get_videos(
    State(server): State<&Server>,
    request: Request<browsing::GetVideos>,
) -> Result<Response> {
    Err(Error)
}
pub async fn get_video_info(
    State(server): State<&Server>,
    request: Request<browsing::GetVideoInfo>,
) -> Result<Response> {
    Err(Error)
}
pub async fn get_artist_info(
    State(server): State<&Server>,
    request: Request<browsing::GetArtistInfo>,
) -> Result<Response> {
    Err(Error)
}
pub async fn get_artist_info2(
    State(server): State<&Server>,
    request: Request<browsing::GetArtistInfo2>,
) -> Result<Response> {
    Err(Error)
}
pub async fn get_album_info(
    State(server): State<&Server>,
    request: Request<browsing::GetAlbumInfo>,
) -> Result<Response> {
    Err(Error)
}
pub async fn get_album_info2(
    State(server): State<&Server>,
    request: Request<browsing::GetAlbumInfo2>,
) -> Result<Response> {
    Err(Error)
}
pub async fn get_similar_songs(
    State(server): State<&Server>,
    request: Request<browsing::GetSimilarSongs>,
) -> Result<Response> {
    Err(Error)
}
pub async fn get_similar_songs2(
    State(server): State<&Server>,
    request: Request<browsing::GetSimilarSongs2>,
) -> Result<Response> {
    Err(Error)
}
pub async fn get_top_songs(
    State(server): State<&Server>,
    request: Request<browsing::GetTopSongs>,
) -> Result<Response> {
    Err(Error)
}
