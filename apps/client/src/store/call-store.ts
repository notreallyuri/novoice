import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import { create } from "zustand";

interface CallStateData {
  channelId: string | null;
  channelName: string | null;
  isActive: boolean;
  isDeafened: boolean;
  isMuted: boolean;
}

interface CallStore extends CallStateData {
  leaveCall: () => Promise<void>;
  setupListeners: () => Promise<UnlistenFn>;
  toggleDeafen: () => Promise<void>;
  toggleMute: () => Promise<void>;
}

export const useCallStore = create<CallStore>((set) => ({
  isActive: false,
  channelId: null,
  channelName: null,
  isMuted: false,
  isDeafened: false,

  setupListeners: async () => {
    const unlisten = await listen<CallStateData>("call_state", (event) => {
      set(event.payload);
    });
    return unlisten;
  },

  toggleMute: async () => {
    await invoke("toggle_mute").catch(console.error);
  },
  toggleDeafen: async () => {
    await invoke("toggle_deafen").catch(console.error);
  },
  leaveCall: async () => {
    await invoke("close_call").catch(console.error);
  },
}));
