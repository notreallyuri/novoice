use entity::category;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, QuerySelect, Set};
use shared::{
    data::{
        CategoryId, GuildId, UserId, channel::prelude::ChannelCategory, permissions::Permissions,
    },
    dtos::category::CreateCategoryRequest,
    ws::{ServerMessage, guild::GuildServerEvents},
};
use uuid::Uuid;

use crate::core::{broadcast, error::AppError, guards::verify_permission, state::SharedState};

pub async fn create(
    state: &SharedState,
    user_id: UserId,
    guild_id: GuildId,
    req: CreateCategoryRequest,
) -> Result<ChannelCategory, AppError> {
    verify_permission(&state.db, user_id, guild_id, Permissions::MANAGE_CHANNELS).await?;

    let highest_position: Option<Option<i32>> = category::Entity::find()
        .filter(category::Column::GuildId.eq(guild_id.0))
        .select_only()
        .expr(category::Column::Position.max())
        .into_tuple()
        .one(&state.db)
        .await?;

    let next_position = highest_position.flatten().unwrap_or(-1) + 1;
    let category_id = Uuid::new_v4();

    let new_category = category::ActiveModel {
        id: Set(category_id),
        guild_id: Set(guild_id.0),
        name: Set(req.name.clone()),
        position: Set(next_position),
    };

    new_category.insert(&state.db).await?;

    let category_dto = ChannelCategory {
        id: CategoryId(category_id),
        guild_id,
        name: req.name,
        position: next_position,
    };

    let event = ServerMessage::Guild(GuildServerEvents::CategoryCreated {
        guild_id,
        category: Box::new(category_dto.clone()),
    });

    broadcast::to_guild(&state.redis.messages, &guild_id, &event).await?;

    Ok(category_dto)
}
