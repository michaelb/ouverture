use crate::music::song::Song;
use sea_orm::prelude::*;
use sea_orm::{entity::*, query::*, DbBackend};

use sea_orm::schema::Schema;
use sea_orm::{error::*, sea_query, ConnectionTrait, DbConn, ExecResult};
use sea_orm::sea_query::{ColumnDef, TableCreateStatement};

use chrono::NaiveDate;

use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "songs")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub title: Option<String>,
    pub source: Option<String>,
    pub duration: i64, // duration of the song in milliseconds
    pub date: Option<NaiveDate>,
    pub bitrate: Option<i64>,
    pub sample_rate: Option<i64>,
    pub album_id: Option<i64>,
    pub artist_id: Option<i64>,
    #[sea_orm(ignore)]
    pub artists: Vec<super::artist::Model>,
    #[sea_orm(ignore)]
    pub album: super::album::Model,
}
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::album::Entity",
        from = "Column::AlbumId",
        to = "super::album::Column::Id"
    )]
    Album,
    #[sea_orm(has_many = "super::artist::Entity")]
    Artist,
}
impl Related<super::album::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Album.def()
    }
}

impl Related<super::artist::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Artist.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

/// Convert a song into an insertable Model
impl From<Song> for ActiveModel {
    fn from(s: Song) -> ActiveModel {
        ActiveModel {
            title: Set(s.title),
            // artist: Set(s.artist),
            // album: Set(s.album),
            source: Set(match s.source {
                None => None,
                Some(source) => Some(source.into()),
            }),
            duration: Set(s.duration.as_millis() as i64),
            ..Default::default()
        }
    }
}

impl From<Model> for Song {
    fn from(a: Model) -> Song {
        Song {
            title: a.title.into(),
            // artist: a.artist.into(),
            // album: a.album.into(),
            source: match a.source {
                None => None,
                Some(source) => Some(source.into()),
            },
            duration: Duration::from_millis(a.duration as u64),
            ..Default::default()
        }
    }
}

pub async fn create_post_table(db: &DbConn) -> Result<ExecResult, DbErr> {
    let stmt = sea_query::Table::create()
        .table(Entity)
        .if_not_exists()
        .col(
            ColumnDef::new(Column::Id)
                .big_integer()
                .not_null()
                .auto_increment()
                .primary_key(),
        )
        .col(ColumnDef::new(Column::Title).string())
        .col(ColumnDef::new(Column::ArtistId).big_integer())
        .col(ColumnDef::new(Column::AlbumId).big_integer())
        .col(ColumnDef::new(Column::Date).date())
        .col(ColumnDef::new(Column::Bitrate).big_integer())
        .col(ColumnDef::new(Column::SampleRate).big_integer())
        .col(ColumnDef::new(Column::Source).string())
        .col(ColumnDef::new(Column::Duration).big_integer())
        .to_owned();

    super::setup::create_table_helper(db, &stmt).await
}
