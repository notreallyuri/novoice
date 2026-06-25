import type { ChannelId, GuildId, LogId, RoleId, UserId } from ".";
import type { Channel, ChannelCategory } from "./channel";
import type { UserPublic } from "./user";

export type Permissions = number;

export interface GuildIdentity {
  avatar_url: string | null;
  bio: string | null;
  display_name: string;
  show_global_username: boolean;
}

export interface Role {
  color: number | null;
  hoist: boolean;
  id: RoleId;
  name: string;
  permissions: Permissions;
  position: number;
}

export interface GuildMember {
  data: UserPublic;
  guild_id: GuildId;
  identity: GuildIdentity | null;
  joined_at: string;
  roles: RoleId[];
  user_id: UserId;
}

export interface GuildProfile {
  banner_url: string | null;
  default_channel_id: ChannelId | null;
  icon_url: string | null;
  name: string;
  owner_id: UserId;
}

export type GuildSummary = {
  id: GuildId;
} & GuildProfile;

export type Guild = {
  id: GuildId;
  roles: Role[];
  members: GuildMember[];
  categories: ChannelCategory[];
  channels: Channel[];
} & GuildProfile;

export enum AuditActionType {
  GuildUpdate = 1,
  ChannelCreate = 10,
  ChannelUpdate = 11,
  ChannelDelete = 12,
  CategoryCreate = 13,
  CategoryUpdate = 14,
  CategoryDelete = 15,
  MemberKick = 20,
  MemberBanAdd = 22,
  MemberBanRemove = 23,
  MemberUpdate = 24,
  RoleCreate = 30,
  RoleUpdate = 31,
  RoleDelete = 32,
  MessageDelete = 72,
}

export interface AuditLogEntry {
  action_type: AuditActionType;
  actor_id: UserId;
  changes: Record<string, unknown> | null;
  created_at: string;
  guild_id: GuildId;
  id: LogId;
  reason: string | null;
  target_id: string | null;
}
