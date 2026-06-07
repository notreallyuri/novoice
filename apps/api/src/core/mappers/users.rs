use super::{FromDomain, IntoDomain};
use entity::{
    presence_preset::Model as PresencePresetModel, relationship::Model as RelationshipModel,
    user::Model as UserProfileModel, user_settings::Model as UserSettingsModel,
};
use shared::data::{
    PresetId, UserId,
    relationship::{RelationshipStatus, UserRelationship},
    user::{Status, UserPresence, UserProfile, UserPublic},
    user_settings::{PresencePreset, UISettings},
};

impl FromDomain<UserProfileModel> for UserProfile {
    fn from_domain(model: UserProfileModel) -> Self {
        UserProfile {
            username: model.username,
            display_name: model.display_name,
            profile_color: model.profile_color,
            avatar_url: model.avatar_url,
            banner_url: model.banner_url,
            bio: model.bio,
        }
    }
}

impl FromDomain<UserSettingsModel> for UISettings {
    fn from_domain(model: UserSettingsModel) -> Self {
        UISettings {
            theme: model.theme_color.into_domain(),
            dark_mode: model.theme_dark_mode.into_domain(),
            rounding: model.theme_rounding.into_domain(),
            spacing: model.theme_spacing.into_domain(),
        }
    }
}

impl FromDomain<PresencePresetModel> for PresencePreset {
    fn from_domain(model: PresencePresetModel) -> Self {
        PresencePreset {
            id: PresetId(model.id),
            label: model.label,
            icon: (model.icon_kind, model.icon_value).into_domain(),
            timer: (model.timer_kind, model.timer_seconds).into_domain(),
            kind: (model.preset_kind, model.process_name).into_domain(),
        }
    }
}

impl FromDomain<(RelationshipModel, UserProfileModel, uuid::Uuid)> for UserRelationship {
    fn from_domain(value: (RelationshipModel, UserProfileModel, uuid::Uuid)) -> Self {
        let (rel, target_user, current_user_id) = value;

        let mut status = rel.status.into_domain();
        if rel.target_id == current_user_id {
            status = match status {
                RelationshipStatus::PendingOutgoing => RelationshipStatus::PendingIncoming,
                RelationshipStatus::PendingIncoming => RelationshipStatus::PendingOutgoing,
                other => other,
            };
        }

        UserRelationship {
            id: UserId(target_user.id),
            user: UserPublic {
                id: UserId(target_user.id),
                profile: UserProfile::from_domain(target_user),
                presence: UserPresence {
                    status: Status::Offline,
                    preset: None,
                },
            },
            status,
            since: rel.since.and_utc().to_rfc3339(),
        }
    }
}
