import {
  Bell,
  type LucideIcon,
  Palette,
  RectangleEllipsis,
  User,
} from "lucide-react";
import { useState } from "react";

export type SettingsRoutes =
  | "Account"
  | "Profile"
  | "Notifications"
  | "Appearance"
  | "Advanced";
export type SettingsRoutesCategories =
  | "hidden"
  | "User Settings"
  | "App Settings";

export interface SidebarItem {
  description?: string;
  icon: LucideIcon;
  name: SettingsRoutes;
}
export type SidebarData = Record<SettingsRoutesCategories, SidebarItem[]>;

export function useSidebarData() {
  const sidebarData: SidebarData = {
    hidden: [
      {
        name: "Profile",
        icon: User,
        description: "Manage your profile settings and preferences",
      },
    ],
    "User Settings": [
      {
        name: "Account",
        description: "Manage your account settings and preferences",
        icon: User,
      },
      {
        name: "Appearance",
        description: "Customize the look and feel of the application",
        icon: Palette,
      },
      {
        name: "Notifications",
        description: "Configure your notification preferences",
        icon: Bell,
      },
    ],
    "App Settings": [
      {
        name: "Advanced",
        icon: RectangleEllipsis,
      },
    ],
  };

  const [activeRoute, setActiveRoute] = useState<SidebarItem>(
    sidebarData["User Settings"][0]
  );

  return {
    sidebarData,
    activeRoute,
    setActiveRoute,
  };
}
