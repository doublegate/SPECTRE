import { useSettings } from '@/hooks/useSettings';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Label } from '@/components/ui/label';
import { Checkbox } from '@/components/ui/checkbox';
import { VERBOSITY_LEVELS } from '@/types/settings';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select';

export function GeneralSettings() {
  const { config, loading, updateConfig } = useSettings();

  if (loading || !config) {
    return <div className="text-muted-foreground">Loading settings...</div>;
  }

  const handleVerbosityChange = async (value: string) => {
    await updateConfig({
      general: {
        ...config.general,
        verbosity: parseInt(value),
      },
    });
  };

  const handleColorChange = async (checked: boolean) => {
    await updateConfig({
      general: {
        ...config.general,
        color: checked,
      },
    });
  };

  return (
    <Card>
      <CardHeader>
        <CardTitle>General Settings</CardTitle>
        <CardDescription>Configure general application behavior</CardDescription>
      </CardHeader>
      <CardContent className="space-y-4">
        <div className="space-y-2">
          <Label htmlFor="verbosity">Verbosity Level</Label>
          <Select
            value={config.general.verbosity.toString()}
            onValueChange={handleVerbosityChange}
          >
            <SelectTrigger id="verbosity">
              <SelectValue placeholder="Select verbosity" />
            </SelectTrigger>
            <SelectContent>
              {VERBOSITY_LEVELS.map((level) => (
                <SelectItem key={level.value} value={level.value.toString()}>
                  {level.label} - {level.description}
                </SelectItem>
              ))}
            </SelectContent>
          </Select>
        </div>

        <div className="flex items-center space-x-2">
          <Checkbox
            id="color"
            checked={config.general.color}
            onCheckedChange={handleColorChange}
          />
          <Label htmlFor="color" className="cursor-pointer">
            Enable colored output
          </Label>
        </div>
      </CardContent>
    </Card>
  );
}
