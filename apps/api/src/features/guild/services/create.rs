use crate::core::{error::AppError, mappers::FromDomain, state::SharedState};
use entity::{
    channel::{self, DbChannelKind, DbChannelMode},
    guild, guild_member, guild_member_role, role, user,
};
use sea_orm::{ActiveModelTrait, EntityTrait, Set, TransactionTrait};
use shared::{
    data::{
        ChannelId, GuildId, RoleId, UserId,
        channel::{
            Channel,
            text_channel::{ChannelMode, TextChannel},
            voice_channel::VoiceChannel,
        },
        guild::{Guild, GuildMember, GuildProfile, Role},
        user::{Status, UserPresence, UserProfile, UserPublic},
    },
    dtos::guild::CreateGuildRequest,
};
use uuid::Uuid;

pub async fn create(
    state: &SharedState,
    user_id: UserId,
    payload: CreateGuildRequest,
) -> Result<Guild, AppError> {
    let owner = user::Entity::find_by_id(user_id.0)
        .one(&state.db)
        .await?
        .ok_or(AppError::NotFound)?;

    let guild_id = Uuid::new_v4();
    let text_channel_id = Uuid::new_v4();
    let voice_channel_id = Uuid::new_v4();
    let everyone_role_id = Uuid::new_v4();
    let member_id = Uuid::new_v4();
    let now_utc = chrono::Utc::now();
    let now = now_utc.into();

    let new_guild = guild::ActiveModel {
        id: Set(guild_id),
        owner_id: Set(user_id.0),
        name: Set(payload.name.clone()),
        default_channel_id: Set(None),
        banner_url: Set(None),
        icon_url: Set(None),
    };

    let new_member = guild_member::ActiveModel {
        id: Set(member_id),
        guild_id: Set(guild_id),
        user_id: Set(user_id.0),
        joined_at: Set(now),

        identity_show_global_username: Set(false),
        identity_display_name: Set(None),
        identity_avatar: Set(None),
        identity_banner: Set(None),
        identity_bio: Set(None),
    };

    let default_role = role::ActiveModel {
        id: Set(everyone_role_id),
        guild_id: Set(guild_id),
        name: Set("@everyone".to_string()),
        permissions: Set(0),
        color: Set(None),
        hoist: Set(false),
        position: Set(0),
    };

    let member_role_link = guild_member_role::ActiveModel {
        role_id: Set(everyone_role_id),
        guild_member_id: Set(member_id),
    };

    let new_text_channel = channel::ActiveModel {
        id: Set(text_channel_id),
        guild_id: Set(guild_id),
        name: Set("general".to_string()),
        position: Set(0),
        category_id: Set(None),
        kind: Set(DbChannelKind::Text),
        mode: Set(Some(DbChannelMode::Chat)),
        bitrate: Set(None),
        user_limit: Set(None),
    };

    let new_voice_channel = channel::ActiveModel {
        id: Set(voice_channel_id),
        guild_id: Set(guild_id),
        name: Set("General".to_string()),
        position: Set(1),
        category_id: Set(None),
        kind: Set(DbChannelKind::Voice),
        mode: Set(None),
        bitrate: Set(Some(64000)),
        user_limit: Set(None),
    };

    let txn = state.db.begin().await?;

    let inserted_guild = new_guild.insert(&txn).await?;
    new_member.insert(&txn).await?;
    default_role.insert(&txn).await?;
    member_role_link.insert(&txn).await?;
    new_text_channel.insert(&txn).await?;
    new_voice_channel.insert(&txn).await?;

    let mut guild_to_update: guild::ActiveModel = inserted_guild.into();
    guild_to_update.default_channel_id = Set(Some(text_channel_id));
    guild_to_update.update(&txn).await?;

    txn.commit().await?;

    let constructed_role = Role {
        id: RoleId(everyone_role_id),
        name: "@everyone".to_string(),
        color: None,
        hoist: false,
        position: 0,
        permissions: 0,
    };

    let constructed_guild = Guild {
        id: GuildId(guild_id),
        profile: GuildProfile {
            owner_id: user_id,
            banner_url: None,
            icon_url: None,
            name: payload.name,
            default_channel_id: Some(ChannelId(text_channel_id)),
        },
        roles: vec![constructed_role],
        categories: vec![],
        members: vec![GuildMember {
            guild_id: GuildId(guild_id),
            user_id,
            roles: vec![RoleId(everyone_role_id)],
            joined_at: now_utc.to_rfc3339(),
            data: UserPublic {
                id: user_id,
                profile: UserProfile::from_domain(owner),
                presence: UserPresence {
                    status: Status::Online,
                    preset: None,
                },
            },
            identity: None,
        }],
        channels: vec![
            Channel::Text(TextChannel {
                id: ChannelId(text_channel_id),
                guild_id: GuildId(guild_id),
                category_id: None,
                name: "general".to_string(),
                position: 0,
                mode: ChannelMode::Chat,
            }),
            Channel::Voice(VoiceChannel {
                id: ChannelId(voice_channel_id),
                guild_id: GuildId(guild_id),
                category_id: None,
                name: "General".to_string(),
                position: 1,
                user_limit: None,
                bitrate: 64000,
                participants: vec![],
            }),
        ],
    };

    Ok(constructed_guild)
}
