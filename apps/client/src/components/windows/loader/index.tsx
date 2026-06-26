import { invoke } from "@tauri-apps/api/core";
import { Loader2 } from "lucide-react";
import { useEffect } from "react";

export function LoaderWindow() {
  useEffect(() => {
    invoke("check_auth_status").catch(console.error);
  }, []);

  return (
    <div className="flex h-screen w-screen items-center justify-center rounded-lg border border-border bg-background">
      <div className="flex animate-pulse flex-col items-center gap-2">
        <Loader2 className="animate-spin text-primary" size={32} />
        <span className="font-semibold text-muted-foreground text-sm">
          Starting NoVoice...
        </span>
      </div>
    </div>
  );
}
