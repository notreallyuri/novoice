import type { CategoryId, ChannelId, GuildId, MessageId, UserId } from ".";
import type { UserPublic } from "./user";

export interface CanvasChannel {
  category_id: CategoryId | null;
  guild_id: GuildId;
  id: ChannelId;
  name: string;
  position: number;
}

export interface ChannelCategory {
  guild_id: GuildId;
  id: CategoryId;
  name: string;
  position: number;
}

export interface DMChannel {
  id: ChannelId;
  is_open: boolean;
  recipients: UserPublic[];
}

export interface DocsChannel {
  category_id: CategoryId | null;
  guild_id: GuildId;
  id: ChannelId;
  name: string;
  position: number;
}

export interface Message {
  author_id: UserId;
  channel_id: ChannelId;
  content: string;
  created_at: string;
  deleted: boolean;
  edited_at: string | null;
  id: MessageId;
  thread_id: MessageId | null;
}

export interface PinnedMessage {
  label: string | null;
  message_id: MessageId;
  pinned_at: string;
  pinned_by: UserId;
}

export type ChannelMode = "chat" | "board" | "threads";

export interface TextChannel {
  category_id: CategoryId | null;
  guild_id: GuildId;
  id: ChannelId;
  mode: ChannelMode;
  name: string;
  position: number;
}

export interface VoiceParticipant {
  deafened: boolean;
  muted: boolean;
  speaking: boolean;
  user_id: UserId;
}

export interface VoiceChannel {
  bitrate: number;
  category_id: CategoryId | null;
  guild_id: GuildId;
  id: ChannelId;
  name: string;
  participants: VoiceParticipant[];
  position: number;
  user_limit: number | null;
}

export type Channel =
  | ({ kind: "text" } & TextChannel)
  | ({ kind: "voice" } & VoiceChannel)
  | ({ kind: "docs" } & DocsChannel)
  | ({ kind: "canvas" } & CanvasChannel);
