use bitflags::bitflags;
use serde::{Deserialize, Serialize};

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(transparent)]
    pub struct Permissions: i64 {
        const CREATE_INVITE              = 1 << 0;
        const KICK_MEMBERS               = 1 << 1;
        const BAN_MEMBERS                = 1 << 2;
        const ADMINISTRATOR              = 1 << 3;
        const MANAGE_CHANNELS            = 1 << 4;
        const MANAGE_GUILD               = 1 << 5;
        const MANAGE_ROLES               = 1 << 6;
        const VIEW_AUDIT_LOG             = 1 << 7;
        const MANAGE_WEBHOOKS            = 1 << 8;
        const MANAGE_EMOJIS_AND_STICKERS = 1 << 9;

        const VIEW_CHANNEL               = 1 << 10;
        const SEND_MESSAGES              = 1 << 11;
        const MANAGE_MESSAGES            = 1 << 12;
        const EMBED_LINKS                = 1 << 13;
        const ATTACH_FILES               = 1 << 14;
        const READ_HISTORY               = 1 << 15;
        const ADD_REACTIONS              = 1 << 16;
        const USE_EXTERNAL_EMOJIS        = 1 << 17;
        const MENTION_EVERYONE           = 1 << 18;

        const CONNECT                    = 1 << 19;
        const SPEAK                      = 1 << 20;
        const MUTE_MEMBERS               = 1 << 21;
        const DEAFEN_MEMBERS             = 1 << 22;
        const MOVE_MEMBERS               = 1 << 23;
        const USE_VOICE_ACTIVITY         = 1 << 24;
        const PRIORITY_SPEAKER           = 1 << 25;
        const STREAM                     = 1 << 26;

        const CREATE_PUBLIC_THREADS      = 1 << 27;
        const CREATE_PRIVATE_THREADS     = 1 << 28;
        const MANAGE_THREADS             = 1 << 29;
        const SEND_MESSAGES_IN_THREADS   = 1 << 30;

        const CHANGE_NICKNAME            = 1 << 31;
        const MANAGE_NICKNAMES           = 1 << 32;
        const MODERATE_MEMBERS           = 1 << 33;
        const VIEW_GUILD_INSIGHTS        = 1 << 34;
        const MANAGE_EVENTS              = 1 << 35;

        const IMPLICIT_MODERATE_MEMBERS = Self::MODERATE_MEMBERS.bits()
            | Self::MANAGE_NICKNAMES.bits()
            | Self::KICK_MEMBERS.bits()
            | Self::BAN_MEMBERS.bits()
            | Self::VIEW_AUDIT_LOG.bits();

        const IMPLICIT_MANAGE_MESSAGES = Self::MANAGE_MESSAGES.bits()
            | Self::SEND_MESSAGES.bits()
            | Self::EMBED_LINKS.bits()
            | Self::ATTACH_FILES.bits()
            | Self::READ_HISTORY.bits()
            | Self::ADD_REACTIONS.bits()
            | Self::MANAGE_THREADS.bits();

        const IMPLICIT_MANAGE_CHANNELS = Self::MANAGE_CHANNELS.bits()
            | Self::VIEW_CHANNEL.bits()
            | Self::MANAGE_WEBHOOKS.bits()
            | Self::IMPLICIT_MANAGE_MESSAGES.bits()
            | Self::CONNECT.bits()
            | Self::SPEAK.bits()
            | Self::MUTE_MEMBERS.bits()
            | Self::DEAFEN_MEMBERS.bits()
            | Self::MOVE_MEMBERS.bits();
    }
}

impl Permissions {
    pub fn compute_base_permissions(role_permissions: &[i64]) -> Self {
        let mut total = Permissions::empty();

        for &role_perm in role_permissions {
            total |= Permissions::from_bits_truncate(role_perm);
        }

        if total.contains(Permissions::ADMINISTRATOR) {
            return Permissions::all();
        }

        if total.contains(Permissions::MANAGE_CHANNELS) {
            total |= Permissions::IMPLICIT_MANAGE_CHANNELS;
        }

        if total.contains(Permissions::MANAGE_MESSAGES) {
            total |= Permissions::IMPLICIT_MANAGE_MESSAGES;
        }

        if total.contains(Permissions::MODERATE_MEMBERS) {
            total |= Permissions::IMPLICIT_MODERATE_MEMBERS;
        }

        total
    }

    #[must_use]
    pub fn apply_override(mut self, allow_bits: i64, deny_bits: i64) -> Self {
        if self.contains(Permissions::ADMINISTRATOR) {
            return self;
        }

        let allow = Permissions::from_bits_truncate(allow_bits);
        let deny = Permissions::from_bits_truncate(deny_bits);

        self &= !deny;
        self |= allow;

        self
    }
}
