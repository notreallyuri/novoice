import {
  createContext,
  type ReactNode,
  useContext,
  useEffect,
  useState,
} from "react";
import type { ThemeColor, ThemeDarkMode } from "@/types/user";

interface ThemeProviderProps {
  children: ReactNode;
  defaultColor?: ThemeColor;
  defaultTheme?: ThemeDarkMode;
  storageKey?: string;
}

interface ThemeProviderState {
  color: ThemeColor;
  setColor: (color: ThemeColor) => void;
  setTheme: (theme: ThemeDarkMode) => void;
  theme: ThemeDarkMode;
}

const initialState: ThemeProviderState = {
  theme: "System",
  color: "Default",
  setTheme: () => null,
  setColor: () => null,
};

const ThemeProviderContext = createContext<ThemeProviderState>(initialState);

export function ThemeProvider({
  children,
  defaultTheme = "System",
  defaultColor = "Default",
  storageKey = "novoice-ui-theme",
  ...props
}: ThemeProviderProps) {
  const [theme, setTheme] = useState<ThemeDarkMode>(() => {
    const saved = localStorage.getItem(`${storageKey}-mode`);
    return (saved as ThemeDarkMode) || defaultTheme;
  });

  const [color, setColor] = useState<ThemeColor>(() => {
    const saved = localStorage.getItem(`${storageKey}-color`);
    return (saved as ThemeColor) || defaultColor;
  });

  useEffect(() => {
    const root = window.document.documentElement;

    root.classList.remove("light", "dark", "havoc", "void");

    if (theme === "System") {
      const systemTheme = window.matchMedia("(prefers-color-scheme: dark)")
        .matches
        ? "dark"
        : "light";
      root.classList.add(systemTheme);
    } else {
      root.classList.add(theme.toLowerCase());
    }

    if (color !== "Default") {
      root.classList.add(color.toLowerCase());
    }
  }, [theme, color]);

  const value = {
    theme,
    color,
    setTheme: (newTheme: ThemeDarkMode) => {
      localStorage.setItem(`${storageKey}-mode`, newTheme);
      setTheme(newTheme);
    },
    setColor: (newColor: ThemeColor) => {
      localStorage.setItem(`${storageKey}-color`, newColor);
      setColor(newColor);
    },
  };

  return (
    <ThemeProviderContext.Provider {...props} value={value}>
      {children}
    </ThemeProviderContext.Provider>
  );
}

export const useTheme = () => {
  const context = useContext(ThemeProviderContext);
  if (context === undefined) {
    throw new Error("useTheme must be used within a ThemeProvider");
  }
  return context;
};
