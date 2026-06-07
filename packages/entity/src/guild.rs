use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "guilds")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub name: String,
    pub owner_id: Uuid,
    pub icon_url: Option<String>,
    pub banner_url: Option<String>,
    pub default_channel_id: Option<Uuid>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::OwnerId",
        to = "super::user::Column::Id",
        on_delete = "NoAction",
        on_update = "NoAction"
    )]
    Owner,
    #[sea_orm(has_many = "super::guild_member::Entity")]
    GuildMembers,
    #[sea_orm(has_many = "super::category::Entity")]
    Categories,
    #[sea_orm(has_many = "super::channel::Entity")]
    Channels,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Owner.def()
    }
}

impl Related<super::guild_member::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::GuildMembers.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

pub struct GuildToUserMembers;

impl Linked for GuildToUserMembers {
    type FromEntity = Entity;
    type ToEntity = super::user::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![
            super::guild_member::Relation::Guild.def().rev(),
            super::guild_member::Relation::User.def(),
        ]
    }
}
