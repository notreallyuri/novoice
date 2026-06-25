import { PanelLeftClose, PanelRightClose } from "lucide-react";
import { CommandPalette } from "@/components/command-palette";
import { FloatingCallManager } from "@/components/floating-call-manager";
import { AppShell } from "@/components/layout/app-shell";
import { Kbd, KbdGroup } from "@/components/ui/kbd";
import { useUIStore } from "@/store/ui-store";

export function MainLayout() {
  const toggleLeftSidebar = useUIStore((s) => s.toggleLeftSidebar);
  const toggleRightSidebar = useUIStore((s) => s.toggleRightSidebar);

  return (
    <div className="relative flex h-screen w-screen flex-col overflow-hidden bg-background text-foreground">
      <FloatingCallManager />
      <CommandPalette />

      <header className="flex h-10 shrink-0 items-center justify-between border-border border-b bg-muted/20 px-4">
        <div className="flex items-center gap-4">
          <button
            className="text-muted-foreground transition-colors hover:text-foreground"
            onClick={toggleLeftSidebar}
            type="button"
          >
            <PanelLeftClose size={18} />
          </button>
          <h1 className="font-bold text-sm tracking-wide">NoVoice</h1>
        </div>

        <p className="font-semibold text-muted-foreground text-xs">
          Press{" "}
          <KbdGroup>
            <Kbd>Ctrl</Kbd> + <Kbd>Space</Kbd>
          </KbdGroup>{" "}
          to open palette
        </p>

        <button
          className="text-muted-foreground transition-colors hover:text-foreground"
          onClick={toggleRightSidebar}
          type="button"
        >
          <PanelRightClose size={18} />
        </button>
      </header>

      <main className="relative h-full w-full flex-1 overflow-hidden">
        <AppShell />
      </main>
    </div>
  );
}
