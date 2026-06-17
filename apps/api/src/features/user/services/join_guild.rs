use crate::core::{error::AppError, state::SharedState};
use entity::{guild_ban, guild_member, guild_member_role, invite, role};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set, TransactionTrait};
use shared::{data::UserId, dtos::user::JoinGuildRequest};
use uuid::Uuid;

pub async fn join_guild(
    state: SharedState,
    user_id: UserId,
    payload: JoinGuildRequest,
) -> Result<(), AppError> {
    let txn = state.db.begin().await?;

    let invite = invite::Entity::find_by_id(payload.invite_code)
        .one(&txn)
        .await?
        .ok_or(AppError::NotFound)?;

    let now = chrono::Utc::now();

    if let Some(expires_at) = invite.expires_at
        && now > expires_at
    {
        return Err(AppError::BadRequest("Invite has expired".to_string()));
    }

    if invite.max_uses > 0 && invite.uses >= invite.max_uses {
        return Err(AppError::BadRequest(
            "Invite usage limit reached".to_string(),
        ));
    }

    let existing_member = guild_member::Entity::find()
        .filter(guild_member::Column::UserId.eq(user_id.0))
        .filter(guild_member::Column::GuildId.eq(invite.guild_id))
        .one(&txn)
        .await?;

    if existing_member.is_some() {
        return Err(AppError::Conflict(
            "You are already a member of this server".to_string(),
        ));
    }

    let existing_ban = guild_ban::Entity::find()
        .filter(guild_ban::Column::GuildId.eq(invite.guild_id))
        .filter(guild_ban::Column::UserId.eq(user_id.0))
        .one(&txn)
        .await?;

    if let Some(ban) = existing_ban {
        if let Some(expiration) = ban.expires_at {
            if now > expiration {
                sea_orm::ModelTrait::delete(ban, &txn).await?;
            } else {
                return Err(AppError::Forbidden(
                    "You are temporarily banned from this server".into(),
                ));
            }
        } else {
            return Err(AppError::Forbidden(
                "You are permanently banned from this server.".into(),
            ));
        }
    }

    let member_id = Uuid::new_v4();

    let mut new_member = guild_member::ActiveModel {
        id: Set(member_id),
        user_id: Set(user_id.0),
        guild_id: Set(invite.guild_id),
        joined_at: Set(now.into()),
        identity_show_global_username: Set(true),
        identity_display_name: Set(None),
        identity_avatar: Set(None),
        identity_banner: Set(None),
        identity_bio: Set(None),
    };

    if let Some(identity) = payload.identity {
        new_member.identity_show_global_username = Set(identity.show_global_username);
        new_member.identity_display_name = Set(identity.display_name);
        new_member.identity_avatar = Set(identity.avatar_url);
        new_member.identity_banner = Set(identity.banner_url);
        new_member.identity_bio = Set(identity.bio);
    }

    let everyone_role = role::Entity::find()
        .filter(role::Column::GuildId.eq(invite.guild_id))
        .filter(role::Column::Name.eq("@everyone"))
        .one(&txn)
        .await?
        .ok_or(AppError::Internal(
            "Guild missing @everyone role".to_string(),
        ))?;

    let new_member_role = guild_member_role::ActiveModel {
        role_id: Set(everyone_role.id),
        guild_member_id: Set(member_id),
    };

    let mut active_invite: invite::ActiveModel = invite.into();
    let current_uses = active_invite.uses.clone().unwrap();
    active_invite.uses = Set(current_uses + 1);

    new_member.insert(&txn).await?;
    new_member_role.insert(&txn).await?;
    active_invite.update(&txn).await?;

    txn.commit().await?;

    Ok(())
}
