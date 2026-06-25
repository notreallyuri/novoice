import { Compass, Hash } from "lucide-react";
import { WorkspaceGrid } from "@/components/workspace/workspace-grid";
import { cn } from "@/lib/utils";
import { useUIStore } from "@/store/ui-store";
import { useWorkspaceStore } from "@/store/workspace-store";

function GuildSidebar() {
  return (
    <div className="z-10 flex h-full w-12 shrink-0 flex-col items-center gap-4 border-sidebar-border border-r bg-sidebar py-4">
      <button
        className="flex size-8 items-center justify-center rounded-full bg-primary/20 text-primary transition-all hover:bg-primary hover:text-primary-foreground"
        type="button"
      >
        <Compass size={18} />
      </button>
      <div className="h-0.5 w-8 rounded-full bg-border" />
      <button
        className="flex size-8 items-center justify-center rounded-full bg-muted font-bold transition-all hover:bg-primary/50"
        type="button"
      >
        N
      </button>
    </div>
  );
}

function ContextSidebar() {
  const openPanel = useWorkspaceStore((state) => state.openPanel);
  const replacePanel = useWorkspaceStore((state) => state.replacePanel);
  const activePanelId = useWorkspaceStore((state) => state.activePanelId);
  const root = useWorkspaceStore((state) => state.root);

  const handleChannelClick = (channel: {
    targetId: string;
    title: string;
    type: string;
  }) => {
    if (!root || (root.nodeType === "group" && root.children.length === 0)) {
      openPanel(channel);
      return;
    }

    if (activePanelId) {
      replacePanel(activePanelId, channel);
    } else {
      openPanel(channel);
    }
  };

  const MOCK_CHANNELS = [
    { targetId: "c-101", title: "general", type: "channel" },
    { targetId: "c-102", title: "rust-backend", type: "channel" },
    { targetId: "c-103", title: "frontend", type: "channel" },
    { targetId: "c-104", title: "voice-chat", type: "channel" },
  ];

  return (
    <div className="flex h-full w-full flex-col bg-sidebar">
      <div className="flex h-12 items-center border-sidebar-border border-b px-4 font-bold text-sidebar-foreground shadow-sm">
        NoVoice Server
      </div>
      <div className="flex flex-1 flex-col gap-1 overflow-y-auto p-2">
        {MOCK_CHANNELS.map((ch) => (
          <button
            className="flex w-full items-center gap-2 rounded-md px-2 py-1.5 text-left text-sidebar-foreground/70 text-sm transition-colors hover:bg-sidebar-accent hover:text-sidebar-accent-foreground"
            key={ch.targetId}
            onClick={() => handleChannelClick(ch)}
            type="button"
          >
            <Hash size={16} />
            {ch.title}
          </button>
        ))}
      </div>
    </div>
  );
}

function DetailsSidebar() {
  return (
    <div className="flex h-full w-full flex-col bg-sidebar">
      <div className="flex h-12 items-center border-sidebar-border border-b px-4 font-bold text-sidebar-foreground shadow-sm">
        Context Details
      </div>
      <div className="flex flex-1 flex-col gap-4 overflow-y-auto p-4">
        <div className="font-bold text-sidebar-foreground/50 text-xs uppercase tracking-widest">
          Online — 3
        </div>
        {/* Mock Members */}
        {[1, 2, 3].map((i) => (
          <div className="flex items-center gap-3" key={i}>
            <div className="size-8 rounded-full bg-muted" />
            <div className="flex flex-col">
              <span className="font-medium text-sm">User {i}</span>
              <span className="text-muted-foreground text-xs">
                Playing Rust
              </span>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}

export function AppShell() {
  const leftSidebarOpen = useUIStore((state) => state.leftSidebarOpen);
  const rightSidebarOpen = useUIStore((state) => state.rightSidebarOpen);

  return (
    <div className="flex h-full w-full overflow-hidden bg-background">
      <GuildSidebar />
      <div
        className={cn(
          "h-full shrink-0 overflow-hidden border-border border-r transition-all duration-300 ease-in-out",
          leftSidebarOpen ? "w-60 opacity-100" : "w-0 border-r-0 opacity-0"
        )}
      >
        <div className="h-full w-60">
          <ContextSidebar />
        </div>
      </div>

      <div className="relative h-full min-w-0 flex-1">
        <WorkspaceGrid />
      </div>

      <div
        className={cn(
          "h-full shrink-0 overflow-hidden border-border border-l transition-all duration-300 ease-in-out",
          rightSidebarOpen ? "w-60 opacity-100" : "w-0 border-l-0 opacity-0"
        )}
      >
        <div className="h-full w-60">
          <DetailsSidebar />
        </div>
      </div>
    </div>
  );
}
