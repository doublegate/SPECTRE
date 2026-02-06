import { useLocation, useNavigate } from "react-router";
import {
  LayoutDashboard,
  Crosshair,
  Radar,
  FlaskConical,
  Radio,
  FlagTriangleRight,
  FileText,
  SettingsIcon,
  PanelLeftClose,
  PanelLeftOpen,
} from "lucide-react";
import { useUiStore } from "@/stores/uiStore";
import { cn } from "@/lib/utils";

const navItems = [
  { path: "/dashboard", label: "Dashboard", icon: LayoutDashboard },
  { path: "/targets", label: "Targets", icon: Crosshair },
  { path: "/recon", label: "Recon", icon: Radar },
  { path: "/analysis", label: "Analysis", icon: FlaskConical },
  { path: "/comms", label: "Comms", icon: Radio },
  { path: "/campaigns", label: "Campaigns", icon: FlagTriangleRight },
  { path: "/reports", label: "Reports", icon: FileText },
  { path: "/settings", label: "Settings", icon: SettingsIcon },
];

export function Sidebar() {
  const location = useLocation();
  const navigate = useNavigate();
  const collapsed = useUiStore((s) => s.sidebarCollapsed);
  const toggleSidebar = useUiStore((s) => s.toggleSidebar);

  return (
    <aside
      className={cn(
        "fixed left-0 top-0 z-30 flex h-screen flex-col border-r border-sidebar-border bg-sidebar-background transition-all duration-200",
        collapsed ? "w-16" : "w-56",
      )}
    >
      {/* Logo */}
      <div className="flex h-14 items-center border-b border-sidebar-border px-4">
        {!collapsed && (
          <span className="text-lg font-bold tracking-wider text-primary">
            SPECTRE
          </span>
        )}
        {collapsed && (
          <span className="mx-auto text-lg font-bold text-primary">S</span>
        )}
      </div>

      {/* Navigation */}
      <nav className="flex-1 space-y-1 p-2">
        {navItems.map(({ path, label, icon: Icon }) => {
          const active = location.pathname === path;
          return (
            <button
              key={path}
              onClick={() => navigate(path)}
              className={cn(
                "flex w-full items-center gap-3 rounded-md px-3 py-2 text-sm transition-colors",
                active
                  ? "bg-sidebar-accent text-sidebar-accent-foreground"
                  : "text-sidebar-foreground hover:bg-sidebar-accent/50",
                collapsed && "justify-center px-2",
              )}
              title={collapsed ? label : undefined}
            >
              <Icon className="h-4 w-4 shrink-0" />
              {!collapsed && <span>{label}</span>}
            </button>
          );
        })}
      </nav>

      {/* Collapse toggle */}
      <div className="border-t border-sidebar-border p-2">
        <button
          onClick={toggleSidebar}
          className="flex w-full items-center justify-center rounded-md p-2 text-sidebar-foreground hover:bg-sidebar-accent/50"
          title={collapsed ? "Expand sidebar" : "Collapse sidebar"}
        >
          {collapsed ? (
            <PanelLeftOpen className="h-4 w-4" />
          ) : (
            <PanelLeftClose className="h-4 w-4" />
          )}
        </button>
      </div>
    </aside>
  );
}
