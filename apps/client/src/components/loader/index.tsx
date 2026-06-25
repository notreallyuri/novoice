import { invoke } from "@tauri-apps/api/core";
import { Loader2 } from "lucide-react";
import { useEffect, useState } from "react";
import { Button } from "../ui/button";
import { LoginForm } from "./login-form";
import { RegisterForm } from "./register-form";

export function LoaderScreen() {
  const [view, setView] = useState<"loading" | "login" | "register">("loading");

  useEffect(() => {
    invoke<boolean>("check_auth_status")
      .then((isAuthenticated) => {
        if (!isAuthenticated) {
          setView("login");
        }
      })
      .catch((err) => {
        console.error("Auth check failed:", err);
        setView("login");
      });
  }, []);

  if (view === "loading") {
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

  return (
    <div className="flex h-screen w-screen flex-col items-center justify-center rounded-lg border border-border bg-background">
      <div className="mb-4 flex flex-col items-center gap-1">
        <h1 className="font-bold text-xl tracking-tight">NoVoice</h1>
        <p className="text-center text-muted-foreground text-xs">
          {view === "login" ? "Welcome back." : "Create an account."}
        </p>
      </div>

      {view === "login" ? <LoginForm /> : <RegisterForm />}

      <Button
        className="mt-2 text-muted-foreground text-xs"
        onClick={() => setView(view === "login" ? "register" : "login")}
        size="sm"
        variant="link"
      >
        {view === "login"
          ? "Need an account? Register"
          : "Already have an account? Login"}
      </Button>
    </div>
  );
}
