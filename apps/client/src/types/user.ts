import type { PresetId, UserId } from ".";

export type Status = "Online" | "Busy" | "Away" | "Invisible" | "Offline";

export interface NotificationSettings {
  active: boolean;
}

export type ThemeSpacing = "Default" | "Compact" | "Comfortable";
export type ThemeRounding = "Default" | "Comfortable" | "Full";
export type ThemeDarkMode = "Light" | "Dark" | "System";
export type ThemeColor = "Default" | "Havoc" | "Void";

export interface UISettings {
  dark_mode: ThemeDarkMode;
  rounding: ThemeRounding;
  spacing: ThemeSpacing;
  theme: ThemeColor;
}

export type PresenceTimer =
  | "Elapsed"
  | { Countdown: { seconds: number } }
  | "Off";

export type PresenceKind = "Fixed" | { AppLinked: { process_name: string } };

export type PresenceIcon =
  | { CustomUpload: { path_url: string | null } }
  | { Emoji: { value: string } }
  | { AppIcon: { process_name: string } };

export interface PresencePreset {
  icon: PresenceIcon;
  id: PresetId;
  kind: PresenceKind;
  label: string;
  timer: PresenceTimer;
}

export interface UserAccount {
  email: string;
  verified: boolean;
}

export interface UserProfile {
  avatar_url: string | null;
  banner_url: string | null;
  bio: string | null;
  display_name: string;
  profile_color: string | null;
  username: string;
}

export interface UserSettings {
  notifications: NotificationSettings;
  presence_presets: PresencePreset[];
  ui: UISettings;
}

export interface UserPresence {
  preset: PresencePreset | null;
  status: Status;
}

export interface User {
  account: UserAccount;
  id: UserId;
  presence: UserPresence;
  profile: UserProfile;
  settings: UserSettings;
}

export type UserPublic = {
  id: UserId;
  presence: UserPresence;
} & UserProfile;
