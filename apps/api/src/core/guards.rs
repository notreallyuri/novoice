use crate::core::{error::AppError, mappers::FromDomain};
use entity::{
    category_override, channel, channel_override, guild_member, guild_member_role, role, user,
};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QuerySelect};
use shared::data::{
    ChannelId, GuildId, UserId,
    guild::{GuildIdentity, GuildMember},
    permissions::Permissions,
    user::{Status, UserPresence, UserProfile, UserPublic},
};

pub async fn verify_channel_permission(
    db: &DatabaseConnection,
    user_id: UserId,
    channel_id: ChannelId,
    permission: Permissions,
) -> Result<(), AppError> {
    let channel = channel::Entity::find_by_id(channel_id.0)
        .one(db)
        .await?
        .ok_or(AppError::NotFound)?;

    let guild_id = GuildId(channel.guild_id);

    let member = guild_member::Entity::find()
        .filter(guild_member::Column::UserId.eq(user_id.0))
        .filter(guild_member::Column::GuildId.eq(guild_id.0))
        .one(db)
        .await?
        .ok_or(AppError::NotFound)?;

    let member_roles = guild_member_role::Entity::find()
        .filter(guild_member_role::Column::GuildMemberId.eq(member.id))
        .find_also_related(role::Entity)
        .all(db)
        .await?;

    let role_ids: Vec<uuid::Uuid> = member_roles.iter().map(|(mr, _)| mr.role_id).collect();
    let role_permissions: Vec<i64> = member_roles
        .into_iter()
        .filter_map(|(_, r)| r.map(|role| role.permissions))
        .collect();

    let mut current_permissions = Permissions::compute_base_permissions(&role_permissions);

    if current_permissions.contains(Permissions::ADMINISTRATOR) {
        return Ok(());
    }

    if let Some(cat_id) = channel.category_id {
        let cat_overrides = category_override::Entity::find()
            .filter(category_override::Column::CategoryId.eq(cat_id))
            .filter(
                sea_orm::Condition::any()
                    .add(category_override::Column::TargetId.eq(user_id.0))
                    .add(category_override::Column::TargetId.is_in(role_ids.clone())),
            )
            .all(db)
            .await?;

        for ov in cat_overrides
            .iter()
            .filter(|o| o.target_type == category_override::OverrideTargetType::Role)
        {
            current_permissions = current_permissions.apply_override(ov.allow_bits, ov.deny_bits);
        }
        for ov in cat_overrides
            .iter()
            .filter(|o| o.target_type == category_override::OverrideTargetType::Member)
        {
            current_permissions = current_permissions.apply_override(ov.allow_bits, ov.deny_bits);
        }
    }

    let chan_overrides = channel_override::Entity::find()
        .filter(channel_override::Column::ChannelId.eq(channel_id.0))
        .filter(
            sea_orm::Condition::any()
                .add(channel_override::Column::TargetId.eq(user_id.0))
                .add(channel_override::Column::TargetId.is_in(role_ids)),
        )
        .all(db)
        .await?;

    for ov in chan_overrides
        .iter()
        .filter(|o| o.target_type == channel_override::OverrideTargetType::Role)
    {
        current_permissions = current_permissions.apply_override(ov.allow_bits, ov.deny_bits);
    }
    for ov in chan_overrides
        .iter()
        .filter(|o| o.target_type == channel_override::OverrideTargetType::Member)
    {
        current_permissions = current_permissions.apply_override(ov.allow_bits, ov.deny_bits);
    }

    if !current_permissions.contains(permission) {
        return Err(AppError::Forbidden(
            "Insufficient permissions in this channel".into(),
        ));
    }

    Ok(())
}

pub async fn verify_permission(
    db: &DatabaseConnection,
    user_id: UserId,
    guild_id: GuildId,
    permission: Permissions,
) -> Result<(), AppError> {
    let member_id = guild_member::Entity::find()
        .select_only()
        .column(guild_member::Column::Id)
        .filter(guild_member::Column::UserId.eq(user_id.0))
        .filter(guild_member::Column::GuildId.eq(guild_id.0))
        .into_tuple::<uuid::Uuid>()
        .one(db)
        .await?
        .ok_or(AppError::Forbidden("Not a member of this server".into()))?;

    let role_permissions: Vec<i64> = guild_member_role::Entity::find()
        .filter(guild_member_role::Column::GuildMemberId.eq(member_id))
        .find_also_related(role::Entity)
        .all(db)
        .await?
        .into_iter()
        .filter_map(|(_, r)| r.map(|role| role.permissions))
        .collect();

    if !Permissions::compute_base_permissions(&role_permissions).contains(permission) {
        return Err(AppError::Forbidden("Insufficient permissions".into()));
    }

    Ok(())
}

pub async fn get_member_with_permission(
    db: &DatabaseConnection,
    user_id: UserId,
    guild_id: GuildId,
    permission: Permissions,
) -> Result<GuildMember, AppError> {
    let (member, user_model) = guild_member::Entity::find()
        .filter(guild_member::Column::UserId.eq(user_id.0))
        .filter(guild_member::Column::GuildId.eq(guild_id.0))
        .find_also_related(user::Entity)
        .one(db)
        .await?
        .ok_or(AppError::Forbidden("Not a member of this server".into()))?;

    let user_model = user_model.ok_or(AppError::NotFound)?;

    let member_roles = guild_member_role::Entity::find()
        .filter(guild_member_role::Column::GuildMemberId.eq(member.id))
        .find_also_related(role::Entity)
        .all(db)
        .await?;

    let mut role_ids = Vec::new();
    let mut role_permissions = Vec::new();

    for (mr, role_opt) in member_roles {
        role_ids.push(shared::data::RoleId(mr.role_id));
        if let Some(r) = role_opt {
            role_permissions.push(r.permissions);
        }
    }

    if !Permissions::compute_base_permissions(&role_permissions).contains(permission) {
        return Err(AppError::Forbidden("Insufficient permissions".into()));
    }

    let has_custom_identity = member.identity_display_name.is_some()
        || member.identity_avatar.is_some()
        || member.identity_bio.is_some();

    let identity = has_custom_identity.then(|| GuildIdentity {
        display_name: member.identity_display_name.unwrap_or_default(),
        avatar_url: member.identity_avatar,
        bio: member.identity_bio,
        show_global_username: member.identity_show_global_username,
    });

    Ok(GuildMember {
        guild_id,
        user_id,
        roles: role_ids,
        joined_at: member.joined_at.to_rfc3339(),
        identity,
        data: UserPublic {
            id: user_id,
            profile: UserProfile::from_domain(user_model),
            presence: UserPresence {
                status: Status::Offline,
                preset: None,
            },
        },
    })
}
