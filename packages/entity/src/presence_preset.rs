use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
#[serde(rename_all = "snake_case")]
pub enum DbPresenceIcon {
    #[sea_orm(num_value = 0)]
    Emoji,
    #[sea_orm(num_value = 1)]
    App,
    #[sea_orm(num_value = 2)]
    CustomUpload,
}

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
#[serde(rename_all = "snake_case")]
pub enum DbPresenceTimer {
    #[sea_orm(num_value = 0)]
    Elapsed,
    #[sea_orm(num_value = 1)]
    Countdown,
    #[sea_orm(num_value = 2)]
    Off,
}

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
#[serde(rename_all = "snake_case")]
pub enum DbPresenceKind {
    #[sea_orm(num_value = 0)]
    AppLinked,
    #[sea_orm(num_value = 1)]
    Fixed,
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "presence_presets")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub user_id: Uuid,
    pub label: String,
    pub icon_kind: DbPresenceIcon,
    pub icon_value: Option<String>,
    pub timer_kind: DbPresenceTimer,
    pub timer_seconds: Option<i64>,
    pub preset_kind: DbPresenceKind,
    pub process_name: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    User,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
