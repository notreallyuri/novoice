import { Fragment, useEffect, useState } from "react";
import { Panel, Group } from "react-resizable-panels";
import { X, Hash } from "lucide-react";
import { Handle } from "./Handle";
import type { ViewPanel } from "@/types/workspace";
import { UnlistenFn, listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";
import { Textarea } from "../ui/textarea";

export function WorkspaceGrid() {
  const [activePanels, setActivePanels] = useState<ViewPanel[]>([]);
  const [isBooting, setIsBooting] = useState(true);

  useEffect(() => {
    let unlisten: UnlistenFn;

    const setupListener = async () => {
      unlisten = await listen<ViewPanel[]>("panels_updated", (event) => {
        setActivePanels(event.payload);
        setIsBooting(false);
      });

      invoke("request_initial_panels");
    };

    setupListener();

    return () => {
      if (unlisten) unlisten();
    };
  }, []);

  const closePanel = (idToRemove: string) => {
    invoke("close_panel", { idToRemove }).catch(console.error);
  };

  if (isBooting) {
    return (
      <div className="w-full h-full bg-background flex items-center justify-center">
        Loading Workspace...
      </div>
    );
  }

  if (activePanels.length === 0) {
    return (
      <div className="w-full h-full flex items-center justify-center bg-background text-muted-foreground uppercase tracking-widest text-sm font-bold">
        No Active Workspaces
      </div>
    );
  }

  return (
    <div className="w-full h-full bg-background overflow-hidden">
      <Group orientation="horizontal">
        {activePanels.map((panel, index) => (
          <Fragment key={panel.id}>
            <Panel defaultSize={100 / activePanels.length} minSize={15}>
              <div className="w-full border h-full flex flex-col bg-[oklch(0.1344_0_0)]">
                {/* Header */}
                <div className="h-12 border-b border-border flex items-center px-4 justify-between bg-muted/10 group">
                  <div className="flex items-center gap-2 font-bold">
                    {panel.type === "channel" && (
                      <Hash size={18} className="text-muted-foreground" />
                    )}
                    <span>{panel.title}</span>
                  </div>
                  <button
                    onClick={() => closePanel(panel.id)}
                    className="opacity-0 group-hover:opacity-100 p-1 hover:bg-destructive hover:text-white transition-all border-2 border-transparent hover:border-destructive"
                  >
                    <X size={16} />
                  </button>
                </div>

                <div className="flex-1 p-4 flex flex-col justify-end overflow-y-auto">
                  <div className="text-sm text-muted-foreground pb-4 border-b-2 border-border mb-4">
                    Welcome to the start of #{panel.title}.
                  </div>
                </div>

                <div className="p-4 border-t border-border bg-background">
                  <Textarea
                    placeholder={`Message #${panel.title}`}
                    className="w-full resize-none bg-[oklch(0.1344_0_0)] border border-border p-3 outline-none focus:border-primary transition-colors font-medium placeholder:text-muted-foreground/50"
                  />
                </div>
              </div>
            </Panel>

            {index < activePanels.length - 1 && <Handle />}
          </Fragment>
        ))}
      </Group>
    </div>
  );
}
