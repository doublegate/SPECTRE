import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { useUiStore, type ThemeName } from '@/stores/uiStore';
import { cn } from '@/lib/utils';

const themes: { name: ThemeName; label: string; description: string }[] = [
  { name: 'dark', label: 'Dark', description: 'Default dark theme with blue accents' },
  { name: 'light', label: 'Light', description: 'Light theme with midnight blue accents' },
  { name: 'tactical', label: 'Tactical', description: 'Military green on black' },
  { name: 'matrix', label: 'Matrix', description: 'Bright green on black' },
  { name: 'hacker', label: 'Hacker', description: 'Cyan and magenta on dark' },
];

export function ThemeSettings() {
  const { theme, setTheme } = useUiStore();

  return (
    <Card>
      <CardHeader>
        <CardTitle>Theme Settings</CardTitle>
        <CardDescription>Choose your preferred visual theme</CardDescription>
      </CardHeader>
      <CardContent>
        <div className="grid gap-3 sm:grid-cols-2 lg:grid-cols-3">
          {themes.map((t) => (
            <button
              key={t.name}
              onClick={() => setTheme(t.name)}
              className={cn(
                'rounded-lg border p-3 text-left transition-colors',
                theme === t.name
                  ? 'border-primary bg-primary/10'
                  : 'border-border hover:border-muted-foreground'
              )}
            >
              <p className="text-sm font-medium text-foreground">{t.label}</p>
              <p className="text-xs text-muted-foreground">{t.description}</p>
            </button>
          ))}
        </div>
      </CardContent>
    </Card>
  );
}
