import { AlertTriangle, Server, Wifi, Box } from "lucide-react";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { StatCard } from "@/components/dashboard/StatCard";
import { SeverityChart } from "@/components/dashboard/SeverityChart";
import { ServicesChart } from "@/components/dashboard/ServicesChart";
import { ActivityTimeline } from "@/components/dashboard/ActivityTimeline";
import { useDashboard } from "@/hooks/useDashboard";
import { useNavigate } from "react-router-dom";

export function Dashboard() {
  const { stats, isLoading, error } = useDashboard();
  const navigate = useNavigate();

  if (error) {
    return (
      <div className="flex items-center justify-center h-[50vh]">
        <div className="text-center space-y-2">
          <AlertTriangle className="h-12 w-12 text-destructive mx-auto" />
          <h2 className="text-xl font-semibold">Failed to load dashboard</h2>
          <p className="text-sm text-muted-foreground">{error}</p>
        </div>
      </div>
    );
  }

  if (!stats) {
    return (
      <div className="flex items-center justify-center h-[50vh]">
        <div className="text-center space-y-2">
          <Server className="h-12 w-12 text-muted-foreground mx-auto" />
          <h2 className="text-xl font-semibold">No data available</h2>
          <p className="text-sm text-muted-foreground">Start a scan to populate the dashboard</p>
        </div>
      </div>
    );
  }

  const handleActivityClick = (activity: any) => {
    // Navigate to recon page with scan ID
    navigate(`/recon?scan=${activity.id}`);
  };

  return (
    <div className="space-y-6 p-6">
      {/* Stat Cards */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
        <StatCard
          title="Total Hosts"
          value={stats.total_hosts}
          icon={<Server />}
          loading={isLoading}
        />
        <StatCard
          title="Open Ports"
          value={stats.open_ports}
          icon={<Wifi />}
          loading={isLoading}
        />
        <StatCard
          title="Services"
          value={stats.services_found}
          icon={<Box />}
          loading={isLoading}
        />
        <StatCard
          title="Findings"
          value={Object.values(stats.severity_counts).reduce((a, b) => a + b, 0)}
          icon={<AlertTriangle />}
          variant={Object.values(stats.severity_counts).reduce((a, b) => a + b, 0) > 0 ? 'critical' : 'default'}
          loading={isLoading}
        />
      </div>

      {/* Charts */}
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        <Card>
          <CardHeader>
            <CardTitle>Severity Distribution</CardTitle>
          </CardHeader>
          <CardContent>
            <SeverityChart data={stats.severity_counts} />
          </CardContent>
        </Card>

        <Card>
          <CardHeader>
            <CardTitle>Top Services</CardTitle>
          </CardHeader>
          <CardContent>
            <ServicesChart data={stats.top_services} />
          </CardContent>
        </Card>
      </div>

      {/* Activity Timeline */}
      <Card>
        <CardHeader>
          <CardTitle>Recent Activity</CardTitle>
        </CardHeader>
        <CardContent>
          <ActivityTimeline
            activities={stats.recent_activity.map(a => ({
              id: a.scan_id,
              type: a.status === 'completed' ? 'scan_completed' : a.status === 'failed' ? 'scan_failed' : 'scan_started',
              timestamp: a.timestamp,
              details: {
                targetCount: a.target_count,
                duration: a.duration_ms ? a.duration_ms / 1000 : undefined,
              },
            }))}
            onActivityClick={handleActivityClick}
          />
        </CardContent>
      </Card>
    </div>
  );
}
