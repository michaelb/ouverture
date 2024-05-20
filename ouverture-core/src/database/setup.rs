use sea_orm::sea_query::{ColumnDef, TableCreateStatement};
use sea_orm::{error::*, sea_query, ConnectionTrait, DbConn, ExecResult};

use sea_orm::prelude::*;
use sea_orm::{entity::*, query::*};
use sea_orm::{entity::prelude::*, ActiveValue};

use std::time::Duration;

use color_eyre::Result;

use super::artist;
use super::song;
use super::album;


use log::{info, debug};


pub async fn create_table_helper(db: &DbConn, stmt: &TableCreateStatement) -> Result<ExecResult, DbErr> {
    let builder = db.get_database_backend();
    db.execute(builder.build(stmt)).await
}

pub async fn create_post_table(db: &DbConn) -> Result<(), DbErr> {
    song::create_post_table(db).await?;
    album::create_post_table(db).await?;
    artist::create_post_table(db).await?;
    debug!("created all db tables");
    Ok(())
}
