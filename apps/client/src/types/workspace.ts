export type PanelType = "channel" | "dm" | "settings";

export interface ViewPanel {
  id: string;
  type: PanelType;
  targetId: string;
  title: string;
}
