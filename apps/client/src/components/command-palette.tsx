import {
  ArrowLeft,
  Grid,
  Hash,
  HelpCircle,
  LayoutPanelLeft,
  LayoutPanelTop,
  type LucideIcon,
  Monitor,
  PanelLeftClose,
  PanelRightClose,
  Plus,
  Replace,
  Settings,
  X,
} from "lucide-react";
import { useEffect, useState } from "react";
import { cn } from "@/lib/utils";
import { useUIStore } from "@/store/ui-store";
import { type SpaceData, useWorkspaceStore } from "@/store/workspace-store";
import {
  Command,
  CommandDialog,
  CommandEmpty,
  CommandGroup,
  CommandInput,
  CommandItem,
  CommandList,
} from "./ui/command";

const MOCK_CHANNELS = [
  { targetId: "c-101", title: "announcements", type: "channel" },
  { targetId: "c-102", title: "rust-backend", type: "channel" },
  { targetId: "c-103", title: "frontend", type: "channel" },
];

interface MenuOption {
  action: () => void;
  danger?: boolean;
  icon: LucideIcon;
  keepOpen?: boolean;
  title: string;
}

export function CommandPalette() {
  const [open, setOpen] = useState(false);
  const [input, setInput] = useState("");
  const [page, setPage] = useState<"home" | "grid" | "channel_action" | "ui">(
    "home"
  );
  const [stagedChannel, setStagedChannel] = useState<Omit<
    SpaceData,
    "id"
  > | null>(null);

  const openPanel = useWorkspaceStore((state) => state.openPanel);
  const splitPanel = useWorkspaceStore((state) => state.splitPanel);
  const replacePanel = useWorkspaceStore((state) => state.replacePanel);
  const setDirection = useWorkspaceStore((state) => state.setDirection);
  const closePanel = useWorkspaceStore((state) => state.closePanel);
  const activePanelId = useWorkspaceStore((state) => state.activePanelId);

  const setSettingsOpen = useUIStore((state) => state.setSettingsOpen);
  const toggleRightSidebar = useUIStore((state) => state.toggleRightSidebar);
  const toggleLeftSidebar = useUIStore((state) => state.toggleLeftSidebar);

  useEffect(() => {
    setInput("");
  }, []);

  useEffect(() => {
    const down = (e: KeyboardEvent) => {
      if (e.key === " " && (e.metaKey || e.ctrlKey)) {
        e.preventDefault();
        setOpen((currentOpen) => {
          if (!currentOpen) {
            setPage("home");
          }
          return !currentOpen;
        });
      }
    };

    document.addEventListener("keydown", down);
    return () => document.removeEventListener("keydown", down);
  }, []);

  const run = (command: () => void, keepOpen?: boolean) => {
    command();
    if (!keepOpen) {
      setOpen(false);
    }
  };

  const handleStageChannel = (channel: Omit<SpaceData, "id">) => {
    setStagedChannel(channel);
    setPage("channel_action");
  };

  const HOME_OPTIONS: MenuOption[] = [
    { title: "Settings", icon: Settings, action: () => setSettingsOpen(true) },
    {
      title: "Grid Controls ...",
      icon: Grid,
      action: () => setPage("grid"),
      keepOpen: true,
    },
    {
      title: "UI Options ...",
      icon: Monitor,
      action: () => setPage("ui"),
      keepOpen: true,
    },
    {
      title: "Help",
      icon: HelpCircle,
      action: () => console.log("Help Clicked"),
    },
  ];

  const UI_OPTIONS: MenuOption[] = [
    {
      title: "Toggle Right Sidebar",
      icon: PanelRightClose,
      action: toggleRightSidebar,
    },
    {
      title: "Toggle Left Sidebar",
      icon: PanelLeftClose,
      action: toggleLeftSidebar,
    },
  ];

  const GRID_OPTIONS: MenuOption[] = [
    {
      title: "Split Horizontally",
      icon: LayoutPanelLeft,
      action: () => setDirection("horizontal"),
    },
    {
      title: "Split Vertically",
      icon: LayoutPanelTop,
      action: () => setDirection("vertical"),
    },
  ];

  if (activePanelId) {
    GRID_OPTIONS.push({
      title: "Close Active Panel",
      icon: X,
      action: () => closePanel(activePanelId),
      danger: true,
    });
  }

  const CHANNEL_ACTIONS: MenuOption[] = [];

  if (stagedChannel) {
    if (activePanelId) {
      CHANNEL_ACTIONS.push(
        {
          title: "Replace Active Panel",
          icon: Replace,
          action: () => replacePanel(activePanelId, stagedChannel),
        },
        {
          title: "Split Right of Active",
          icon: LayoutPanelLeft,
          action: () => splitPanel(activePanelId, "horizontal", stagedChannel),
        },
        {
          title: "Split Down from Active",
          icon: LayoutPanelTop,
          action: () => splitPanel(activePanelId, "vertical", stagedChannel),
        }
      );
    }
    CHANNEL_ACTIONS.push({
      title: "Append to Workspace",
      icon: Plus,
      action: () => openPanel(stagedChannel),
    });
  }

  if (!open) {
    return null;
  }

  return (
    <CommandDialog onOpenChange={setOpen} open={open}>
      <Command loop>
        <CommandInput
          onValueChange={setInput}
          placeholder="Type a command or search..."
          value={input}
        />
        <CommandList>
          <CommandEmpty>No results found.</CommandEmpty>

          {page === "home" && (
            <>
              <CommandGroup heading="Channels">
                {MOCK_CHANNELS.map((channel) => (
                  <CommandItem
                    className="flex cursor-pointer items-center gap-2"
                    key={channel.targetId}
                    onSelect={() => handleStageChannel(channel)}
                  >
                    <Hash className="text-muted-foreground" size={16} />
                    <span>{channel.title}</span>
                  </CommandItem>
                ))}
              </CommandGroup>

              <CommandGroup heading="Navigation">
                {HOME_OPTIONS.map((option) => (
                  <CommandItem
                    className="flex cursor-pointer items-center gap-2"
                    key={option.title}
                    onSelect={() => run(option.action, option.keepOpen)}
                  >
                    <option.icon className="text-muted-foreground" size={16} />
                    <span>{option.title}</span>
                  </CommandItem>
                ))}
              </CommandGroup>
            </>
          )}

          {page === "channel_action" && stagedChannel && (
            <>
              <CommandGroup heading={`Action for #${stagedChannel.title}`}>
                {CHANNEL_ACTIONS.map((option) => (
                  <CommandItem
                    className="flex cursor-pointer items-center gap-2"
                    key={option.title}
                    onSelect={() => run(option.action, option.keepOpen)}
                  >
                    <option.icon className="text-muted-foreground" size={16} />
                    <span>{option.title}</span>
                  </CommandItem>
                ))}
              </CommandGroup>

              <CommandGroup heading="Navigation">
                <CommandItem
                  className="flex cursor-pointer items-center gap-2"
                  onSelect={() => setPage("home")}
                >
                  <ArrowLeft className="text-muted-foreground" size={16} />
                  <span>Back</span>
                </CommandItem>
              </CommandGroup>
            </>
          )}

          {page === "ui" && (
            <>
              <CommandGroup heading="UI Options">
                {UI_OPTIONS.map((option) => (
                  <CommandItem
                    className="flex cursor-pointer items-center gap-2"
                    key={option.title}
                    onSelect={() => run(option.action, option.keepOpen)}
                  >
                    <option.icon size={16} />
                    <span>{option.title}</span>
                  </CommandItem>
                ))}
              </CommandGroup>

              <CommandGroup heading="Navigation">
                <CommandItem
                  className="flex cursor-pointer items-center gap-2"
                  onSelect={() => setPage("home")}
                >
                  <ArrowLeft className="text-muted-foreground" size={16} />
                  <span>Back</span>
                </CommandItem>
              </CommandGroup>
            </>
          )}

          {page === "grid" && (
            <>
              <CommandGroup heading="Grid Layout & Actions">
                {GRID_OPTIONS.map((option) => (
                  <CommandItem
                    className={cn(
                      "flex cursor-pointer items-center gap-2",
                      option.danger &&
                        "text-destructive data-[selected=true]:bg-destructive/10 data-[selected=true]:text-destructive"
                    )}
                    key={option.title}
                    onSelect={() => run(option.action, option.keepOpen)}
                  >
                    <option.icon
                      className={cn(!option.danger && "text-muted-foreground")}
                      size={16}
                    />
                    <span>{option.title}</span>
                  </CommandItem>
                ))}
              </CommandGroup>

              <CommandGroup heading="Navigation">
                <CommandItem
                  className="flex cursor-pointer items-center gap-2"
                  onSelect={() => setPage("home")}
                >
                  <ArrowLeft className="text-muted-foreground" size={16} />
                  <span>Back to Main Menu</span>
                </CommandItem>
              </CommandGroup>
            </>
          )}
        </CommandList>
      </Command>
    </CommandDialog>
  );
}
