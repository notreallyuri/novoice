export type PanelType = "channel" | "dm" | "settings";

export interface ViewPanel {
  id: string;
  targetId: string;
  title: string;
  type: PanelType;
}
