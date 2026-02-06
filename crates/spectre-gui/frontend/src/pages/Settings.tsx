import { Settings as SettingsIcon } from 'lucide-react';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';
import { GeneralSettings } from '@/components/settings/GeneralSettings';
import { ScanSettings } from '@/components/settings/ScanSettings';
import { ChefSettings } from '@/components/settings/ChefSettings';
import { CommsSettings } from '@/components/settings/CommsSettings';
import { OutputSettings } from '@/components/settings/OutputSettings';
import { ThemeSettings } from '@/components/settings/ThemeSettings';
import { ShortcutsSettings } from '@/components/settings/ShortcutsSettings';
import { AboutSettings } from '@/components/settings/AboutSettings';

export function Settings() {
  return (
    <div className="space-y-4">
      <div className="flex items-center gap-3">
        <SettingsIcon className="h-5 w-5 text-primary" />
        <h2 className="text-lg font-semibold">Settings</h2>
      </div>

      <Tabs defaultValue="general" className="w-full">
        <TabsList className="grid w-full grid-cols-4 lg:grid-cols-8">
          <TabsTrigger value="general">General</TabsTrigger>
          <TabsTrigger value="scan">Scan</TabsTrigger>
          <TabsTrigger value="chef">Analysis</TabsTrigger>
          <TabsTrigger value="comms">Comms</TabsTrigger>
          <TabsTrigger value="output">Output</TabsTrigger>
          <TabsTrigger value="theme">Theme</TabsTrigger>
          <TabsTrigger value="shortcuts">Shortcuts</TabsTrigger>
          <TabsTrigger value="about">About</TabsTrigger>
        </TabsList>

        <TabsContent value="general" className="mt-4">
          <GeneralSettings />
        </TabsContent>

        <TabsContent value="scan" className="mt-4">
          <ScanSettings />
        </TabsContent>

        <TabsContent value="chef" className="mt-4">
          <ChefSettings />
        </TabsContent>

        <TabsContent value="comms" className="mt-4">
          <CommsSettings />
        </TabsContent>

        <TabsContent value="output" className="mt-4">
          <OutputSettings />
        </TabsContent>

        <TabsContent value="theme" className="mt-4">
          <ThemeSettings />
        </TabsContent>

        <TabsContent value="shortcuts" className="mt-4">
          <ShortcutsSettings />
        </TabsContent>

        <TabsContent value="about" className="mt-4">
          <AboutSettings />
        </TabsContent>
      </Tabs>
    </div>
  );
}
