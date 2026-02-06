import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from '@/components/ui/table';

interface Shortcut {
  category: string;
  action: string;
  keys: string;
}

const shortcuts: Shortcut[] = [
  { category: 'Global', action: 'Settings', keys: 'Ctrl+,' },
  { category: 'Global', action: 'Help', keys: 'F1' },
  { category: 'Global', action: 'Quit', keys: 'Ctrl+Q' },
  { category: 'Scan', action: 'New Scan', keys: 'Ctrl+N' },
  { category: 'Scan', action: 'Stop Scan', keys: 'Ctrl+S' },
  { category: 'Campaign', action: 'New Campaign', keys: 'Ctrl+Shift+N' },
  { category: 'Export', action: 'Export JSON', keys: 'Ctrl+E' },
  { category: 'Navigation', action: 'Dashboard', keys: 'Alt+1' },
  { category: 'Navigation', action: 'Recon', keys: 'Alt+2' },
  { category: 'Navigation', action: 'Campaigns', keys: 'Alt+3' },
  { category: 'Navigation', action: 'Reports', keys: 'Alt+4' },
];

export function ShortcutsSettings() {
  return (
    <Card>
      <CardHeader>
        <CardTitle>Keyboard Shortcuts</CardTitle>
        <CardDescription>Quick reference for keyboard shortcuts</CardDescription>
      </CardHeader>
      <CardContent>
        <Table>
          <TableHeader>
            <TableRow>
              <TableHead>Category</TableHead>
              <TableHead>Action</TableHead>
              <TableHead>Shortcut</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {shortcuts.map((shortcut, index) => (
              <TableRow key={index}>
                <TableCell className="font-medium">{shortcut.category}</TableCell>
                <TableCell>{shortcut.action}</TableCell>
                <TableCell>
                  <code className="rounded bg-muted px-2 py-1 text-xs">{shortcut.keys}</code>
                </TableCell>
              </TableRow>
            ))}
          </TableBody>
        </Table>
      </CardContent>
    </Card>
  );
}
