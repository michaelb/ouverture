use std::path::PathBuf;
use std::time::Duration;
use chrono::prelude::*;

#[derive(Clone)]
pub enum AudioFormat {
    mp3,
    wav,
    ogg,
    opus,
    flac,
    m4a,
    unsupported,
}

#[derive(Clone)]
pub struct Song {
    /// Artist of the song
    artist: Option<String>,
    /// Album of the song
    album: Option<String>,
    /// Title of the song
    title: Option<String>,


    /// Where to fetch the music data
    source: Option<SongSource>,


    /// Duration of the song
    duration: Duration,

    /// Creation date
    added_date: DateTime<Local>,


    /// Play counter
    play_count: usize,

    /// Skip count
    skip_count: usize,

    /// Rating
    rating: Rating,

    // / uslt lyrics
    // lyric_frames: Vec<Lyrics>,
    // lyric_selected_index: usize,
    // parsed_lyric: Option<Lyric>,
    // picture: Option<Picture>,
    format: AudioFormat,
}


#[derive(Clone)]
pub enum SongSource {
    FilePath(PathBuf),
    YoutubeUrl(String)
}

#[derive(Clone)]
/// Rating in % points, 0 = worst, and 100 = best
pub enum Rating {

    Auto(usize),
    Manual(usize),

    /// Synced from somewhere else (report count update there)
    Sync(usize), 
}


impl Default for Rating {
    fn default() -> Self {
        Rating::Auto(50)
    }
}

impl Default for Song {
    fn default() -> Self {
        Song {
            artist: None,
            album: None,
            title: None,
            source: None,
            duration: Duration::from_millis(0),
            added_date: Local::now(),
            play_count: 0,
            skip_count: 0,
            rating: Rating::default(),
            format: AudioFormat::unsupported,
        }
    }
}
