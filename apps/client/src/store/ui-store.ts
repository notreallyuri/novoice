import { create } from "zustand";

export type SidebarView = null | "members" | "threads" | "pins" | "profile";

interface UIState {
  activeGuildId: string | null;
  createChannelOpen: boolean;
  createGuildOpen: boolean;
  leftSidebarOpen: boolean;
  rightSidebarOpen: boolean;
  rightSidebarView: SidebarView;
  setActiveGuild: (id: string | null) => void;
  setCreateChannelOpen: (open: boolean) => void;
  setCreateGuildOpen: (open: boolean) => void;
  setRightSidebarView: (view: SidebarView) => void;

  setSettingsOpen: (open: boolean) => void;

  settingsOpen: boolean;

  toggleLeftSidebar: () => void;
  toggleRightSidebar: () => void;
}

export const useUIStore = create<UIState>((set) => ({
  leftSidebarOpen: true,
  rightSidebarOpen: false,

  settingsOpen: false,
  createChannelOpen: false,
  createGuildOpen: false,

  activeGuildId: null,
  rightSidebarView: null,

  toggleLeftSidebar: () =>
    set((state) => ({ leftSidebarOpen: !state.leftSidebarOpen })),
  toggleRightSidebar: () =>
    set((state) => ({ rightSidebarOpen: !state.rightSidebarOpen })),
  setRightSidebarView: (view) => set(() => ({ rightSidebarView: view })),
  setActiveGuild: (id) => set(() => ({ activeGuildId: id })),

  setSettingsOpen: (open) => set(() => ({ settingsOpen: open })),
  setCreateGuildOpen: (open) => set(() => ({ createGuildOpen: open })),
  setCreateChannelOpen: (open) => set(() => ({ createChannelOpen: open })),
}));
