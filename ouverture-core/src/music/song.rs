use chrono::prelude::{DateTime, Local};
use std::error::Error;
use std::path::{Path, PathBuf};
use std::time::Duration;

use sea_orm::prelude::*;

#[derive(Clone, Debug)]
pub enum AudioFormat {
    mp3,
    wav,
    ogg,
    opus,
    flac,
    m4a,
    unsupported,
}

#[derive(Clone, Debug)]
pub struct Song {
    /// Artist of the song
    pub artist: Option<String>,
    /// Album of the song
    pub album: Option<String>,
    /// Title of the song
    pub title: Option<String>,

    /// Where to fetch the music data
    pub source: Option<SongSource>,

    /// Duration of the song
    pub duration: Duration,

    /// Creation date
    pub added_date: DateTime<Local>,

    /// Play counter
    pub play_count: usize,

    /// Skip count
    pub skip_count: usize,

    /// Rating
    pub rating: Rating,

    // / uslt lyrics
    // lyric_frames: Vec<Lyrics>,
    // lyric_selected_index: usize,
    // parsed_lyric: Option<Lyric>,
    // picture: Option<Picture>,
    format: AudioFormat,
}

#[derive(Clone, Debug)]
pub enum SongSource {
    FilePath(PathBuf),
    YoutubeUrl(String),
}

use SongSource::*;
impl Into<String> for SongSource {
    fn into(self) -> String {
        match self {
            FilePath(path) => String::from("file:") + path.to_str().unwrap(),
            YoutubeUrl(url) => String::from("yt_url:") + &url,
        }
    }
}

#[derive(Clone, Debug)]
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

impl Song {
    pub fn from_path(path: &Path) -> Song {
        Song::default()
    }
}
