import { useLocation } from "react-router";
import { Palette } from "lucide-react";
import { useUiStore, type ThemeName } from "@/stores/uiStore";
import { useState } from "react";
import { cn } from "@/lib/utils";

const themes: { name: ThemeName; label: string }[] = [
  { name: "dark", label: "Dark" },
  { name: "light", label: "Light" },
  { name: "tactical", label: "Tactical" },
  { name: "matrix", label: "Matrix" },
  { name: "hacker", label: "Hacker" },
];

const pageTitles: Record<string, string> = {
  "/dashboard": "Dashboard",
  "/targets": "Targets",
  "/recon": "Recon",
  "/analysis": "Analysis",
  "/comms": "Comms",
  "/campaigns": "Campaigns",
  "/reports": "Reports",
  "/settings": "Settings",
};

export function Header() {
  const location = useLocation();
  const { theme, setTheme } = useUiStore();
  const [themeOpen, setThemeOpen] = useState(false);
  const title = pageTitles[location.pathname] ?? "SPECTRE";

  return (
    <header className="flex h-14 items-center justify-between border-b border-border bg-card px-4">
      <h1 className="text-lg font-semibold text-foreground">{title}</h1>

      {/* Theme picker */}
      <div className="relative">
        <button
          onClick={() => setThemeOpen(!themeOpen)}
          className="flex items-center gap-2 rounded-md px-3 py-1.5 text-sm text-muted-foreground hover:bg-muted/30 hover:text-foreground"
        >
          <Palette className="h-4 w-4" />
          <span className="capitalize">{theme}</span>
        </button>

        {themeOpen && (
          <div className="absolute right-0 top-full z-50 mt-1 w-36 rounded-md border border-border bg-popover p-1 shadow-lg">
            {themes.map((t) => (
              <button
                key={t.name}
                onClick={() => {
                  setTheme(t.name);
                  setThemeOpen(false);
                }}
                className={cn(
                  "flex w-full items-center rounded-sm px-3 py-1.5 text-sm",
                  theme === t.name
                    ? "bg-accent text-accent-foreground"
                    : "text-popover-foreground hover:bg-muted/30",
                )}
              >
                {t.label}
              </button>
            ))}
          </div>
        )}
      </div>
    </header>
  );
}
