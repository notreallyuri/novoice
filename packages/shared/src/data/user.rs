use super::{
    UserId,
    user_settings::{NotificationSettings, PresencePreset, UISettings},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Status {
    Online,
    Busy,
    Away,
    Invisible,
    Offline,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: UserId,
    pub account: UserAccount,
    pub profile: UserProfile,
    pub settings: UserSettings,
    pub presence: UserPresence,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserPublic {
    pub id: UserId,
    #[serde(flatten)]
    pub profile: UserProfile,
    pub presence: UserPresence,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserAccount {
    pub email: String,
    pub verified: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserProfile {
    pub username: String,
    pub display_name: String,
    pub avatar_url: Option<String>,
    pub banner_url: Option<String>,
    pub profile_color: Option<String>,
    pub bio: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserSettings {
    pub ui: UISettings,
    pub notifications: NotificationSettings,
    pub presence_presets: Vec<PresencePreset>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserPresence {
    pub status: Status,
    pub preset: Option<PresencePreset>,
}
