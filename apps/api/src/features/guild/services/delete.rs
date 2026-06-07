use crate::core::{broadcast, error::AppError, state::SharedState};
use entity::{channel, guild};
use futures_util::StreamExt;
use sea_orm::{ColumnTrait, EntityTrait, ModelTrait, QueryFilter};
use shared::{
    data::{GuildId, UserId},
    ws::{ServerMessage, guild::GuildServerEvents},
};

pub async fn delete(
    state: &SharedState,
    guild_id: GuildId,
    user_id: UserId,
) -> Result<(), AppError> {
    let guild = guild::Entity::find_by_id(guild_id.0)
        .one(&state.db)
        .await?
        .ok_or(AppError::NotFound)?;

    if guild.owner_id != user_id.0 {
        return Err(AppError::Forbidden(
            "Only the guild owner can delete this guild".into(),
        ));
    }

    let channels = channel::Entity::find()
        .filter(channel::Column::GuildId.eq(guild_id.0))
        .all(&state.db)
        .await?;

    guild.delete(&state.db).await?;

    let event = ServerMessage::Guild(GuildServerEvents::Deleted { guild_id });
    broadcast::to_guild(&state.redis.presence, &guild_id, &event).await?;

    let state_clone = state.clone();

    tokio::spawn(async move {
        futures_util::stream::iter(channels)
            .for_each_concurrent(10, |c| {
                let state = state_clone.clone();

                async move {
                    let _ = state
                        .scylla
                        .session
                        .query_unpaged("DELETE FROM messages WHERE channel_id = ?", (c.id,))
                        .await;
                }
            })
            .await;
    });

    Ok(())
}
