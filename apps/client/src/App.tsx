import "./globals.css";
import { FloatingCallManager } from "@/components/FloatingCallManager";
import { WorkspaceGrid } from "@/components/workspace/WorkspaceGrid";
import { CommandPalette } from "@/components/CommandPalette";
import { useEffect, useState } from "react";
import { listen, UnlistenFn } from "@tauri-apps/api/event";

interface CallState {
  isActive: boolean;
  channeld?: string;
  channelName?: string;
}

function App() {
  const [_callState, setCallState] = useState<CallState>({ isActive: false });

  useEffect(() => {
    let unlisten: UnlistenFn;

    const setupCallListener = async () => {
      unlisten = await listen<CallState>("call_state_updated", (event) => {
        setCallState(event.payload);
      });
    };

    setupCallListener();

    return () => {
      if (unlisten) unlisten();
    };
  }, []);

  return (
    <div className="h-screen w-screen overflow-hidden bg-background text-foreground relative flex">
      <FloatingCallManager />
      <CommandPalette />
      <main className="flex-1 p-2 pt-0 relative flex flex-col w-full h-full">
        <div className="shrink-0 justify-between items-center flex w-full px-2 py-1">
          <h1 className="font-bold">NoVoice</h1>
          <p className="text-sm font-semibold text-muted-foreground">
            Press <code className="bg-muted py-0.5 px-1 mx-1">CTRL + K</code> to
            open the command palette
          </p>
        </div>
        <WorkspaceGrid />
      </main>
    </div>
  );
}

export default App;
