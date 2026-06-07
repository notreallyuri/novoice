use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: uuid::Uuid,
    #[sea_orm(unique)]
    pub username: String,
    pub display_name: String,
    pub banner_url: Option<String>,
    pub avatar_url: Option<String>,
    pub bio: Option<String>,
    pub profile_color: Option<String>,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Clone, Copy, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_one = "super::user_account::Entity")]
    UserAccount,
    #[sea_orm(has_one = "super::user_settings::Entity")]
    UserSettings,
    #[sea_orm(has_many = "super::guild_member::Entity")]
    GuildMember,
    #[sea_orm(has_many = "super::guild::Entity")]
    Guild,
    #[sea_orm(has_many = "super::presence_preset::Entity")]
    PresencePresets,
}

impl Related<super::user_settings::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserSettings.def()
    }
}

impl Related<super::user_account::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserAccount.def()
    }
}

impl Related<super::guild_member::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::GuildMember.def()
    }
}

impl Related<super::guild::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Guild.def()
    }
}

impl Related<super::presence_preset::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PresencePresets.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

pub struct UserJoinedGuilds;

impl Linked for UserJoinedGuilds {
    type FromEntity = Entity;
    type ToEntity = super::guild::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![
            super::guild_member::Relation::User.def().rev(),
            super::guild_member::Relation::Guild.def(),
        ]
    }
}
