use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
pub enum DbChannelKind {
    #[sea_orm(num_value = 0)]
    Text,
    #[sea_orm(num_value = 1)]
    Voice,
    #[sea_orm(num_value = 2)]
    Docs,
    #[sea_orm(num_value = 3)]
    Canvas,
}

#[derive(
    Default, Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize,
)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
pub enum DbChannelMode {
    #[default]
    #[sea_orm(num_value = 0)]
    Chat,
    #[sea_orm(num_value = 1)]
    Board,
    #[sea_orm(num_value = 2)]
    Threads,
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "channels")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub guild_id: Uuid,
    pub name: String,
    pub position: i32,
    pub category_id: Option<Uuid>,
    pub kind: DbChannelKind,
    pub mode: Option<DbChannelMode>,
    pub user_limit: Option<i32>,
    pub bitrate: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::category::Entity",
        from = "Column::CategoryId",
        to = "super::category::Column::Id",
        on_update = "NoAction",
        on_delete = "SetNull"
    )]
    Category,
    #[sea_orm(
        belongs_to = "super::guild::Entity",
        from = "Column::GuildId",
        to = "super::guild::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Guild,
    #[sea_orm(has_many = "super::channel_override::Entity")]
    ChannelOverrides,
}

impl Related<super::category::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Category.def()
    }
}

impl Related<super::guild::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Guild.def()
    }
}

impl Related<super::channel_override::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ChannelOverrides.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
