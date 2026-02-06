/**
 * Keyboard shortcuts configuration for SPECTRE GUI
 *
 * Provides centralized keyboard shortcut definitions for:
 * - Navigation between pages (Alt+1-5)
 * - Action shortcuts (Ctrl+N, Ctrl+S, Ctrl+F)
 * - Help access (F1)
 */

export interface KeyboardShortcut {
  key: string;
  description: string;
  action?: string;
}

export interface ShortcutCategory {
  [key: string]: KeyboardShortcut;
}

export interface ShortcutConfig {
  navigation: ShortcutCategory;
  actions: ShortcutCategory;
  help: ShortcutCategory;
}

export const KEYBOARD_SHORTCUTS: ShortcutConfig = {
  navigation: {
    dashboard: {
      key: 'Alt+1',
      description: 'Go to Dashboard',
      action: '/dashboard',
    },
    recon: {
      key: 'Alt+2',
      description: 'Go to Reconnaissance',
      action: '/recon',
    },
    campaigns: {
      key: 'Alt+3',
      description: 'Go to Campaigns',
      action: '/campaigns',
    },
    reports: {
      key: 'Alt+4',
      description: 'Go to Reports',
      action: '/reports',
    },
    settings: {
      key: 'Alt+5',
      description: 'Go to Settings',
      action: '/settings',
    },
  },
  actions: {
    newScan: {
      key: 'Ctrl+N',
      description: 'New Scan',
    },
    save: {
      key: 'Ctrl+S',
      description: 'Save current work',
    },
    search: {
      key: 'Ctrl+F',
      description: 'Search current page',
    },
  },
  help: {
    openHelp: {
      key: 'F1',
      description: 'Open help documentation',
    },
  },
};

/**
 * Parse keyboard event to check if it matches a shortcut
 */
export function matchesShortcut(
  event: KeyboardEvent,
  shortcut: KeyboardShortcut
): boolean {
  const parts = shortcut.key.split('+');
  const key = parts[parts.length - 1].toLowerCase();
  const modifiers = parts.slice(0, -1);

  // Check key match
  if (event.key.toLowerCase() !== key.toLowerCase()) {
    return false;
  }

  // Check modifiers
  const hasAlt = modifiers.includes('Alt');
  const hasCtrl = modifiers.includes('Ctrl');
  const hasShift = modifiers.includes('Shift');

  return (
    event.altKey === hasAlt &&
    event.ctrlKey === hasCtrl &&
    event.shiftKey === hasShift
  );
}

/**
 * Get all shortcuts as a flat array for display
 */
export function getAllShortcuts(): Array<{
  category: string;
  shortcuts: Array<{ name: string; shortcut: KeyboardShortcut }>;
}> {
  return [
    {
      category: 'Navigation',
      shortcuts: Object.entries(KEYBOARD_SHORTCUTS.navigation).map(
        ([name, shortcut]) => ({ name, shortcut })
      ),
    },
    {
      category: 'Actions',
      shortcuts: Object.entries(KEYBOARD_SHORTCUTS.actions).map(
        ([name, shortcut]) => ({ name, shortcut })
      ),
    },
    {
      category: 'Help',
      shortcuts: Object.entries(KEYBOARD_SHORTCUTS.help).map(
        ([name, shortcut]) => ({ name, shortcut })
      ),
    },
  ];
}
