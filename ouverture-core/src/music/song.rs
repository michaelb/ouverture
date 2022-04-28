use audiotags::Tag;
use chrono::prelude::{DateTime, Local};
use infer;
use log::{debug, info, trace};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::path::{Path, PathBuf};
use std::time::Duration;

use sea_orm::prelude::*;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum AudioFormat {
    mp3,
    wav,
    aac,
    ogg,
    flac,
    m4a,
    unsupported,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
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
    pub format: AudioFormat,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SongSource {
    FilePath(PathBuf),
    YoutubeUrl(String),
    Unknown,
}

use SongSource::*;
impl Into<String> for SongSource {
    fn into(self) -> String {
        match self {
            FilePath(path) => String::from("file:") + path.to_str().unwrap(),
            YoutubeUrl(url) => String::from("yt_url:") + &url,
            Unknown => String::from("unknown"),
        }
    }
}

impl From<String> for SongSource {
    fn from(s: String) -> Self {
        let yt = "yt_url:";
        if s.starts_with(yt) {
            return YoutubeUrl(s.strip_prefix(yt).unwrap().into());
        }
        let path = "path:";
        if s.starts_with(path) {
            return FilePath(s.strip_prefix(path).unwrap().into());
        }

        return Unknown;
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
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
        let tag = Tag::new().read_from_path(path).unwrap();

        let kind = infer::get_from_path(path)
            .expect("file type read successfully")
            .expect("file type known");

        use AudioFormat::*;
        let format = match kind.mime_type() {
            "audio/midi" => unsupported,
            "audio/mpeg" => mp3,
            "audio/m4a" => m4a,
            "audio/ogg" => ogg,
            "audio/x-flac" => flac,
            "audio/x-wav" => wav,
            "audio/amr" => unsupported,
            "audio/aac" => aac,
            "audio/x-aiff" => unsupported,
            _ => unsupported,
        };

        if format == unsupported {
            // TODO reject the new song
        }

        Song {
            source: Some(FilePath(path.to_path_buf())),
            title: tag.title().map(|s| s.into()),
            artist: tag.artists().map(|v| v[0].to_string()),
            album: tag.album().map(|a| a.title.to_string()),
            format,

            ..Default::default()
        }
    }
}
