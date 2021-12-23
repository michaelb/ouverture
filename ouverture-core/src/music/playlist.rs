use super::song::Song;

#[derive(Clone)]
pub struct Playlist {
    songs: Vec<Song>,
}
