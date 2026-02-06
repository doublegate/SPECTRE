import { useSettings } from '@/hooks/useSettings';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Label } from '@/components/ui/label';
import { Checkbox } from '@/components/ui/checkbox';
import { Input } from '@/components/ui/input';

export function ChefSettings() {
  const { config, loading, updateConfig } = useSettings();

  if (loading || !config) {
    return <div className="text-muted-foreground">Loading settings...</div>;
  }

  const handleDockerImageChange = async (e: React.ChangeEvent<HTMLInputElement>) => {
    await updateConfig({
      chef: {
        ...config.chef,
        docker_image: e.target.value,
      },
    });
  };

  const handleContainerNameChange = async (e: React.ChangeEvent<HTMLInputElement>) => {
    await updateConfig({
      chef: {
        ...config.chef,
        container_name: e.target.value,
      },
    });
  };

  const handleAutoStartChange = async (checked: boolean) => {
    await updateConfig({
      chef: {
        ...config.chef,
        auto_start: checked,
      },
    });
  };

  const handleTimeoutChange = async (e: React.ChangeEvent<HTMLInputElement>) => {
    const value = parseInt(e.target.value);
    if (!isNaN(value)) {
      await updateConfig({
        chef: {
          ...config.chef,
          timeout: value,
        },
      });
    }
  };

  return (
    <Card>
      <CardHeader>
        <CardTitle>Analysis Settings</CardTitle>
        <CardDescription>Configure CyberChef-MCP data analysis engine</CardDescription>
      </CardHeader>
      <CardContent className="space-y-4">
        <div className="space-y-2">
          <Label htmlFor="docker-image">Docker Image</Label>
          <Input
            id="docker-image"
            value={config.chef.docker_image}
            onChange={handleDockerImageChange}
            placeholder="doublegate/cyberchef-mcp:latest"
          />
          <p className="text-xs text-muted-foreground">
            Docker image to use for CyberChef-MCP container
          </p>
        </div>

        <div className="space-y-2">
          <Label htmlFor="container-name">Container Name</Label>
          <Input
            id="container-name"
            value={config.chef.container_name}
            onChange={handleContainerNameChange}
            placeholder="spectre-cyberchef"
          />
        </div>

        <div className="space-y-2">
          <Label htmlFor="timeout">Timeout (seconds)</Label>
          <Input
            id="timeout"
            type="number"
            value={config.chef.timeout}
            onChange={handleTimeoutChange}
            min={5}
            max={300}
          />
        </div>

        <div className="flex items-center space-x-2">
          <Checkbox
            id="auto-start"
            checked={config.chef.auto_start}
            onCheckedChange={handleAutoStartChange}
          />
          <Label htmlFor="auto-start" className="cursor-pointer">
            Auto-start container if not running
          </Label>
        </div>
      </CardContent>
    </Card>
  );
}
