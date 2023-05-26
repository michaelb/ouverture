pub mod setup;

use pg_embed::pg_enums::PgAuthMethod;
use pg_embed::pg_errors::PgEmbedError;
use pg_embed::pg_fetch::{PgFetchSettings, PG_V13};
use pg_embed::postgres::{PgEmbed, PgSettings};
use platform_dirs::AppDirs;
use std::path::PathBuf;
use std::time::Duration;

use crate::config::Config;
use crate::music::song::*;
use log::{debug, info};

use color_eyre::eyre::eyre;
use color_eyre::Result;
use sea_orm::entity::prelude::*;
use sea_orm::{entity::*, query::*};
use sea_orm::{Database, DatabaseConnection};

pub async fn setup_db(config: Config) -> Result<PgEmbed> {
    std::fs::create_dir_all(config.database_dir.clone())?;
    let pg_settings = PgSettings {
        // Where to store the postgresql database
        database_dir: PathBuf::from(config.database_dir),
        port: config.database_port as u16,
        user: "ouverture".to_string(),
        password: "ouverture".to_string(),

        // authentication method
        auth_method: PgAuthMethod::Plain,
        // If persistent is false clean up files and directories on drop, otherwise keep them
        persistent: true,
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
    info!(
        "database fetch settings: host = {:?}, platform = {:?}",
        fetch_settings.host,
        fetch_settings.platform()
    );

    let mut pg = PgEmbed::new(pg_settings, fetch_settings)
        .await
        .map_err(|e| eyre!(e.to_string()))?;

    // Download, unpack, create password file and database cluster
    pg.setup().await.map_err(|e| eyre!(e.to_string()))?;

    Ok(pg)
}

pub async fn start_db(pg: &mut PgEmbed, config: Config) -> Result<()> {
    pg.start_db().await.map_err(|e| eyre!(e.to_string()))?;

    // First time setup
    if !pg
        .database_exists("ouverture")
        .await
        .map_err(|e| eyre!(e.to_string()))?
    {
        pg.create_database("ouverture")
            .await
            .map_err(|e| eyre!(e.to_string()))?;
        info!("empty database created");

        let database_url = "postgres://ouverture:ouverture@localhost:".to_string()
            + &config.database_port.to_string()
            + "/ouverture";

        let conn = Database::connect(&database_url).await?;
        let _ = setup::create_post_table(&conn).await;
    }

    Ok(())
}

pub async fn add_db(config: &Config, song: Song) -> Result<()> {
    let database_url = "postgres://ouverture:ouverture@localhost:".to_string()
        + &config.database_port.to_string()
        + "/ouverture";
    let db = Database::connect(&database_url).await.unwrap();
    debug!("Adding song {song:?}");
    setup::ActiveModel::from(song).insert(&db).await?;
    debug!("Song added to db successfully!");
    Ok(())
}

pub async fn test_db(config: Config) {
    let database_url = "postgres://ouverture:ouverture@localhost:".to_string()
        + &config.database_port.to_string()
        + "/ouverture";
    let db = Database::connect(&database_url).await.unwrap();
    debug!("test DB connection established");
    let test_song = setup::ActiveModel {
        title: Set(Some("test title".to_owned())),
        ..Default::default()
    };

    let res = test_song.insert(&db).await.unwrap();
    debug!("insert result : {:?}", res);

    let song_found: Option<setup::Model> = setup::Entity::find_by_id(1).one(&db).await.unwrap();
    debug!("song found: {:?}", song_found);
}
