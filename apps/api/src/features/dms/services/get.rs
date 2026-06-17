use crate::core::{error::AppError, mappers::FromDomain, state::SharedState};
use entity::{dm_channel_member, user};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use shared::data::{
    ChannelId, UserId,
    channel::prelude::DMChannel,
    user::{Status, UserPresence, UserProfile, UserPublic},
};
use uuid::Uuid;

pub async fn get_open_dms(
    state: &SharedState,
    user_id: UserId,
) -> Result<Vec<DMChannel>, AppError> {
    let my_open_memberships = dm_channel_member::Entity::find()
        .filter(dm_channel_member::Column::UserId.eq(user_id.0))
        .filter(dm_channel_member::Column::IsOpen.eq(true))
        .all(&state.db)
        .await?;

    if my_open_memberships.is_empty() {
        return Ok(Vec::new());
    }

    let active_channel_ids: Vec<Uuid> = my_open_memberships.iter().map(|m| m.channel_id).collect();

    let other_members = dm_channel_member::Entity::find()
        .filter(dm_channel_member::Column::ChannelId.is_in(active_channel_ids.clone()))
        .filter(dm_channel_member::Column::UserId.ne(user_id.0))
        .find_also_related(user::Entity)
        .all(&state.db)
        .await?;

    let recipient_ids: Vec<Uuid> = other_members.iter().map(|(m, _)| m.user_id).collect();
    let presence_map =
        crate::core::presence::get_bulk(&state.redis.presence, &recipient_ids).await?;

    let channels = my_open_memberships
        .into_iter()
        .filter_map(|my_membership| {
            let recipient_data = other_members
                .iter()
                .find(|(m, _)| m.channel_id == my_membership.channel_id)?;

            let target_user_model = recipient_data.1.as_ref()?;
            let target_id = target_user_model.id;

            let presence = presence_map
                .get(&target_id)
                .cloned()
                .unwrap_or(UserPresence {
                    status: Status::Offline,
                    preset: None,
                });

            Some(DMChannel {
                id: ChannelId(my_membership.channel_id),
                recipients: vec![UserPublic {
                    id: UserId(target_id),
                    profile: UserProfile::from_domain(target_user_model.clone()),
                    presence,
                }],
                is_open: true,
            })
        })
        .collect();

    Ok(channels)
}
