import { useEffect } from "react";
import { Outlet, useNavigate } from "react-router";
import { Sidebar } from "./Sidebar";
import { Header } from "./Header";
import { StatusBar } from "./StatusBar";
import { ErrorBoundary } from "@/components/ErrorBoundary";
import { useUiStore } from "@/stores/uiStore";
import { cn } from "@/lib/utils";
import { KEYBOARD_SHORTCUTS, matchesShortcut } from "@/config/shortcuts";

export function MainLayout() {
  const sidebarCollapsed = useUiStore((s) => s.sidebarCollapsed);
  const navigate = useNavigate();

  // Global keyboard shortcuts handler
  useEffect(() => {
    const handleKeyboard = (e: KeyboardEvent) => {
      // Navigation shortcuts (Alt+1-5)
      Object.entries(KEYBOARD_SHORTCUTS.navigation).forEach(([_, shortcut]) => {
        if (matchesShortcut(e, shortcut) && shortcut.action) {
          e.preventDefault();
          navigate(shortcut.action);
        }
      });

      // Help shortcut (F1)
      if (matchesShortcut(e, KEYBOARD_SHORTCUTS.help.openHelp)) {
        e.preventDefault();
        // TODO: Open help dialog or navigate to help page
        console.log('Help requested (F1)');
      }

      // Action shortcuts
      if (matchesShortcut(e, KEYBOARD_SHORTCUTS.actions.newScan)) {
        e.preventDefault();
        navigate('/recon');
        // TODO: Trigger new scan dialog
      }
    };

    window.addEventListener('keydown', handleKeyboard);
    return () => window.removeEventListener('keydown', handleKeyboard);
  }, [navigate]);

  return (
    <div className="flex h-screen w-screen overflow-hidden bg-background text-foreground">
      <Sidebar />
      <div
        className={cn(
          "flex flex-1 flex-col transition-all duration-200",
          sidebarCollapsed ? "ml-16" : "ml-56",
        )}
      >
        <Header />
        <main className="flex-1 overflow-auto p-4">
          <ErrorBoundary>
            <Outlet />
          </ErrorBoundary>
        </main>
        <StatusBar />
      </div>
    </div>
  );
}
