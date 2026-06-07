use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
#[serde(rename_all = "snake_case")]
pub enum DbRelationshipStatus {
    #[sea_orm(num_value = 0)]
    None,
    #[sea_orm(num_value = 1)]
    Friend,
    #[sea_orm(num_value = 2)]
    Blocked,
    #[sea_orm(num_value = 3)]
    PendingIncoming,
    #[sea_orm(num_value = 4)]
    PendingOutgoing,
}

#[derive(Clone, Debug, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "relationships")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub user_id: Uuid,
    #[sea_orm(primary_key, auto_increment = false)]
    pub target_id: Uuid,
    pub status: DbRelationshipStatus,
    pub since: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::TargetId",
        to = "super::user::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Users2,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Users1,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Users2.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
