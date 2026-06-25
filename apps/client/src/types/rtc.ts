import type { CategoryId, ChannelId, GuildId, MessageId, UserId } from ".";
import type {
  Channel,
  ChannelCategory,
  DMChannel,
  Message,
  PinnedMessage,
} from "./channel";
import type { Guild, GuildMember } from "./guild";
import type { UserRelationship } from "./relationship";
import type { User, UserPresence, UserPublic } from "./user";

export type GuildServerEvents =
  | { event: "Created"; guild: Guild }
  | { event: "Joined"; guild: Guild }
  | { event: "MemberJoined"; guild_id: GuildId; member: GuildMember }
  | { event: "MemberLeft"; guild_id: GuildId; user_id: UserId }
  | { event: "CategoryCreated"; guild_id: GuildId; category: ChannelCategory }
  | { event: "CategoryUpdated"; guild_id: GuildId; category: ChannelCategory }
  | { event: "CategoryDeleted"; guild_id: GuildId; category_id: CategoryId }
  | { event: "ChannelCreated"; guild_id: GuildId; channel: Channel }
  | { event: "ChannelUpdated"; guild_id: GuildId; channel: Channel }
  | { event: "ChannelDeleted"; guild_id: GuildId; channel_id: ChannelId }
  | { event: "Deleted"; guild_id: GuildId };

export type ChatClientEvents =
  | {
      event: "Send";
      channel_id: ChannelId;
      content: string;
      thread_id: MessageId | null;
    }
  | {
      event: "Edit";
      channel_id: ChannelId;
      message_id: MessageId;
      content: string;
    }
  | { event: "Delete"; channel_id: ChannelId; message_id: MessageId }
  | {
      event: "Pin";
      channel_id: ChannelId;
      message_id: MessageId;
      label: string | null;
    }
  | { event: "Unpin"; channel_id: ChannelId; message_id: MessageId };

export type ChatServerEvents =
  | { event: "Received"; message: Message }
  | { event: "Edited"; message: Message }
  | { event: "Deleted"; channel_id: ChannelId; message_id: MessageId }
  | { event: "Pinned"; channel_id: ChannelId; pin: PinnedMessage }
  | { event: "Unpinned"; channel_id: ChannelId; message_id: MessageId };

export type RtcClientEvents =
  | { event: "JoinVoice"; channel_id: ChannelId }
  | { event: "LeaveVoice" }
  | { event: "SdpOffer"; sdp: string }
  | { event: "SdpAnswer"; sdp: string }
  | { event: "IceCandidate"; candidate: string };

export type RtcServerEvents =
  | { event: "SdpOffer"; sdp: string }
  | { event: "SdpAnswer"; sdp: string }
  | { event: "IceCandidate"; candidate: string }
  | { event: "UserJoinedVoice"; user_id: UserId }
  | { event: "UserLeftVoice"; user_id: UserId };

export type UserServerEvents =
  | { event: "Ready" }
  | { event: "IdentityValidated"; user: User }
  | { event: "PresenceUpdate"; user: UserPublic }
  | { event: "RelationshipUpdate"; relationship: UserRelationship }
  | { event: "DirectMessageCreated"; channel: DMChannel };

export type ClientMessage =
  | { type: "Identify"; data: { token: string } }
  | { type: "Chat"; data: ChatClientEvents }
  | { type: "Rtc"; data: RtcClientEvents }
  | { type: "SetPresence"; data: { presence: UserPresence } };

export type ServerMessage =
  | { type: "User"; data: UserServerEvents }
  | { type: "Chat"; data: ChatServerEvents }
  | { type: "Guild"; data: GuildServerEvents }
  | { type: "Rtc"; data: RtcServerEvents }
  | { type: "Error"; data: { message: string } };
