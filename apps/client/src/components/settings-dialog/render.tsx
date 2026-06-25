import type { SettingsRoutes } from "./nav-hook";

interface Props {
  currentRoute: SettingsRoutes;
}

export function SettingsRender({ currentRoute }: Props) {
  switch (currentRoute) {
    case "Account":
      return <div>Account Settings</div>;
    case "Profile":
      return <div>Profile Settings</div>;
    case "Notifications":
      return <div>Notifications Settings</div>;
    case "Appearance":
      return <div>Appearance Settings</div>;
    case "Advanced":
      return <div>Advanced settings comming soon!</div>;
    default:
      return <div>Select a settings category</div>;
  }
}
