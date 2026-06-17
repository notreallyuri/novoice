use crate::core::{
    error::AppError,
    mappers::{FromDomain, IntoDomain},
    presence,
    state::SharedState,
};
use entity::{
    category, category_override, channel, channel_override, guild, guild_member, guild_member_role,
    relationship::{self, DbRelationshipStatus},
    role, user,
};
use sea_orm::{ColumnTrait, Condition, EntityTrait, ModelTrait, QueryFilter};
use shared::{
    data::{
        ChannelId, GuildId, RoleId, UserId,
        guild::{Guild, GuildIdentity, GuildMember, GuildProfile, GuildSummary, Role},
        permissions::Permissions,
        relationship::UserRelationship,
        user::{Status, User, UserAccount, UserPresence, UserProfile, UserPublic, UserSettings},
        user_settings::NotificationSettings,
    },
    dtos::user::GetMeResponse,
};
use std::collections::HashMap;
use uuid::Uuid;

pub async fn get_me(state: &SharedState, user_id: UserId) -> Result<GetMeResponse, AppError> {
    let user_model = user::Entity::find_by_id(user_id.0)
        .one(&state.db)
        .await?
        .ok_or(AppError::NotFound)?;

    let (account_res, settings_res, presets_res, guilds_res, relationships_res) = tokio::try_join!(
        user_model
            .find_related(entity::user_account::Entity)
            .one(&state.db),
        user_model
            .find_related(entity::user_settings::Entity)
            .one(&state.db),
        user_model
            .find_related(entity::presence_preset::Entity)
            .all(&state.db),
        user_model
            .find_linked(entity::user::UserJoinedGuilds)
            .all(&state.db),
        relationship::Entity::find()
            .filter(
                sea_orm::Condition::any()
                    .add(relationship::Column::UserId.eq(user_id.0))
                    .add(relationship::Column::TargetId.eq(user_id.0))
            )
            .all(&state.db)
    )?;

    let account_model = account_res.ok_or(AppError::Internal("Account missing".into()))?;
    let settings_model = settings_res.ok_or(AppError::Internal("Settings missing".into()))?;
    let notification_active = settings_model.notification_active;

    let user = User {
        id: user_id,
        profile: UserProfile::from_domain(user_model),
        account: UserAccount {
            email: account_model.email,
            verified: account_model.verified,
        },
        settings: UserSettings {
            ui: settings_model.into_domain(),
            notifications: NotificationSettings {
                active: notification_active,
            },
            presence_presets: presets_res.into_iter().map(|p| p.into_domain()).collect(),
        },
        presence: UserPresence {
            status: Status::Online,
            preset: None,
        },
    };

    let guilds = guilds_res
        .into_iter()
        .map(|g| GuildSummary {
            id: GuildId(g.id),
            profile: GuildProfile {
                owner_id: UserId(g.owner_id),
                banner_url: g.banner_url,
                icon_url: g.icon_url,
                name: g.name,
                default_channel_id: g.default_channel_id.map(shared::data::ChannelId),
            },
        })
        .collect();

    let target_user_ids: Vec<uuid::Uuid> = relationships_res
        .iter()
        .map(|r| {
            if r.user_id == user_id.0 {
                r.target_id
            } else {
                r.user_id
            }
        })
        .collect();

    let mut target_users_map: std::collections::HashMap<uuid::Uuid, entity::user::Model> =
        if target_user_ids.is_empty() {
            std::collections::HashMap::new()
        } else {
            entity::user::Entity::find()
                .filter(entity::user::Column::Id.is_in(target_user_ids.clone()))
                .all(&state.db)
                .await?
                .into_iter()
                .map(|u| (u.id, u))
                .collect()
        };

    let mut presence_request_ids = vec![user_id.0];
    presence_request_ids.extend(&target_user_ids);

    let presence_map =
        crate::core::presence::get_bulk(&state.redis.presence, &presence_request_ids).await?;

    let relationships = relationships_res
        .into_iter()
        .filter_map(|rel| {
            if rel.target_id == user_id.0 && rel.status == DbRelationshipStatus::Blocked {
                return None;
            }

            let target_id = if rel.user_id == user_id.0 {
                rel.target_id
            } else {
                rel.user_id
            };

            let target_user = target_users_map.remove(&target_id)?;

            let mut domain_rel: UserRelationship = (rel, target_user, user_id.0).into_domain();

            if let Some(real_presence) = presence_map.get(&target_id) {
                domain_rel.user.presence = real_presence.clone();
            }

            Some(domain_rel)
        })
        .collect();

    Ok(GetMeResponse {
        user,
        guilds,
        relationships,
    })
}

