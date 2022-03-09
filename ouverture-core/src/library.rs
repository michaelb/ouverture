use crate::config::Config;
use crate::music::song::*;
use async_walkdir::{DirEntry, WalkDir};
use futures_lite::stream::StreamExt;
use log::{debug, error, info, trace, warn};
use std::path::Path;

use crate::database::add_db;

use sea_orm::entity::prelude::*;
use sea_orm::{entity::*, query::*};
use sea_orm::{Database, DatabaseConnection};

use crate::database::*;

pub async fn scan(config: &Config) {
    for path_to_dir in &config.library {
        let mut entries = WalkDir::new(path_to_dir);
        loop {
            match entries.next().await {
                Some(Ok(entry)) => {
                    trace!("Found file in library: {e}", e = entry.path().display());
                    let song = Song::from_path(&entry.path());
                    let res = add_db(config, song).await;
                    trace!("added correctly ? {res:?}");
                }
                Some(Err(e)) => {
                    warn!("error: {}", e);
                    break;
                }
                None => break,
            }
        }
    }
}

pub async fn list(config: &Config, query: Option<String>) {
    let database_url = "postgres://ouverture:ouverture@localhost:".to_string()
        + &config.database_port.to_string()
        + "/ouverture";
    let db = Database::connect(&database_url).await.unwrap();

    let song_found: Vec<setup::Model> = setup::Entity::find().all(&db).await.unwrap();

    println!("{song_found:?}");
}
