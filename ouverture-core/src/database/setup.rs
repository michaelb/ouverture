use sea_orm::sea_query::{ColumnDef, TableCreateStatement};
use sea_orm::{error::*, sea_query, ConnectionTrait, DbConn, ExecResult};

use crate::music::song::{Song, SongSource};
use sea_orm::prelude::*;
use sea_orm::{entity::*, query::*};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "songs")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub source: Option<String>,
}
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

async fn create_table(db: &DbConn, stmt: &TableCreateStatement) -> Result<ExecResult, DbErr> {
    let builder = db.get_database_backend();
    db.execute(builder.build(stmt)).await
}

pub async fn create_post_table(db: &DbConn) -> Result<ExecResult, DbErr> {
    let stmt = sea_query::Table::create()
        .table(Entity)
        .if_not_exists()
        .col(
            ColumnDef::new(Column::Id)
                .integer()
                .not_null()
                .auto_increment()
                .primary_key(),
        )
        .col(ColumnDef::new(Column::Title).string())
        .col(ColumnDef::new(Column::Artist).string())
        .col(ColumnDef::new(Column::Album).string())
        .col(ColumnDef::new(Column::Source).string())
        .to_owned();

    create_table(db, &stmt).await
}

impl From<Song> for ActiveModel {
    fn from(s: Song) -> ActiveModel {
        ActiveModel {
            title: Set(s.title),
            artist: Set(s.artist),
            album: Set(s.album),
            source: Set(match s.source {
                None => None,
                Some(source) => Some(source.into()),
            }),
            ..Default::default()
        }
    }
}

impl From<Model> for Song {
    fn from(a: Model) -> Song {
        Song {
            title: {
                match a.title {
                    Some(t) => Some(t),
                    _ => None,
                }
            },
            ..Default::default()
        }
    }
}
