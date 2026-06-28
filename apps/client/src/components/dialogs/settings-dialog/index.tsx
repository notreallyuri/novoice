import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogTitle,
} from "@/components/ui/dialog";
import { useUIStore } from "@/store/ui-store";
import { SidebarProvider } from "../ui/sidebar";
import { useSidebarData } from "./nav-hook";
import { SettingsRender } from "./render";
import { SettingsSidebar } from "./settings-sidebar";

export function SettingDialog() {
  const { sidebarData, setActiveRoute, activeRoute } = useSidebarData();

  const settingsOpen = useUIStore((state) => state.settingsOpen);
  const setSettingsOpen = useUIStore((state) => state.setSettingsOpen);

  return (
    <Dialog onOpenChange={setSettingsOpen} open={settingsOpen}>
      <DialogContent className="flex h-[calc(100vh-10rem)] w-[calc(100vw-16rem)] max-w-none! overflow-hidden border border-sidebar p-0">
        <DialogTitle className="sr-only">Settings</DialogTitle>
        <DialogDescription className="sr-only">
          Configure your preferences and settings for the application
        </DialogDescription>
        <SidebarProvider className="min-h-0 flex-1">
          <SettingsSidebar
            currentRoute={activeRoute}
            data={sidebarData}
            onRouteChange={setActiveRoute}
          />
          <main className="relative flex min-h-0 flex-1 flex-col overflow-hidden bg-background">
            <div className="w-full shrink-0 border-border border-b px-8 py-2">
              <h1 className="font-bold text-2xl">{activeRoute.name}</h1>
              <p className="text-muted-foreground text-sm">
                {activeRoute.description}
              </p>
            </div>
            <div className="min-h-0 flex-1 overflow-y-auto px-8 py-6">
              <SettingsRender currentRoute={activeRoute.name} />
            </div>
          </main>
        </SidebarProvider>
      </DialogContent>
    </Dialog>
  );
}
