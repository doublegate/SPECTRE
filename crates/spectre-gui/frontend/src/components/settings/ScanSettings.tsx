import { useSettings } from '@/hooks/useSettings';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Label } from '@/components/ui/label';
import { Checkbox } from '@/components/ui/checkbox';
import { TIMING_TEMPLATES, PORT_PRESETS } from '@/types/settings';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select';

export function ScanSettings() {
  const { config, loading, updateConfig } = useSettings();

  if (loading || !config) {
    return <div className="text-muted-foreground">Loading settings...</div>;
  }

  const handleTimingChange = async (value: string) => {
    await updateConfig({
      scan: {
        ...config.scan,
        default_timing: parseInt(value),
      },
    });
  };

  const handlePortsChange = async (value: string) => {
    await updateConfig({
      scan: {
        ...config.scan,
        default_ports: value,
      },
    });
  };

  const handleServiceDetectionChange = async (checked: boolean) => {
    await updateConfig({
      scan: {
        ...config.scan,
        service_detection: checked,
      },
    });
  };

  const handleOsDetectionChange = async (checked: boolean) => {
    await updateConfig({
      scan: {
        ...config.scan,
        os_detection: checked,
      },
    });
  };

  const handleResolveDnsChange = async (checked: boolean) => {
    await updateConfig({
      scan: {
        ...config.scan,
        resolve_dns: checked,
      },
    });
  };

  return (
    <Card>
      <CardHeader>
        <CardTitle>Scan Settings</CardTitle>
        <CardDescription>Configure default scanning behavior (ProRT-IP)</CardDescription>
      </CardHeader>
      <CardContent className="space-y-4">
        <div className="space-y-2">
          <Label htmlFor="timing">Timing Template</Label>
          <Select
            value={config.scan.default_timing.toString()}
            onValueChange={handleTimingChange}
          >
            <SelectTrigger id="timing">
              <SelectValue placeholder="Select timing" />
            </SelectTrigger>
            <SelectContent>
              {TIMING_TEMPLATES.map((template) => (
                <SelectItem key={template.value} value={template.value.toString()}>
                  {template.label} - {template.description}
                </SelectItem>
              ))}
            </SelectContent>
          </Select>
        </div>

        <div className="space-y-2">
          <Label htmlFor="ports">Default Ports</Label>
          <div className="flex gap-2">
            <Select value={config.scan.default_ports} onValueChange={handlePortsChange}>
              <SelectTrigger id="ports" className="w-full">
                <SelectValue placeholder="Select port range" />
              </SelectTrigger>
              <SelectContent>
                {PORT_PRESETS.map((preset) => (
                  <SelectItem key={preset.value} value={preset.value}>
                    {preset.label} - {preset.description}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
          </div>
          <p className="text-xs text-muted-foreground">
            Current: {config.scan.default_ports}
          </p>
        </div>

        <div className="space-y-2">
          <div className="flex items-center space-x-2">
            <Checkbox
              id="service-detection"
              checked={config.scan.service_detection}
              onCheckedChange={handleServiceDetectionChange}
            />
            <Label htmlFor="service-detection" className="cursor-pointer">
              Enable service detection
            </Label>
          </div>

          <div className="flex items-center space-x-2">
            <Checkbox
              id="os-detection"
              checked={config.scan.os_detection}
              onCheckedChange={handleOsDetectionChange}
            />
            <Label htmlFor="os-detection" className="cursor-pointer">
              Enable OS detection
            </Label>
          </div>

          <div className="flex items-center space-x-2">
            <Checkbox
              id="resolve-dns"
              checked={config.scan.resolve_dns}
              onCheckedChange={handleResolveDnsChange}
            />
            <Label htmlFor="resolve-dns" className="cursor-pointer">
              Resolve DNS names
            </Label>
          </div>
        </div>
      </CardContent>
    </Card>
  );
}
