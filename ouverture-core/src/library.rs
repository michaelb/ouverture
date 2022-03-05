use crate::config::Config;
use crate::music::song::*;
use log::{debug, error, info, trace};
use std::path::Path;
use walkdir::{DirEntry, WalkDir};

pub fn scan(config: &Config) {
    for path_to_dir in &config.library {
        for entry in WalkDir::new(path_to_dir)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| !e.file_type().is_dir())
        {
            trace!("Found file in library: {e}", e = entry.path().display());
            let song = Song::from_path(entry.path());
        }
    }
}
