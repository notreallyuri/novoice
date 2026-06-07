use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "guild_members_roles")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub role_id: Uuid,
    #[sea_orm(primary_key, auto_increment = false)]
    pub guild_member_id: Uuid,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::role::Entity",
        from = "Column::RoleId",
        to = "super::role::Column::Id",
        on_delete = "Cascade"
    )]
    Role,
    #[sea_orm(
        belongs_to = "super::guild_member::Entity",
        from = "Column::GuildMemberId",
        to = "super::guild_member::Column::Id",
        on_delete = "Cascade"
    )]
    GuildMember,
}

impl Related<super::role::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Role.def()
    }
}

impl Related<super::guild_member::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::GuildMember.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
