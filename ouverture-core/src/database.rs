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
use log::info;

use diesel::pg::PgConnection;
use diesel::prelude::*;

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

        PgConnection::establish(&database_url)
            .expect(&format!("Error connecting to {}", database_url));
    }

    Ok(())
}

pub async fn test() -> Result<PgEmbed, Box<dyn Error>> {
    let app_dirs = AppDirs::new(Some("ouverture/postgres"), true).unwrap();
    std::fs::create_dir_all(&app_dirs.data_dir).unwrap();

    let pg_settings = PgSettings {
        // Where to store the postgresql database
        database_dir: PathBuf::from(&app_dirs.data_dir),
        port: 5432,
        user: "postgres".to_string(),
        password: "password".to_string(),

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

    let mut pg = PgEmbed::new(pg_settings, fetch_settings).await?;

    {
        // Download, unpack, create password file and database cluster
        pg.setup().await?;

        // start postgresql database
        pg.start_db().await?;

        // create a new database
        // to enable migrations view the [Usage] section for details
        pg.create_database("database_name").await?;

        // drop a database
        // to enable migrations view [Usage] for details
        pg.drop_database("database_name").await?;

        // check database existence
        // to enable migrations view [Usage] for details
        pg.database_exists("database_name").await?;

        // run migration sql scripts
        // to enable migrations view [Usage] for details
        pg.migrate("database_name").await?;

        // stop postgresql database
        pg.stop_db().await?;
    }
    // get the base postgresql uri
    // `postgres://{username}:{password}@localhost:{port}`
    let pg_uri: &str = &pg.db_uri;

    // get a postgresql database uri
    // `postgres://{username}:{password}@localhost:{port}/{specified_database_name}`
    let pg_db_uri: String = pg.full_db_uri("database_name");
    println!("pg_uri: = {:?}", pg_uri);
    println!("pg_db_uri: = {:?}", pg_db_uri);
    println!("done");
    return Ok(pg);
}
