import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import { create } from "zustand";

export interface SpaceData {
  id: string;
  targetId: string;
  title: string;
  type: string;
}

export type WorkspaceNode =
  | {
      nodeType: "group";
      id: string;
      direction: "horizontal" | "vertical";
      children: WorkspaceNode[];
    }
  | { nodeType: "panel"; data: SpaceData };

interface WorkspaceDataPayload {
  root: WorkspaceNode;
}

interface WorkspaceState {
  activePanelId: string | null;
  closePanel: (id: string) => Promise<void>;
  isBooting: boolean;
  openPanel: (panel: Omit<SpaceData, "id">) => Promise<void>;
  replacePanel: (
    targetId: string,
    panelToOpen: Omit<SpaceData, "id">
  ) => Promise<void>;
  root: WorkspaceNode | null;
  setActivePanel: (id: string | null) => void;
  setDirection: (direction: "horizontal" | "vertical") => Promise<void>;

  setupListeners: () => Promise<UnlistenFn>;
  splitPanel: (
    targetId: string,
    direction: "horizontal" | "vertical",
    panelToOpen: Omit<SpaceData, "id">
  ) => Promise<void>;
}

export const useWorkspaceStore = create<WorkspaceState>((set, get) => ({
  root: null,
  activePanelId: null,
  isBooting: true,

  setupListeners: async () => {
    const unlisten = await listen<WorkspaceDataPayload>(
      "spaces_state",
      (event) => {
        set({
          root: event.payload.root,
          isBooting: false,
        });
      }
    );

    invoke("request_initial_spaces").catch(console.error);

    return unlisten;
  },

  setActivePanel: (id) => set({ activePanelId: id }),

  closePanel: async (id) => {
    if (get().activePanelId === id) {
      set({ activePanelId: null });
    }
    await invoke("close_space", { id }).catch(console.error);
  },

  replacePanel: async (targetId, panel) => {
    const newId = crypto.randomUUID();
    await invoke("replace_space", {
      targetId,
      newPanelId: newId,
      targetPanelId: panel.targetId,
      title: panel.title,
      spaceType: panel.type,
    }).catch(console.error);

    set({ activePanelId: newId });
  },

  openPanel: async (panel) => {
    const newId = crypto.randomUUID();
    await invoke("open_space", {
      id: newId,
      targetId: panel.targetId,
      title: panel.title,
      spaceType: panel.type,
    }).catch(console.error);

    set({ activePanelId: newId });
  },

  setDirection: async (direction) => {
    await invoke("set_layout_direction", { direction }).catch(console.error);
  },

  splitPanel: async (targetId, direction, panel) => {
    const newPanelId = crypto.randomUUID();
    await invoke("split_space", {
      targetId,
      newGroupId: crypto.randomUUID(),
      newPanelId,
      targetPanelId: panel.targetId,
      title: panel.title,
      spaceType: panel.type,
      splitDirection: direction,
    }).catch(console.error);

    set({ activePanelId: newPanelId });
  },
}));
