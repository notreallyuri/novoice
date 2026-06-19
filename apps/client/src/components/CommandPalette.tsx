import { useEffect, useState } from "react";
import {
  Command,
  CommandDialog,
  CommandEmpty,
  CommandGroup,
  CommandInput,
  CommandItem,
  CommandList,
} from "./ui/command";
import { invoke } from "@tauri-apps/api/core";
import { Hash } from "lucide-react";
import { ViewPanel } from "@/types/workspace";

const panels: ViewPanel[] = [
  {
    id: "1",
    title: "announcements",
    type: "channel",
    targetId: "c-101",
  },
  {
    id: "2",
    title: "rust-backend",
    type: "channel",
    targetId: "c-102",
  },
  {
    id: "3",
    title: "frontend",
    type: "channel",
    targetId: "c-103",
  },
];

export function CommandPalette() {
  const [open, setOpen] = useState(false);

  useEffect(() => {
    const down = (e: KeyboardEvent) => {
      if (e.key == "k" && (e.metaKey || e.ctrlKey)) {
        e.preventDefault();
        setOpen((open) => !open);
      }
    };

    document.addEventListener("keydown", down);
    return () => document.removeEventListener("keydown", down);
  }, []);

  const triggerChannelOpen = async (
    targetId: string,
    title: string,
    panelType: string,
  ) => {
    try {
      await invoke("open_panel", {
        panelId: crypto.randomUUID(),
        targetId,
        title,
        panelType,
      });
      setOpen(false);
    } catch (err) {
      console.error("Failed to open panel:", err);
    }
  };

  if (!open) return null;

  return (
    <CommandDialog open={open} onOpenChange={setOpen}>
      <Command loop>
        <CommandInput />
        <CommandList>
          <CommandEmpty>No results found.</CommandEmpty>
          <CommandGroup>
            <CommandGroup heading="Text Channels">
              {panels.map((p) => (
                <CommandItem
                  key={p.id}
                  id={p.id}
                  onSelect={() =>
                    triggerChannelOpen(p.targetId, p.title, p.type)
                  }
                  className="flex items-center gap-2 cursor-pointer aria-selected:bg-muted  rounded-none"
                >
                  <Hash />
                  <span className="font-bold">{p.title}</span>
                </CommandItem>
              ))}
            </CommandGroup>
          </CommandGroup>
        </CommandList>
      </Command>
    </CommandDialog>
  );
}
