use crate::core::{broadcast, error::AppError, guards::verify_permission, state::SharedState};
use entity::category;
use sea_orm::{ActiveModelTrait, EntityTrait, IntoActiveModel, Set, TryIntoModel};
use shared::{
    data::{
        CategoryId, GuildId, UserId, channel::category::ChannelCategory, permissions::Permissions,
    },
    dtos::category::UpdateCategoryRequest,
    ws::{ServerMessage, guild::GuildServerEvents},
};

pub async fn update(
    state: &SharedState,
    user_id: UserId,
    guild_id: GuildId,
    category_id: CategoryId,
    payload: UpdateCategoryRequest,
) -> Result<ChannelCategory, AppError> {
    verify_permission(&state.db, user_id, guild_id, Permissions::MANAGE_CHANNELS).await?;

    let category_model = category::Entity::find_by_id(category_id.0)
        .one(&state.db)
        .await?
        .ok_or(AppError::NotFound)?;

    if category_model.guild_id != guild_id.0 {
        return Err(AppError::Forbidden(
            "Category does not belong to this guild".into(),
        ));
    }

    let mut active_category = category_model.into_active_model();
    let mut has_changes = false;

    if let Some(new_name) = payload.name {
        active_category.name = Set(new_name);
        has_changes = true;
    }

    if let Some(new_pos) = payload.position {
        active_category.position = Set(new_pos);
        has_changes = true;
    }

    let updated_model = if has_changes {
        active_category.update(&state.db).await?
    } else {
        active_category.try_into_model().unwrap()
    };

    let category_dto = ChannelCategory {
        id: category_id,
        guild_id,
        name: updated_model.name,
        position: updated_model.position,
    };

    let event = ServerMessage::Guild(GuildServerEvents::CategoryUpdated {
        guild_id,
        category: Box::new(category_dto.clone()),
    });

    broadcast::to_guild(&state.redis.messages, &guild_id, &event).await?;

    Ok(category_dto)
}
