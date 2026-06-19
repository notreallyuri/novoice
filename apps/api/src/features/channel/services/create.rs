use crate::core::{
    audit::log_action, broadcast, error::AppError, guards::verify_permission, state::SharedState,
};
use entity::channel::{self, DbChannelKind, DbChannelMode};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, QuerySelect, Set};
use shared::{
    data::{
        ChannelId, GuildId, UserId,
        audit_log::AuditActionType,
        channel::{
            Channel,
            prelude::{CanvasChannel, ChannelMode, DocsChannel, TextChannel, VoiceChannel},
        },
        permissions::Permissions,
    },
    dtos::channel::{CreateChannelKind, CreateChannelRequest},
    ws::{ServerMessage, guild::GuildServerEvents},
};
use uuid::Uuid;

pub async fn create(
    state: &SharedState,
    user_id: UserId,
    guild_id: GuildId,
    req: CreateChannelRequest,
) -> Result<Channel, AppError> {
    verify_permission(&state.db, user_id, guild_id, Permissions::MANAGE_CHANNELS).await?;

    let position_query = channel::Entity::find().filter(channel::Column::GuildId.eq(guild_id.0));

    let position_query = match req.category_id {
        Some(cat_id) => position_query.filter(channel::Column::CategoryId.eq(cat_id.0)),
        None => position_query.filter(channel::Column::CategoryId.is_null()),
    };

    let highest_position: Option<Option<i32>> = position_query
        .select_only()
        .expr(channel::Column::Position.max())
        .into_tuple()
        .one(&state.db)
        .await?;

    let next_position = highest_position.flatten().unwrap_or(-1) + 1;
    let channel_id = Uuid::new_v4();

    let (db_kind, db_mode, db_bitrate) = match req.kind {
        CreateChannelKind::Text => (DbChannelKind::Text, Some(DbChannelMode::Chat), None),
        CreateChannelKind::Voice => (DbChannelKind::Voice, None, Some(64_000)),
        CreateChannelKind::Canvas => (DbChannelKind::Canvas, None, None),
        CreateChannelKind::Docs => (DbChannelKind::Docs, None, None),
    };

    let new_channel = channel::ActiveModel {
        id: Set(channel_id),
        guild_id: Set(guild_id.0),
        category_id: Set(req.category_id.map(|id| id.0)),
        name: Set(req.name.clone()),
        position: Set(next_position),
        kind: Set(db_kind),
        mode: Set(db_mode),
        bitrate: Set(db_bitrate),
        user_limit: Set(None),
    };

    new_channel.insert(&state.db).await?;

    let _ = log_action(
        &state.db,
        guild_id,
        user_id,
        AuditActionType::ChannelCreate,
        Some(channel_id),
        None,
        Some(serde_json::json!({
            "name": req.name,
            "kind": req.kind,
        })),
    )
    .await;

    let channel_dto = match req.kind {
        CreateChannelKind::Text => Channel::Text(TextChannel {
            id: ChannelId(channel_id),
            guild_id,
            category_id: req.category_id,
            name: req.name,
            position: next_position,
            mode: ChannelMode::Chat,
        }),
        CreateChannelKind::Voice => Channel::Voice(VoiceChannel {
            id: ChannelId(channel_id),
            guild_id,
            category_id: req.category_id,
            name: req.name,
            position: next_position,
            user_limit: None,
            bitrate: 64_000,
            participants: vec![],
        }),
        CreateChannelKind::Canvas => Channel::Canvas(CanvasChannel {
            id: ChannelId(channel_id),
            guild_id,
            category_id: req.category_id,
            name: req.name,
            position: next_position,
        }),
        CreateChannelKind::Docs => Channel::Docs(DocsChannel {
            id: ChannelId(channel_id),
            guild_id,
            category_id: req.category_id,
            name: req.name,
            position: next_position,
        }),
    };

    let event = ServerMessage::Guild(GuildServerEvents::ChannelCreated {
        guild_id,
        channel: Box::new(channel_dto.clone()),
    });

    broadcast::to_guild(&state.redis.messages, &guild_id, &event).await?;

    Ok(channel_dto)
}
