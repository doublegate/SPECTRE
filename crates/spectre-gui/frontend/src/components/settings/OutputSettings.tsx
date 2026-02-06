import { useSettings } from '@/hooks/useSettings';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Label } from '@/components/ui/label';
import { Checkbox } from '@/components/ui/checkbox';
import { OUTPUT_FORMATS } from '@/types/settings';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select';

export function OutputSettings() {
  const { config, loading, updateConfig } = useSettings();

  if (loading || !config) {
    return <div className="text-muted-foreground">Loading settings...</div>;
  }

  const handleFormatChange = async (value: string) => {
    await updateConfig({
      output: {
        ...config.output,
        format: value,
      },
    });
  };

  const handleTimestampsChange = async (checked: boolean) => {
    await updateConfig({
      output: {
        ...config.output,
        timestamps: checked,
      },
    });
  };

  const handlePrettyJsonChange = async (checked: boolean) => {
    await updateConfig({
      output: {
        ...config.output,
        pretty_json: checked,
      },
    });
  };

  return (
    <Card>
      <CardHeader>
        <CardTitle>Output Settings</CardTitle>
        <CardDescription>Configure output formats and options</CardDescription>
      </CardHeader>
      <CardContent className="space-y-4">
        <div className="space-y-2">
          <Label htmlFor="format">Default Output Format</Label>
          <Select value={config.output.format} onValueChange={handleFormatChange}>
            <SelectTrigger id="format">
              <SelectValue placeholder="Select format" />
            </SelectTrigger>
            <SelectContent>
              {OUTPUT_FORMATS.map((fmt) => (
                <SelectItem key={fmt.value} value={fmt.value}>
                  {fmt.label} - {fmt.description}
                </SelectItem>
              ))}
            </SelectContent>
          </Select>
        </div>

        <div className="space-y-2">
          <div className="flex items-center space-x-2">
            <Checkbox
              id="timestamps"
              checked={config.output.timestamps}
              onCheckedChange={handleTimestampsChange}
            />
            <Label htmlFor="timestamps" className="cursor-pointer">
              Include timestamps in output
            </Label>
          </div>

          <div className="flex items-center space-x-2">
            <Checkbox
              id="pretty-json"
              checked={config.output.pretty_json}
              onCheckedChange={handlePrettyJsonChange}
            />
            <Label htmlFor="pretty-json" className="cursor-pointer">
              Pretty-print JSON output
            </Label>
          </div>
        </div>
      </CardContent>
    </Card>
  );
}
