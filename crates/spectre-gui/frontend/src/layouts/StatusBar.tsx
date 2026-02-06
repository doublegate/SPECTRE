import { useStatus } from "@/hooks/useStatus";

export function StatusBar() {
  const { version, status } = useStatus();

  const componentCount = status?.components.length ?? 0;
  const availableCount =
    status?.components.filter((c) => c.status === "available").length ?? 0;

  return (
    <footer className="flex h-8 items-center justify-between border-t border-border bg-card px-4 text-xs text-muted-foreground">
      <div className="flex items-center gap-4">
        <span>
          SPECTRE {version?.version ?? "..."}
        </span>
        <span className="text-border">|</span>
        <span>
          Components: {availableCount}/{componentCount}
        </span>
      </div>
      <div className="flex items-center gap-4">
        <span>
          {status?.config_loaded ? "Config loaded" : "No config"}
        </span>
      </div>
    </footer>
  );
}
