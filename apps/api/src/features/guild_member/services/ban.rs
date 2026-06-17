use crate::core::{
    broadcast, cache::guild_members::cache_remove_member, error::AppError,
    guards::verify_permission, state::SharedState,
};
use entity::{guild, guild_ban, guild_member};
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, ModelTrait, QueryFilter,
};
use shared::{
    data::{GuildId, UserId, permissions::Permissions},
    dtos::guild_member::BanMemberRequest,
    ws::{ServerMessage, guild::GuildServerEvents},
};

pub async fn ban(
    state: &SharedState,
    user_id: UserId,
    guild_id: GuildId,
    target_id: UserId,
    req: BanMemberRequest,
) -> Result<(), AppError> {
    verify_permission(&state.db, user_id, guild_id, Permissions::BAN_MEMBERS).await?;

    let target_guild = guild::Entity::find_by_id(guild_id.0)
        .one(&state.db)
        .await?
        .ok_or(AppError::NotFound)?;

    if target_guild.owner_id == target_id.0 {
        return Err(AppError::Forbidden("Cannot ban the server owner.".into()));
    }

    let now = chrono::Utc::now();

    let existing_ban = guild_ban::Entity::find()
        .filter(guild_ban::Column::GuildId.eq(guild_id.0))
        .filter(guild_ban::Column::UserId.eq(target_id.0))
        .one(&state.db)
        .await?;

    if existing_ban.is_none() {
        let new_ban = guild_ban::ActiveModel {
            guild_id: Set(guild_id.0),
            user_id: Set(target_id.0),
            reason: Set(req.reason),
            expires_at: Set(req.expires_at.map(|dt| dt.into())),
            banned_by: Set(target_id.0),
            created_at: Set(now.into()),
        };
        new_ban.insert(&state.db).await?;
    }

    let member = guild_member::Entity::find()
        .filter(guild_member::Column::UserId.eq(target_id.0))
        .filter(guild_member::Column::GuildId.eq(guild_id.0))
        .one(&state.db)
        .await?;

    if let Some(m) = member {
        m.delete(&state.db).await?;
        cache_remove_member(state, guild_id, target_id).await;

        let event = ServerMessage::Guild(GuildServerEvents::MemberLeft {
            guild_id,
            user_id: target_id,
        });

        broadcast::to_guild(&state.redis.messages, &guild_id, &event).await?;
        broadcast::to_user(&state.redis.messages, &target_id, &event).await?;
    }

    Ok(())
}
