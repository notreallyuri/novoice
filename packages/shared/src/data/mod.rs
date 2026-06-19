use derive_more::{Display, FromStr, Into};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod audit_log;
pub mod channel;
pub mod guild;
pub mod permissions;
pub mod relationship;
pub mod rtc;
pub mod user;
pub mod user_settings;

#[derive(
    Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Display, FromStr, Into,
)]
pub struct LogId(pub Uuid);
#[derive(
    Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Display, FromStr, Into,
)]
#[serde(transparent)]
pub struct GuildId(pub Uuid);
#[derive(
    Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Display, FromStr, Into,
)]
#[serde(transparent)]
pub struct RoleId(pub Uuid);
#[derive(
    Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Display, FromStr, Into,
)]
#[serde(transparent)]
pub struct UserId(pub Uuid);
#[derive(
    Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Display, FromStr, Into,
)]
#[serde(transparent)]
pub struct ChannelId(pub Uuid);
#[derive(
    Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Display, FromStr, Into,
)]
#[serde(transparent)]
pub struct MessageId(pub Uuid);
#[derive(
    Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Display, FromStr, Into,
)]
#[serde(transparent)]
pub struct CategoryId(pub Uuid);
#[derive(
    Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Display, FromStr, Into,
)]
#[serde(transparent)]
pub struct PresetId(pub Uuid);
