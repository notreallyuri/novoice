use crate::data::PresetId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct NotificationSettings {
    pub active: bool,
}

#[derive(Default, Clone, PartialEq, Debug, Copy, Serialize, Deserialize)]
pub enum ThemeSpacing {
    #[default]
    Default,
    Compact,
    Comfortable,
}

#[derive(Default, Clone, PartialEq, Debug, Copy, Serialize, Deserialize)]
pub enum ThemeRounding {
    #[default]
    Default,
    Comfortable,
    Full,
}

#[derive(Default, Clone, PartialEq, Debug, Copy, Serialize, Deserialize)]
pub enum ThemeDarkMode {
    Light,
    Dark,
    #[default]
    System,
}

#[derive(Default, Clone, PartialEq, Debug, Copy, Serialize, Deserialize)]
pub enum ThemeColor {
    #[default]
    Default,
    Havoc,
    Void,
}

#[derive(PartialEq, Serialize, Deserialize, Clone, Debug)]
pub struct UISettings {
    pub theme: ThemeColor,
    pub dark_mode: ThemeDarkMode,
    pub rounding: ThemeRounding,
    pub spacing: ThemeSpacing,
}

#[derive(PartialEq, Serialize, Deserialize, Clone, Debug)]
pub struct PresencePreset {
    pub id: PresetId,
    pub label: String,
    pub icon: PresenceIcon,
    pub timer: PresenceTimer,
    pub kind: PresenceKind,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PresenceTimer {
    Elapsed,
    Countdown { seconds: u64 },
    Off,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PresenceKind {
    Fixed,
    AppLinked { process_name: String },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PresenceIcon {
    CustomUpload { path_url: Option<String> },
    Emoji { value: String },
    AppIcon { process_name: String },
}
