import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import { create } from "zustand";
import type { User } from "@/types/user";

interface AuthState {
  currentUser: User | null;
  isAuthenticating: boolean;
  logout: () => Promise<void>;
  setupListeners: () => Promise<UnlistenFn>;
}

export const useAuthStore = create<AuthState>((set) => ({
  currentUser: null,
  isAuthenticating: true,

  setupListeners: async () => {
    const unlisten = await listen<User>("current_user", (event) => {
      set({ currentUser: event.payload, isAuthenticating: false });
    });

    invoke("get_current_user").catch(console.error);

    return unlisten;
  },
  logout: async () => {
    await invoke("logout").catch(console.error);
    set({ currentUser: null });
  },
}));
