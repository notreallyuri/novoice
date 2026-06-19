use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "audit_logs")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub guild_id: Uuid,
    pub actor_id: Uuid,
    pub target_id: Option<Uuid>,
    pub action_type: i32,
    pub reason: Option<String>,
    pub changes: Option<serde_json::Value>,
    pub created_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::guild::Entity",
        from = "Column::GuildId",
        to = "super::guild::Column::Id",
        on_delete = "Cascade"
    )]
    Guild,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::ActorId",
        to = "super::user::Column::Id",
        on_delete = "Cascade"
    )]
    Actor,
}

impl Related<super::guild::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Guild.def()
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Actor.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
