use crate::core::mappers::FromDomain;
use entity::{
    presence_preset::{DbPresenceIcon, DbPresenceKind, DbPresenceTimer},
    relationship::DbRelationshipStatus,
    user_settings::{DbThemeColor, DbThemeDarkMode, DbThemeRounding, DbThemeSpacing},
};
use shared::data::{
    relationship::RelationshipStatus,
    user_settings::{
        PresenceIcon, PresenceKind, PresenceTimer, ThemeColor, ThemeDarkMode, ThemeRounding,
        ThemeSpacing,
    },
};

impl FromDomain<DbThemeColor> for ThemeColor {
    fn from_domain(value: DbThemeColor) -> Self {
        match value {
            DbThemeColor::Default => ThemeColor::Default,
            DbThemeColor::Havoc => ThemeColor::Havoc,
            DbThemeColor::Void => ThemeColor::Void,
        }
    }
}

impl FromDomain<DbThemeDarkMode> for ThemeDarkMode {
    fn from_domain(value: DbThemeDarkMode) -> Self {
        match value {
            DbThemeDarkMode::System => ThemeDarkMode::System,
            DbThemeDarkMode::Light => ThemeDarkMode::Light,
            DbThemeDarkMode::Dark => ThemeDarkMode::Dark,
        }
    }
}

impl FromDomain<DbThemeRounding> for ThemeRounding {
    fn from_domain(value: DbThemeRounding) -> Self {
        match value {
            DbThemeRounding::Default => ThemeRounding::Default,
            DbThemeRounding::Comfortable => ThemeRounding::Comfortable,
            DbThemeRounding::Full => ThemeRounding::Full,
        }
    }
}

impl FromDomain<DbThemeSpacing> for ThemeSpacing {
    fn from_domain(value: DbThemeSpacing) -> Self {
        match value {
            DbThemeSpacing::Default => ThemeSpacing::Default,
            DbThemeSpacing::Comfortable => ThemeSpacing::Comfortable,
            DbThemeSpacing::Compact => ThemeSpacing::Compact,
        }
    }
}

impl FromDomain<DbRelationshipStatus> for RelationshipStatus {
    fn from_domain(value: DbRelationshipStatus) -> Self {
        match value {
            DbRelationshipStatus::None => RelationshipStatus::None,
            DbRelationshipStatus::Friend => RelationshipStatus::Friend,
            DbRelationshipStatus::Blocked => RelationshipStatus::Blocked,
            DbRelationshipStatus::PendingIncoming => RelationshipStatus::PendingIncoming,
            DbRelationshipStatus::PendingOutgoing => RelationshipStatus::PendingOutgoing,
        }
    }
}

impl FromDomain<(DbPresenceIcon, Option<String>)> for PresenceIcon {
    fn from_domain(value: (DbPresenceIcon, Option<String>)) -> Self {
        let (kind, val) = value;
        match kind {
            DbPresenceIcon::Emoji => PresenceIcon::Emoji {
                value: val.unwrap_or_default(),
            },
            DbPresenceIcon::App => PresenceIcon::AppIcon {
                process_name: val.unwrap_or_default(),
            },
            DbPresenceIcon::CustomUpload => PresenceIcon::CustomUpload { path_url: val },
        }
    }
}

impl FromDomain<(DbPresenceTimer, Option<i64>)> for PresenceTimer {
    fn from_domain(value: (DbPresenceTimer, Option<i64>)) -> Self {
        let (kind, val) = value;
        match kind {
            DbPresenceTimer::Elapsed => PresenceTimer::Elapsed,
            DbPresenceTimer::Countdown => PresenceTimer::Countdown {
                seconds: val.unwrap_or(0) as u64,
            },
            DbPresenceTimer::Off => PresenceTimer::Off,
        }
    }
}

impl FromDomain<(DbPresenceKind, Option<String>)> for PresenceKind {
    fn from_domain(value: (DbPresenceKind, Option<String>)) -> Self {
        let (kind, val) = value;
        match kind {
            DbPresenceKind::AppLinked => PresenceKind::AppLinked {
                process_name: val.unwrap_or_default(),
            },
            DbPresenceKind::Fixed => PresenceKind::Fixed,
        }
    }
}
