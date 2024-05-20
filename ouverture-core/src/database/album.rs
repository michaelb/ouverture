
use sea_orm::prelude::*;
use sea_orm::{entity::*, query::*, ExecResult};

use sea_orm::sea_query::{Table, ColumnDef, TableCreateStatement};

use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "album")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub name: String,
    // pub artist: String,
    pub artist_id: Option<i64>,
    pub year: Option<i16>,
    // pub cover: Option<String>,
    #[sea_orm(ignore)]
    pub tracks: Vec<super::song::Model>,
}


impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::song::Entity")]
    Song,
    #[sea_orm(
        belongs_to = "super::artist::Entity",
        from = "Column::ArtistId",
        to = "super::artist::Column::Id"
    )]
    Artist,
}

impl Related<super::song::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Song.def()
    }
}

impl Related<super::artist::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Artist.def()
    }
}

pub async fn create_post_table(db: &DbConn) -> Result<ExecResult, DbErr> {
    let stmt = Table::create()
        .table(Entity)
        .if_not_exists()
        .col(
            ColumnDef::new(Column::Id)
                .big_integer()
                .not_null()
                .auto_increment()
                .primary_key(),
        )
        .col(ColumnDef::new(Column::Name).string())
        .col(ColumnDef::new(Column::ArtistId).big_integer())
        .col(ColumnDef::new(Column::Year).small_integer())
        .to_owned();

    super::setup::create_table_helper(db, &stmt).await
}
