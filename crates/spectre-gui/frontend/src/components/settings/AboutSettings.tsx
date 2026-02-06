import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Separator } from '@/components/ui/separator';

export function AboutSettings() {
  return (
    <Card>
      <CardHeader>
        <CardTitle>About SPECTRE</CardTitle>
        <CardDescription>Version information and credits</CardDescription>
      </CardHeader>
      <CardContent className="space-y-4">
        <div className="space-y-2">
          <h3 className="text-sm font-medium text-foreground">SPECTRE GUI</h3>
          <p className="text-sm text-muted-foreground">Version: 0.5.0-alpha.6</p>
          <p className="text-sm text-muted-foreground">
            Security Platform for Encrypted Comms, Testing, Enumeration, Recon
          </p>
        </div>

        <Separator />

        <div className="space-y-2">
          <h3 className="text-sm font-medium text-foreground">Component Versions</h3>
          <ul className="space-y-1 text-sm text-muted-foreground">
            <li>ProRT-IP: v1.0.0 - Network reconnaissance (10M+ pps scanning)</li>
            <li>CyberChef-MCP: v1.9.0 - Data analysis (463 operations)</li>
            <li>WRAITH-Protocol: v2.3.7 - Secure communications (10+ Gbps E2EE)</li>
          </ul>
        </div>

        <Separator />

        <div className="space-y-2">
          <h3 className="text-sm font-medium text-foreground">Tech Stack</h3>
          <ul className="space-y-1 text-sm text-muted-foreground">
            <li>Tauri 2.10 - Desktop application framework</li>
            <li>React 19 - UI library</li>
            <li>TypeScript - Type-safe JavaScript</li>
            <li>Tailwind CSS 4 - Styling</li>
            <li>Rust 1.92+ - Backend logic</li>
          </ul>
        </div>

        <Separator />

        <div className="space-y-2">
          <h3 className="text-sm font-medium text-foreground">License</h3>
          <p className="text-sm text-muted-foreground">
            Multi-license project: SPECTRE GUI (MIT), WRAITH (MIT), ProRT-IP (GPLv3),
            CyberChef-MCP (Apache 2.0)
          </p>
        </div>

        <Separator />

        <div className="space-y-2">
          <h3 className="text-sm font-medium text-foreground">Repository</h3>
          <a
            href="https://github.com/doublegate/SPECTRE"
            target="_blank"
            rel="noopener noreferrer"
            className="text-sm text-primary hover:underline"
          >
            github.com/doublegate/SPECTRE
          </a>
        </div>
      </CardContent>
    </Card>
  );
}
