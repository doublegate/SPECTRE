import { useState, useEffect } from 'react';
import { FlaskConical } from 'lucide-react';
import { invoke } from '@tauri-apps/api/core';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select';
import type { ChefOperation, ChefRequest, ChefResult } from '@/types/chef';
import { CHEF_CATEGORIES } from '@/types/chef';

export function Analysis() {
  const [operations, setOperations] = useState<ChefOperation[]>([]);
  const [selectedOp, setSelectedOp] = useState<string>('');
  const [category, setCategory] = useState<string>('All');
  const [input, setInput] = useState<string>('');
  const [output, setOutput] = useState<string>('');
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | undefined>();

  useEffect(() => {
    loadOperations();
  }, [category]);

  const loadOperations = async () => {
    try {
      const cat = category === 'All' ? undefined : category;
      const ops = await invoke<ChefOperation[]>('list_chef_operations', { category: cat });
      setOperations(ops);
      if (ops.length > 0 && !selectedOp) {
        setSelectedOp(ops[0].name);
      }
    } catch (err) {
      console.error('Failed to load operations:', err);
      setError(err instanceof Error ? err.message : 'Failed to load operations');
    }
  };

  const executeOperation = async () => {
    if (!selectedOp || !input) return;

    setLoading(true);
    setError(undefined);

    try {
      const request: ChefRequest = {
        operation: selectedOp,
        input,
        args: undefined,
      };
      const result = await invoke<ChefResult>('execute_chef', { request });
      setOutput(result.output);
    } catch (err) {
      console.error('Failed to execute operation:', err);
      setError(err instanceof Error ? err.message : 'Operation failed');
      setOutput('');
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="space-y-4">
      <div className="flex items-center gap-3">
        <FlaskConical className="h-5 w-5 text-primary" />
        <h2 className="text-lg font-semibold">Data Analysis</h2>
      </div>

      {/* Operations Browser */}
      <Card>
        <CardHeader>
          <CardTitle>CyberChef Operations</CardTitle>
          <CardDescription>
            Select a data transformation operation from {operations.length} available
          </CardDescription>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="grid gap-4 sm:grid-cols-2">
            <div className="space-y-2">
              <label className="text-sm font-medium">Category</label>
              <Select value={category} onValueChange={setCategory}>
                <SelectTrigger>
                  <SelectValue />
                </SelectTrigger>
                <SelectContent>
                  {CHEF_CATEGORIES.map((cat) => (
                    <SelectItem key={cat} value={cat}>
                      {cat}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            </div>

            <div className="space-y-2">
              <label className="text-sm font-medium">Operation</label>
              <Select value={selectedOp} onValueChange={setSelectedOp}>
                <SelectTrigger>
                  <SelectValue placeholder="Select operation" />
                </SelectTrigger>
                <SelectContent>
                  {operations.map((op) => (
                    <SelectItem key={op.name} value={op.name}>
                      {op.name}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            </div>
          </div>

          {selectedOp && (
            <div className="rounded-lg bg-muted p-3">
              <p className="text-sm text-muted-foreground">
                {operations.find((op) => op.name === selectedOp)?.description}
              </p>
            </div>
          )}
        </CardContent>
      </Card>

      {/* Input/Output */}
      <div className="grid gap-4 lg:grid-cols-2">
        <Card>
          <CardHeader>
            <CardTitle>Input</CardTitle>
          </CardHeader>
          <CardContent>
            <textarea
              value={input}
              onChange={(e) => setInput(e.target.value)}
              placeholder="Enter data to transform..."
              className="w-full rounded-md border border-input bg-background p-3 text-sm text-foreground placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-ring"
              rows={10}
            />
            <div className="mt-4">
              <Button onClick={executeOperation} disabled={loading || !selectedOp || !input}>
                {loading ? 'Processing...' : 'Execute'}
              </Button>
            </div>
          </CardContent>
        </Card>

        <Card>
          <CardHeader>
            <CardTitle>Output</CardTitle>
          </CardHeader>
          <CardContent>
            {error ? (
              <div className="rounded-lg bg-destructive/10 p-3 text-sm text-destructive">
                {error}
              </div>
            ) : (
              <div className="min-h-[200px] rounded-md border border-input bg-background p-3 text-sm font-mono text-foreground">
                {output || 'Results will appear here...'}
              </div>
            )}
          </CardContent>
        </Card>
      </div>
    </div>
  );
}

// Default export for lazy loading
export default Analysis;
