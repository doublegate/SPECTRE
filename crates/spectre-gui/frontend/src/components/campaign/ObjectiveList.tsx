import { useState } from "react";
import { Circle, Plus, X } from "lucide-react";

interface ObjectiveListProps {
  objectives: string[];
  editable: boolean;
  onChange?: (objectives: string[]) => void;
}

export function ObjectiveList({ objectives, editable, onChange }: ObjectiveListProps) {
  const [newObjective, setNewObjective] = useState("");

  const handleAdd = () => {
    if (newObjective.trim() && onChange) {
      onChange([...objectives, newObjective.trim()]);
      setNewObjective("");
    }
  };

  const handleRemove = (index: number) => {
    if (onChange) {
      onChange(objectives.filter((_, i) => i !== index));
    }
  };

  return (
    <div className="space-y-3">
      <div className="flex items-center justify-between">
        <h3 className="text-sm font-medium text-foreground">Campaign Objectives</h3>
        {editable && (
          <span className="text-xs text-muted-foreground">
            {objectives.length} objective{objectives.length !== 1 ? "s" : ""}
          </span>
        )}
      </div>

      <div className="space-y-2">
        {objectives.length === 0 ? (
          <p className="text-sm text-muted-foreground">No objectives defined yet.</p>
        ) : (
          objectives.map((objective, index) => (
            <div
              key={index}
              className="flex items-center gap-2 rounded-md border border-border bg-card p-2"
            >
              <Circle className="h-4 w-4 flex-shrink-0 text-muted-foreground" />
              <span className="flex-1 text-sm text-foreground">{objective}</span>
              {editable && onChange && (
                <button
                  onClick={() => handleRemove(index)}
                  className="rounded p-1 text-muted-foreground transition-colors hover:bg-destructive/10 hover:text-destructive"
                  title="Remove objective"
                >
                  <X className="h-3 w-3" />
                </button>
              )}
            </div>
          ))
        )}
      </div>

      {editable && onChange && (
        <div className="flex gap-2">
          <input
            type="text"
            value={newObjective}
            onChange={(e) => setNewObjective(e.target.value)}
            onKeyDown={(e) => {
              if (e.key === "Enter") {
                handleAdd();
              }
            }}
            placeholder="Add an objective..."
            className="flex-1 rounded-md border border-input bg-background px-3 py-2 text-sm text-foreground placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-ring"
          />
          <button
            onClick={handleAdd}
            disabled={!newObjective.trim()}
            className="rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground transition-opacity hover:opacity-90 disabled:opacity-50"
          >
            <Plus className="h-4 w-4" />
          </button>
        </div>
      )}
    </div>
  );
}