pub async fn get_guild(
    state: &SharedState,
    guild_id: GuildId,
    user_id: UserId,
) -> Result<Guild, AppError> {
    let (guild, current_member) = tokio::try_join!(
        guild::Entity::find_by_id(guild_id.0).one(&state.db),
        guild_member::Entity::find()
            .filter(guild_member::Column::UserId.eq(user_id.0))
            .filter(guild_member::Column::GuildId.eq(guild_id.0))
            .one(&state.db)
    )?;

    let guild = guild.ok_or(AppError::NotFound)?;
    let current_member = current_member.ok_or(AppError::Unauthorized)?;

    let current_member_roles = guild_member_role::Entity::find()
        .filter(guild_member_role::Column::GuildMemberId.eq(current_member.id))
        .find_also_related(role::Entity)
        .all(&state.db)
        .await?;

    let mut current_role_ids = Vec::new();
    let mut current_role_permissions = Vec::new();

    for (mr, role_opt) in current_member_roles {
        current_role_ids.push(mr.role_id);
        if let Some(r) = role_opt {
            current_role_permissions.push(r.permissions);
        }
    }

    let base_permissions = Permissions::compute_base_permissions(&current_role_permissions);
    let is_admin = base_permissions.contains(Permissions::ADMINISTRATOR);

    let (categories, channels) = tokio::try_join!(
        category::Entity::find()
            .filter(category::Column::GuildId.eq(guild_id.0))
            .all(&state.db),
        channel::Entity::find()
            .filter(channel::Column::GuildId.eq(guild_id.0))
            .all(&state.db)
    )?;

    let cat_ids: Vec<Uuid> = categories.iter().map(|c| c.id).collect();
    let chan_ids: Vec<Uuid> = channels.iter().map(|c| c.id).collect();

    let (cat_overrides, chan_overrides, all_members, all_roles, guild_roles_db) = tokio::try_join!(
        category_override::Entity::find()
            .filter(category_override::Column::CategoryId.is_in(cat_ids))
            .filter(
                Condition::any()
                    .add(
                        Condition::all()
                            .add(
                                category_override::Column::TargetType
                                    .eq(category_override::OverrideTargetType::Member)
                            )
                            .add(category_override::Column::TargetId.eq(user_id.0))
                    )
                    .add(
                        Condition::all()
                            .add(
                                category_override::Column::TargetType
                                    .eq(category_override::OverrideTargetType::Role)
                            )
                            .add(
                                category_override::Column::TargetId.is_in(current_role_ids.clone())
                            )
                    )
            )
            .all(&state.db),
        channel_override::Entity::find()
            .filter(channel_override::Column::ChannelId.is_in(chan_ids))
            .filter(
                Condition::any()
                    .add(
                        sea_orm::Condition::all()
                            .add(
                                channel_override::Column::TargetType
                                    .eq(channel_override::OverrideTargetType::Member)
                            )
                            .add(channel_override::Column::TargetId.eq(user_id.0))
                    )
                    .add(
                        sea_orm::Condition::all()
                            .add(
                                channel_override::Column::TargetType
                                    .eq(channel_override::OverrideTargetType::Role)
                            )
                            .add(channel_override::Column::TargetId.is_in(current_role_ids))
                    )
            )
            .all(&state.db),
        guild_member::Entity::find()
            .filter(guild_member::Column::GuildId.eq(guild_id.0))
            .find_also_related(user::Entity)
            .all(&state.db),
        guild_member_role::Entity::find()
            .inner_join(guild_member::Entity)
            .filter(guild_member::Column::GuildId.eq(guild_id.0))
            .find_also_related(role::Entity)
            .all(&state.db),
        role::Entity::find()
            .filter(role::Column::GuildId.eq(guild_id.0))
            .all(&state.db)
    )?;

    let user_ids: Vec<Uuid> = all_members.iter().map(|(m, _)| m.user_id).collect();
    let presence_map = presence::get_bulk(&state.redis.presence, &user_ids).await?;

    let mut cat_overrides_map: HashMap<Uuid, Vec<category_override::Model>> = HashMap::new();

    for ov in cat_overrides {
        cat_overrides_map
            .entry(ov.category_id)
            .or_default()
            .push(ov);
    }

    let mut chan_overrides_map: HashMap<Uuid, Vec<channel_override::Model>> = HashMap::new();

    for ov in chan_overrides {
        chan_overrides_map
            .entry(ov.channel_id)
            .or_default()
            .push(ov);
    }

    let visible_channels = channels.into_iter().filter_map(|chan| {
        if is_admin {
            return Some(chan);
        }

        let mut perms = base_permissions;

        if let Some(cat_id) = chan.category_id
            && let Some(ovs) = cat_overrides_map.get(&cat_id)
        {
            let mut role_allow = 0;
            let mut role_deny = 0;
            let mut member_allow = 0;
            let mut member_deny = 0;

            for ov in ovs {
                match ov.target_type {
                    category_override::OverrideTargetType::Role => {
                        role_allow |= ov.allow_bits;
                        role_deny |= ov.deny_bits;
                    }
                    category_override::OverrideTargetType::Member => {
                        member_allow |= ov.allow_bits;
                        member_deny |= ov.deny_bits;
                    }
                }
            }

            perms = perms.apply_override(role_allow, role_deny);
            perms = perms.apply_override(member_allow, member_deny);
        }

        if let Some(ovs) = chan_overrides_map.get(&chan.id) {
            let mut role_allow = 0;
            let mut role_deny = 0;
            let mut member_allow = 0;
            let mut member_deny = 0;

            for ov in ovs {
                match ov.target_type {
                    channel_override::OverrideTargetType::Role => {
                        role_allow |= ov.allow_bits;
                        role_deny |= ov.deny_bits;
                    }
                    channel_override::OverrideTargetType::Member => {
                        member_allow |= ov.allow_bits;
                        member_deny |= ov.deny_bits;
                    }
                }
            }

            perms = perms.apply_override(role_allow, role_deny);
            perms = perms.apply_override(member_allow, member_deny);
        }

        if perms.contains(Permissions::VIEW_CHANNEL) {
            Some(chan)
        } else {
            None
        }
    });

    let guild_roles: Vec<Role> = guild_roles_db
        .into_iter()
        .map(|r| r.into_domain())
        .collect();

    let mut member_roles_map: HashMap<Uuid, Vec<RoleId>> = HashMap::new();

    for (gmr, role_opt) in all_roles {
        if let Some(r) = role_opt {
            member_roles_map
                .entry(gmr.guild_member_id)
                .or_default()
                .push(RoleId(r.id));
        }
    }

    let mapped_members = all_members
        .into_iter()
        .filter_map(|(member, user_opt)| {
            let user_model = user_opt?;
            let role_ids = member_roles_map.remove(&member.id).unwrap_or_default();

            let presence = presence_map
                .get(&member.user_id)
                .cloned()
                .unwrap_or(UserPresence {
                    status: Status::Offline,
                    preset: None,
                });

            let has_identity = member.identity_display_name.is_some()
                || member.identity_avatar.is_some()
                || member.identity_bio.is_some();

            Some(GuildMember {
                guild_id: GuildId(member.guild_id),
                user_id: UserId(member.user_id),
                roles: role_ids,
                joined_at: member.joined_at.to_rfc3339(),
                identity: has_identity.then(|| GuildIdentity {
                    display_name: member.identity_display_name.unwrap_or_default(),
                    avatar_url: member.identity_avatar,
                    bio: member.identity_bio,
                    show_global_username: member.identity_show_global_username,
                }),
                data: UserPublic {
                    id: UserId(user_model.id),
                    profile: UserProfile::from_domain(user_model),
                    presence,
                },
            })
        })
        .collect();

    Ok(Guild {
        id: GuildId(guild.id),
        profile: GuildProfile {
            owner_id: UserId(guild.owner_id),
            banner_url: guild.banner_url,
            icon_url: guild.icon_url,
            name: guild.name,
            default_channel_id: guild.default_channel_id.map(ChannelId),
        },
        roles: guild_roles,
        categories: categories.into_iter().map(|ct| ct.into_domain()).collect(),
        channels: visible_channels.map(|cn| cn.into_domain()).collect(),
        members: mapped_members,
    })
}
