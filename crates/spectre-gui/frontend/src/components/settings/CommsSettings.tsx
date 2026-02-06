import { useSettings } from '@/hooks/useSettings';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Label } from '@/components/ui/label';
import { Checkbox } from '@/components/ui/checkbox';
import { Input } from '@/components/ui/input';

export function CommsSettings() {
  const { config, loading, updateConfig } = useSettings();

  if (loading || !config) {
    return <div className="text-muted-foreground">Loading settings...</div>;
  }

  const handleExpirationChange = async (e: React.ChangeEvent<HTMLInputElement>) => {
    const value = parseInt(e.target.value);
    if (!isNaN(value)) {
      await updateConfig({
        comms: {
          ...config.comms,
          default_expiration: value,
        },
      });
    }
  };

  const handleCompressChange = async (checked: boolean) => {
    await updateConfig({
      comms: {
        ...config.comms,
        compress: checked,
      },
    });
  };

  const handleTimeoutChange = async (e: React.ChangeEvent<HTMLInputElement>) => {
    const value = parseInt(e.target.value);
    if (!isNaN(value)) {
      await updateConfig({
        comms: {
          ...config.comms,
          timeout: value,
        },
      });
    }
  };

  return (
    <Card>
      <CardHeader>
        <CardTitle>Communications Settings</CardTitle>
        <CardDescription>Configure WRAITH secure communications</CardDescription>
      </CardHeader>
      <CardContent className="space-y-4">
        <div className="space-y-2">
          <Label htmlFor="expiration">Default Message Expiration (seconds)</Label>
          <Input
            id="expiration"
            type="number"
            value={config.comms.default_expiration}
            onChange={handleExpirationChange}
            min={0}
          />
          <p className="text-xs text-muted-foreground">
            0 = no expiration. Messages will auto-delete after this time.
          </p>
        </div>

        <div className="space-y-2">
          <Label htmlFor="comms-timeout">Connection Timeout (seconds)</Label>
          <Input
            id="comms-timeout"
            type="number"
            value={config.comms.timeout}
            onChange={handleTimeoutChange}
            min={5}
            max={300}
          />
        </div>

        <div className="flex items-center space-x-2">
          <Checkbox
            id="compress"
            checked={config.comms.compress}
            onCheckedChange={handleCompressChange}
          />
          <Label htmlFor="compress" className="cursor-pointer">
            Enable compression for transfers
          </Label>
        </div>
      </CardContent>
    </Card>
  );
}
