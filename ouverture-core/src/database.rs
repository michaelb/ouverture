    mod setup;

use pg_embed::pg_enums::PgAuthMethod;
use pg_embed::pg_errors::PgEmbedError;
use pg_embed::pg_fetch::{PgFetchSettings, PG_V13};
use pg_embed::postgres::{PgEmbed, PgSettings};
use platform_dirs::AppDirs;
use std::error::Error;
use std::path::PathBuf;
use std::time::Duration;
use tokio::net::TcpListener;

use crate::config::Config;
use crate::music::song::*;
use log::{debug, info};

use sea_orm::entity::prelude::*;
use sea_orm::{entity::*, query::*};
use sea_orm::{Database, DatabaseConnection};

pub async fn setup_db(config: Config) -> Result<PgEmbed, Box<dyn Error>> {
    std::fs::create_dir_all(config.database_dir.clone())?;
    let pg_settings = PgSettings {
        // Where to store the postgresql database
        database_dir: PathBuf::from(config.database_dir),
        port: config.database_port.parse().unwrap(),
        user: "ouverture".to_string(),
        password: "ouverture".to_string(),

        // authentication method
        auth_method: PgAuthMethod::Plain,
        // If persistent is false clean up files and directories on drop, otherwise keep them
        persistent: false,
        // duration to wait before terminating process execution
        // pg_ctl start/stop and initdb timeout
        // if set to None the process will not be terminated
        timeout: Some(Duration::from_secs(15)),
        // If migration sql scripts need to be run, the directory containing those scripts can be
        // specified here with `Some(PathBuf(path_to_dir)), otherwise `None` to run no migrations.
        // To enable migrations view the **Usage** section for details
        migration_dir: None,
    };

    let fetch_settings = PgFetchSettings {
        version: PG_V13,
        ..Default::default()
    };

    let mut pg = PgEmbed::new(pg_settings, fetch_settings).await?;

    // Download, unpack, create password file and database cluster
    pg.setup().await?;

    Ok(pg)
}

pub async fn start_db(pg: &mut PgEmbed, config: Config) -> Result<(), Box<dyn Error>> {
    pg.start_db().await?;

    // First time setup
    if !pg.database_exists("ouverture").await? {
        pg.create_database("ouverture").await?;
        info!("empty database created");

        let database_url = "postgres://ouverture:ouverture@localhost:".to_string()
            + &config.database_port
            + "/ouverture";

        let conn = Database::connect(&database_url).await?;
        let _ = setup::create_post_table(&conn).await;
    }

    Ok(())
}

pub async fn add_db(config: Config, song: Song) -> Result<(), Box<dyn Error>> {
    let database_url = "postgres://ouverture:ouverture@localhost:".to_string()
        + &config.database_port
        + "/ouverture";
    let db = Database::connect(&database_url).await.unwrap();
    debug!("Adding song {song:?}");
    setup::ActiveModel::from(song).insert(&db).await?;
    debug!("Success!");
    Ok(())
}

pub async fn test_db(config: Config) {
    let database_url = "postgres://ouverture:ouverture@localhost:".to_string()
        + &config.database_port
        + "/ouverture";
    debug!("test DB connection established");
    let db = Database::connect(&database_url).await.unwrap();
    let test_song = setup::ActiveModel {
        title: Set(Some("test title".to_owned())),
        ..Default::default()
    };

    let res = test_song.insert(&db).await.unwrap();
    debug!("insert result : {:?}", res);

    let song_found: Option<setup::Model> = setup::Entity::find_by_id(1).one(&db).await.unwrap();
    debug!("song found: {:?}", song_found);

}
