import { Outlet } from "react-router";
import { Sidebar } from "./Sidebar";
import { Header } from "./Header";
import { StatusBar } from "./StatusBar";
import { ErrorBoundary } from "@/components/ErrorBoundary";
import { useUiStore } from "@/stores/uiStore";
import { cn } from "@/lib/utils";

export function MainLayout() {
  const sidebarCollapsed = useUiStore((s) => s.sidebarCollapsed);

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
