import { Moon, Palette, Sun } from "lucide-react";
import { cn } from "@/lib/utils";
import type { ThemeColor, ThemeDarkMode } from "@/types/user";
import { useTheme } from "./theme-provider";
import { Button } from "./ui/button";
import { Popover, PopoverContent, PopoverTrigger } from "./ui/popover";

export function ThemeToggle() {
  const { theme, color, setTheme, setColor } = useTheme();

  const modes: ThemeDarkMode[] = ["Light", "Dark", "System"];
  const colors: ThemeColor[] = ["Default", "Havoc", "Void"];

  return (
    <Popover>
      <PopoverTrigger render={<Button size="icon" variant="ghost" />}>
        <Palette className="size-4 text-muted-foreground" />
      </PopoverTrigger>
      <PopoverContent align="end" className="w-48 p-2" sideOffset={8}>
        <div className="flex flex-col gap-4">
          <div className="flex flex-col gap-2">
            <span className="font-semibold text-muted-foreground text-xs uppercase tracking-wider">
              Mode
            </span>
            <div className="grid grid-cols-3 gap-1 rounded-none border border-border bg-muted/50 p-1">
              {modes.map((m) => (
                <button
                  className={cn(
                    "flex items-center justify-center rounded-none py-1 text-xs transition-colors",
                    theme === m
                      ? "bg-background text-foreground shadow-sm"
                      : "text-muted-foreground hover:text-foreground"
                  )}
                  key={m}
                  onClick={() => setTheme(m)}
                  type="button"
                >
                  {m === "Light" && <Sun size={14} />}
                  {m === "Dark" && <Moon size={14} />}
                  {m === "System" && (
                    <span className="font-medium text-[10px]">AUTO</span>
                  )}
                </button>
              ))}
            </div>
          </div>

          <div className="flex flex-col gap-2">
            <span className="font-semibold text-muted-foreground text-xs uppercase tracking-wider">
              Accent
            </span>
            <div className="flex flex-col gap-1">
              {colors.map((c) => (
                <button
                  className={cn(
                    "flex items-center gap-2 rounded-none px-2 py-1.5 text-left text-xs transition-colors",
                    color === c
                      ? "bg-primary text-primary-foreground"
                      : "hover:bg-muted"
                  )}
                  key={c}
                  onClick={() => setColor(c)}
                  type="button"
                >
                  <div
                    className={cn(
                      "size-3 rounded-full border border-border/50",
                      c === "Default" && "bg-zinc-500",
                      c === "Havoc" && "bg-red-900",
                      c === "Void" && "bg-blue-900"
                    )}
                  />
                  {c}
                </button>
              ))}
            </div>
          </div>
        </div>
      </PopoverContent>
    </Popover>
  );
}
