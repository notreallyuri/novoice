import { z } from "zod";
import type { GuildSummary } from "./guild";
import type { UserRelationship } from "./relationship";
import type { User } from "./user";

// --- Auth ---

export const RegisterStepAccount = z.object({
  username: z
    .string()
    .min(3, "Username must be at least 3 characters.")
    .max(32, "Username cannot exceed 32 characters.")
    .regex(
      /^[a-zA-Z0-9_]+$/,
      "Username can only contain letters, numbers, and underscores."
    ),
  email: z.email("Please enter a valid email address."),
  password: z
    .string()
    .min(8, "Password must be at least 8 characters.")
    .max(128, "Password cannot exceed 128 characters."),
});
export type RegisterStepAccount = z.infer<typeof RegisterStepAccount>;

export const RegisterStepProfile = z.object({
  avatar_url: z.url("Must be a valid URL.").nullable(),
  banner_url: z.url("Must be a valid URL.").nullable(),
  display_name: z
    .string()
    .min(2, "Display name must be at least 2 characters.")
    .max(32, "Display name cannot exceed 32 characters."),
  bio: z.string().max(190, "Bio cannot exceed 190 characters.").nullable(),
});
export type RegisterStepProfile = z.infer<typeof RegisterStepProfile>;

export const RegisterRequest = z.object({
  account: RegisterStepAccount,
  profile: RegisterStepProfile,
});
export type RegisterRequest = z.infer<typeof RegisterRequest>;

export const LoginRequest = z.object({
  email: z.email("Please enter a valid email address."),
  password: z.string().min(1, "Password is required."),
});
export type LoginRequest = z.infer<typeof LoginRequest>;

export const AuthResponse = z.object({
  token: z.string(),
});
export type AuthResponse = z.infer<typeof AuthResponse>;

// --- Guild ---

export const CreateGuildRequest = z.object({
  name: z.string().min(6).max(32),
});
export type CreateGuildRequest = z.infer<typeof CreateGuildRequest>;

export const CreateInviteRequest = z.object({
  max_uses: z.number().nullable(),
  requires_approval: z.boolean().nullable(),
  expires_at: z.string().nullable(),
});
export type CreateInviteRequest = z.infer<typeof CreateInviteRequest>;

export const CreateGuildMemberRequest = z.object({
  user_id: z.uuid(),
  guild_id: z.uuid(),
  identity_display_name: z.string().nullable(),
  identity_avatar: z.string().nullable(),
  identity_bio: z.string().nullable(),
});
export type CreateGuildMemberRequest = z.infer<typeof CreateGuildMemberRequest>;

export const BanMemberRequest = z.object({
  reason: z.string().nullable(),
  expires_at: z.string().nullable(),
});
export type BanMemberRequest = z.infer<typeof BanMemberRequest>;

// --- Category ---

export const CreateCategoryRequest = z.object({
  name: z.string(),
});
export type CreateCategoryRequest = z.infer<typeof CreateCategoryRequest>;

export const UpdateCategoryRequest = z.object({
  name: z.string().nullable(),
  position: z.number().nullable(),
});
export type UpdateCategoryRequest = z.infer<typeof UpdateCategoryRequest>;

// --- Channel ---

export const CreateChannelKind = z.enum(["text", "voice", "canvas", "docs"]);
export type CreateChannelKind = z.infer<typeof CreateChannelKind>;

export const CreateChannelRequest = z.object({
  name: z.string(),
  kind: CreateChannelKind,
  category_id: z.uuid().nullable(),
});
export type CreateChannelRequest = z.infer<typeof CreateChannelRequest>;

export const UpdateChannelRequest = z.object({
  name: z.string().nullable(),
  position: z.number().nullable(),
  category_id: z.uuid().nullable().optional(),
  mode: z.enum(["chat", "board", "threads"]).nullable(),
  bitrate: z.number().nullable(),
  user_limit: z.number().nullable().optional(),
});
export type UpdateChannelRequest = z.infer<typeof UpdateChannelRequest>;

// --- DM ---

export const CreateDmRequest = z.object({
  target_user_id: z.uuid(),
});
export type CreateDmRequest = z.infer<typeof CreateDmRequest>;

// --- Message ---

export const MessageQueryParams = z.object({
  limit: z.number().nullable(),
  before: z.uuid().nullable(),
  thread_id: z.uuid().nullable(),
});
export type MessageQueryParams = z.infer<typeof MessageQueryParams>;

export const MessageSendRequest = z.object({
  content: z.string(),
  thread_id: z.uuid().nullable(),
});
export type MessageSendRequest = z.infer<typeof MessageSendRequest>;

export const MessageEditRequest = z.object({
  content: z.string(),
});
export type MessageEditRequest = z.infer<typeof MessageEditRequest>;

export const MessagePinRequest = z.object({
  label: z.string().nullable(),
});
export type MessagePinRequest = z.infer<typeof MessagePinRequest>;

// --- Context & Misc ---

export const GetMeResponse = z.object({
  user: z.custom<User>(),
  guilds: z.array(z.custom<GuildSummary>()),
  relationships: z.array(z.custom<UserRelationship>()),
});
export type GetMeResponse = z.infer<typeof GetMeResponse>;

export const GuildIdentityRequest = z.object({
  display_name: z.string().nullable(),
  avatar_url: z.string().nullable(),
  banner_url: z.string().nullable(),
  bio: z.string().nullable(),
  show_global_username: z.boolean(),
});
export type GuildIdentityRequest = z.infer<typeof GuildIdentityRequest>;

export const JoinGuildRequest = z.object({
  invite_code: z.string(),
  identity: GuildIdentityRequest.nullable(),
});
export type JoinGuildRequest = z.infer<typeof JoinGuildRequest>;
