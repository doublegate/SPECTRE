import { SettingsIcon } from "lucide-react";
import { useUiStore, type ThemeName } from "@/stores/uiStore";
import { cn } from "@/lib/utils";

const themes: { name: ThemeName; label: string; description: string }[] = [
  { name: "dark", label: "Dark", description: "Default dark theme with blue accents" },
  { name: "light", label: "Light", description: "Light theme with midnight blue accents" },
  { name: "tactical", label: "Tactical", description: "Military green on black" },
  { name: "matrix", label: "Matrix", description: "Bright green on black" },
  { name: "hacker", label: "Hacker", description: "Cyan and magenta on dark" },
];

export function Settings() {
  const { theme, setTheme } = useUiStore();

  return (
    <div className="space-y-6">
      <div className="flex items-center gap-3">
        <SettingsIcon className="h-5 w-5 text-primary" />
        <h2 className="text-lg font-semibold">Settings</h2>
      </div>

      {/* Theme section */}
      <div className="rounded-lg border border-border bg-card p-4">
        <h3 className="mb-3 text-sm font-medium text-foreground">Theme</h3>
        <div className="grid gap-3 sm:grid-cols-2 lg:grid-cols-3">
          {themes.map((t) => (
            <button
              key={t.name}
              onClick={() => setTheme(t.name)}
              className={cn(
                "rounded-lg border p-3 text-left transition-colors",
                theme === t.name
                  ? "border-primary bg-primary/10"
                  : "border-border hover:border-muted-foreground",
              )}
            >
              <p className="text-sm font-medium text-foreground">{t.label}</p>
              <p className="text-xs text-muted-foreground">{t.description}</p>
            </button>
          ))}
        </div>
      </div>

      {/* General section */}
      <div className="rounded-lg border border-border bg-card p-4">
        <h3 className="mb-3 text-sm font-medium text-foreground">General</h3>
        <p className="text-sm text-muted-foreground">
          Additional settings (scan, chef, comms, output) will be available in Sprint 5.6.
        </p>
      </div>
    </div>
  );
}
