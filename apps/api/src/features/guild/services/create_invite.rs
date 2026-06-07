use crate::core::{error::AppError, guards::verify_permission, state::SharedState};
use entity::invite;
use sea_orm::{ActiveModelTrait, Set};
use shared::{
    data::{GuildId, UserId, permissions::Permissions},
    dtos::guild::CreateInviteRequest,
};
use uuid::Uuid;

pub async fn create_invite(
    state: &SharedState,
    user_id: UserId,
    guild_id: GuildId,
    payload: CreateInviteRequest,
) -> Result<String, AppError> {
    verify_permission(&state.db, user_id, guild_id, Permissions::CREATE_INVITE).await?;

    let invite_code = Uuid::new_v4();
    let now = chrono::Utc::now().into();

    let new_invite = invite::ActiveModel {
        invite_code: Set(invite_code),
        guild_id: Set(guild_id.0),
        creator_id: Set(user_id.0),
        max_uses: Set(payload.max_uses.unwrap_or(0)),
        uses: Set(0),
        requires_approval: Set(payload.requires_approval.unwrap_or(false)),
        expires_at: Set(payload.expires_at.map(|dt| dt.into())),
        created_at: Set(now),
    };

    new_invite.insert(&state.db).await?;

    Ok(invite_code.to_string())
}
