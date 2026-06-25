import React, { useEffect } from "react";
import ReactDOM from "react-dom/client";
import "./globals.css";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { MainLayout } from "./components/layout/main-layout";
import { LoaderScreen } from "./components/loader";
import { TooltipProvider } from "./components/ui/tooltip";
import { useSpatialNavigation } from "./hooks/use-spatial-navigation";
import { useAuthStore } from "./store/auth-store";
import { useCallStore } from "./store/call-store";
import { useWorkspaceStore } from "./store/workspace-store";

const currentWindow = getCurrentWindow();

function App() {
  const setupWorkspaceListeners = useWorkspaceStore((s) => s.setupListeners);
  const setupCallListeners = useCallStore((s) => s.setupListeners);
  const setupAuthListeners = useAuthStore((s) => s.setupListeners);

  useSpatialNavigation();

  useEffect(() => {
    let unlistenWorkspace: () => void;
    let unlistenCall: () => void;
    let unlistenAuth: () => void;

    setupWorkspaceListeners().then((fn) => {
      unlistenWorkspace = fn;
    });

    setupCallListeners().then((fn) => {
      unlistenCall = fn;
    });

    setupAuthListeners().then((fn) => {
      unlistenAuth = fn;
    });

    return () => {
      if (unlistenWorkspace) {
        unlistenWorkspace();
      }
      if (unlistenCall) {
        unlistenCall();
      }
      if (unlistenAuth) {
        unlistenAuth();
      }
    };
  }, [setupWorkspaceListeners, setupCallListeners, setupAuthListeners]);

  if (currentWindow.label === "loader") {
    return <LoaderScreen />;
  }

  return (
    <TooltipProvider>
      <MainLayout />
    </TooltipProvider>
  );
}

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);
