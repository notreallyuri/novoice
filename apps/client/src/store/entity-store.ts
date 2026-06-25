import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { create } from "zustand";
import type { ChannelId, GuildId, UserId } from "@/types";
import type { DMChannel, Message } from "@/types/channel";
import type { Guild } from "@/types/guild";
import type { UserRelationship } from "@/types/relationship";
import type { UserPublic } from "@/types/user";

interface EntityState {
  dmChannels: Record<ChannelId, DMChannel>;
  guilds: Record<GuildId, Guild>;
  hydrateEntities: () => Promise<void>;
  isHydrating: boolean;
  messages: Record<ChannelId, Message[]>;
  relationships: Record<UserId, UserRelationship>;
  setupListeners: () => Promise<() => void>;
  users: Record<UserId, UserPublic>;
}

export const useEntityStore = create<EntityState>((set) => ({
  guilds: {},
  dmChannels: {},
  relationships: {},
  users: {},
  messages: {},
  isHydrating: true,

  hydrateEntities: async () => {
    try {
      const data = await invoke<{
        guilds: Record<GuildId, Guild>;
        dmChannels: Record<ChannelId, DMChannel>;
        relationships: Record<UserId, UserRelationship>;
        users: Record<UserId, UserPublic>;
        messages: Record<ChannelId, Message[]>;
      }>("get_mock_entities");

      set({
        guilds: data.guilds || {},
        dmChannels: data.dmChannels || {},
        relationships: data.relationships || {},
        users: data.users || {},
        messages: data.messages || {},
        isHydrating: false,
      });
    } catch (error) {
      console.error("Failed to fetch entities:", error);
      set({ isHydrating: false });
    }
  },

  setupListeners: async () => {
    const unlistenGuilds = await listen<Guild>("guilds_event", (event) => {
      set((state) => ({
        guilds: { ...state.guilds, [event.payload.id]: event.payload },
      }));
    });

    const unlistenMessages = await listen<Message>("message_event", (event) => {
      set((state) => {
        const msg = event.payload;
        const channelMessages = state.messages[msg.channel_id] || [];

        return {
          messages: {
            ...state.messages,
            [msg.channel_id]: [...channelMessages, msg],
          },
        };
      });
    });

    const unlistenUsers = await listen<UserPublic>("users_event", (event) => {
      set((state) => ({
        users: { ...state.users, [event.payload.id]: event.payload },
      }));
    });

    const unlistenDMs = await listen<DMChannel>(
      "dm_channels_event",
      (event) => {
        set((state) => ({
          dmChannels: {
            ...state.dmChannels,
            [event.payload.id]: event.payload,
          },
        }));
      }
    );

    const unlistenRelationships = await listen<UserRelationship>(
      "relationships_event",
      (event) => {
        set((state) => ({
          relationships: {
            ...state.relationships,
            [event.payload.id]: event.payload,
          },
        }));
      }
    );

    return () => {
      unlistenGuilds();
      unlistenMessages();
      unlistenUsers();
      unlistenDMs();
      unlistenRelationships();
    };
  },
}));
