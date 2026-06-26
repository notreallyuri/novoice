import React, { useEffect } from "react";
import ReactDOM from "react-dom/client";
import "./globals.css";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { MainLayout } from "./components/layout/main-layout";
import { ThemeProvider } from "./components/theme-provider";
import { TooltipProvider } from "./components/ui/tooltip";
import { AuthWindow } from "./components/windows/auth";
import { LoaderWindow } from "./components/windows/loader";
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
    if (currentWindow.label !== "main") {
      return;
    }

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
    return <LoaderWindow />;
  }

  if (currentWindow.label === "auth") {
    return <AuthWindow />;
  }

  return (
    <TooltipProvider>
      <MainLayout />
    </TooltipProvider>
  );
}

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <ThemeProvider defaultColor="Default" defaultTheme="System">
      <App />
    </ThemeProvider>
  </React.StrictMode>
);
