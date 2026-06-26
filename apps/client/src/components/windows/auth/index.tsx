import { Mic } from "lucide-react";
import { useState } from "react";
import { ThemeToggle } from "@/components/theme-toggle";
import { Button } from "@/components/ui/button";
import { LoginForm } from "./login-form";
import { RegisterForm } from "./register-form";

export function AuthWindow() {
  const [view, setView] = useState<"login" | "register">("login");

  return (
    <div className="flex h-screen w-screen bg-background">
      <div className="relative flex h-full w-full flex-col items-center justify-center overflow-hidden bg-zinc-50 dark:bg-zinc-950">
        <div className="absolute top-[-20%] left-[-10%] h-[50%] w-[50%] rounded-full bg-primary/20 blur-[120px]" />
        <div className="absolute right-[-10%] bottom-[-20%] h-[50%] w-[50%] rounded-full bg-primary/20 blur-[120px]" />
        <div className="fade-in zoom-in-95 z-10 flex animate-in flex-col items-center gap-6 text-center duration-1000">
          <div className="flex size-20 items-center justify-center rounded-3xl border border-primary/20 bg-primary/10 shadow-2xl backdrop-blur-md">
            <Mic className="text-primary" size={40} />
          </div>
          <div className="space-y-2">
            <h2 className="font-bold text-3xl text-foreground tracking-tight">
              Welcome to NoVoice
            </h2>
            <p className="max-w-sm text-muted-foreground text-sm">
              Experience crystal-clear voice and seamless collaborative
              workspaces built for power users.
            </p>
          </div>
        </div>

        <div className="mask-[radial-gradient(ellipse_50%_50%_at_50%_50%,#000_70%,transparent_100%)] absolute inset-0 bg-[linear-gradient(to_right,#80808012_1px,transparent_1px),linear-gradient(to_bottom,#80808012_1px,transparent_1px)] bg-size-[24px_24px]" />
      </div>

      <div className="z-20 h-full w-0.5 bg-border shadow-[-10px_0_20px_rgba(0,0,0,0.5)]" />

      <div className="relative z-20 flex h-full w-125 shrink-0 flex-col items-center justify-center bg-background">
        <div className="absolute top-4 right-4">
          <ThemeToggle />
        </div>
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
    </div>
  );
}
