use crate::music::song::*;
use walkdir::{WalkDir, DirEntry};
use log::{error,trace, info, debug};
use std::path::Path;

pub fn scan(path_to_dir: &Path) {
    for entry in WalkDir::new(path_to_dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| !e.file_type().is_dir())
    {
        trace!("Found file in library: {e}", e=entry.path().display());
        let song = Song::from_path(entry.path());

    }
}
