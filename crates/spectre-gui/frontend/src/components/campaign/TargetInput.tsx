import { useState } from "react";
import { CheckCircle2, FileText, Loader2 } from "lucide-react";
import { useTargetParser } from "@/hooks/useTargetParser";

interface TargetInputProps {
  onTargetsChange: (targets: string[]) => void;
  initialValue?: string;
}

export function TargetInput({ onTargetsChange, initialValue = "" }: TargetInputProps) {
  const [input, setInput] = useState(initialValue);
  const { targets, loading, error, parseTargets, clear } = useTargetParser();

  const handleParse = async () => {
    const lines = input
      .split("\n")
      .map((line) => line.trim())
      .filter((line) => line.length > 0);

    if (lines.length === 0) {
      return;
    }

    try {
      const parsed = await parseTargets(lines);
      const allTargets = parsed.flatMap((p) => p.expanded);
      onTargetsChange(allTargets);
    } catch (err) {
      // Error is handled by the hook
      console.error("Failed to parse targets:", err);
    }
  };

  const handleClear = () => {
    setInput("");
    clear();
    onTargetsChange([]);
  };

  const totalTargets = targets.reduce((sum, t) => sum + t.count, 0);

  return (
    <div className="space-y-3">
      <div className="flex items-center justify-between">
        <label className="text-sm font-medium text-foreground">Target Specification</label>
        {targets.length > 0 && (
          <span className="text-xs text-green-500">
            <CheckCircle2 className="mr-1 inline h-3 w-3" />
            {totalTargets} target{totalTargets !== 1 ? "s" : ""} parsed
          </span>
        )}
      </div>

      <textarea
        value={input}
        onChange={(e) => setInput(e.target.value)}
        placeholder="Enter targets (one per line):&#10;192.168.1.0/24&#10;10.0.0.1&#10;example.com"
        rows={6}
        className="w-full rounded-md border border-input bg-background px-3 py-2 font-mono text-sm text-foreground placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-ring"
      />

      {error && (
        <div className="rounded-md border border-destructive/20 bg-destructive/10 p-3">
          <p className="text-sm text-destructive">{error}</p>
        </div>
      )}

      {targets.length > 0 && (
        <div className="rounded-md border border-border bg-card p-3">
          <h4 className="mb-2 text-sm font-medium text-foreground">Parsed Targets:</h4>
          <div className="space-y-1 text-sm">
            {targets.map((target, index) => (
              <div key={index} className="flex justify-between text-muted-foreground">
                <span className="font-mono">{target.original}</span>
                <span>
                  {target.count} {target.count === 1 ? "host" : "hosts"}
                </span>
              </div>
            ))}
          </div>
        </div>
      )}

      <div className="flex gap-2">
        <button
          onClick={handleParse}
          disabled={loading || input.trim().length === 0}
          className="flex items-center gap-2 rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground transition-opacity hover:opacity-90 disabled:opacity-50"
        >
          {loading && <Loader2 className="h-4 w-4 animate-spin" />}
          Parse Targets
        </button>

        <button
          onClick={handleClear}
          disabled={input.length === 0 && targets.length === 0}
          className="rounded-md border border-input bg-background px-4 py-2 text-sm font-medium text-foreground transition-colors hover:bg-accent disabled:opacity-50"
        >
          Clear
        </button>

        <button
          disabled
          className="flex items-center gap-2 rounded-md border border-input bg-background px-4 py-2 text-sm font-medium text-muted-foreground opacity-50"
          title="File import coming in future sprint"
        >
          <FileText className="h-4 w-4" />
          Import File
        </button>
      </div>

      <p className="text-xs text-muted-foreground">
        Supported formats: Individual IPs (192.168.1.1), CIDR notation (192.168.1.0/24), hostnames
        (example.com)
      </p>
    </div>
  );
}
