import { CheckCircle2, Circle, XCircle } from 'lucide-react';
import { formatDistanceToNow } from 'date-fns';

export interface ScanActivity {
  id: string;
  type: 'scan_started' | 'scan_completed' | 'scan_failed';
  timestamp: string;
  details: {
    targetCount?: number;
    duration?: number;
    hostsScanned?: number;
  };
}

export interface ActivityTimelineProps {
  activities: ScanActivity[];
  limit?: number;
  onActivityClick?: (activity: ScanActivity) => void;
}

const STATUS_CONFIG = {
  scan_started: {
    icon: Circle,
    color: 'text-blue-600 dark:text-blue-400',
    bg: 'bg-blue-100 dark:bg-blue-950',
    label: 'Scan Started',
  },
  scan_completed: {
    icon: CheckCircle2,
    color: 'text-green-600 dark:text-green-400',
    bg: 'bg-green-100 dark:bg-green-950',
    label: 'Scan Completed',
  },
  scan_failed: {
    icon: XCircle,
    color: 'text-red-600 dark:text-red-400',
    bg: 'bg-red-100 dark:bg-red-950',
    label: 'Scan Failed',
  },
};

function formatDuration(seconds: number): string {
  if (seconds < 60) return `${seconds}s`;
  if (seconds < 3600) return `${Math.floor(seconds / 60)}m ${seconds % 60}s`;
  const hours = Math.floor(seconds / 3600);
  const minutes = Math.floor((seconds % 3600) / 60);
  return `${hours}h ${minutes}m`;
}

export function ActivityTimeline({
  activities,
  limit = 5,
  onActivityClick,
}: ActivityTimelineProps) {
  const displayActivities = activities.slice(0, limit);

  if (displayActivities.length === 0) {
    return (
      <div className="flex items-center justify-center h-[200px] text-muted-foreground">
        <div className="text-center">
          <p className="text-lg font-medium">No recent activity</p>
          <p className="text-sm">Start a scan to see activity</p>
        </div>
      </div>
    );
  }

  return (
    <div className="space-y-4">
      {displayActivities.map((activity, index) => {
        const config = STATUS_CONFIG[activity.type];
        const Icon = config.icon;
        const isLast = index === displayActivities.length - 1;

        return (
          <div
            key={activity.id}
            className={`relative flex gap-4 ${
              onActivityClick ? 'cursor-pointer hover:bg-muted/50 p-2 rounded-lg transition-colors' : ''
            }`}
            onClick={() => onActivityClick?.(activity)}
          >
            {/* Timeline line */}
            {!isLast && (
              <div className="absolute left-5 top-10 bottom-0 w-0.5 bg-border" />
            )}

            {/* Status icon */}
            <div className={`flex-shrink-0 w-10 h-10 rounded-full flex items-center justify-center ${config.bg}`}>
              <Icon className={`h-5 w-5 ${config.color}`} />
            </div>

            {/* Content */}
            <div className="flex-1 min-w-0">
              <div className="flex items-center justify-between">
                <p className="text-sm font-medium">{config.label}</p>
                <p className="text-xs text-muted-foreground">
                  {formatDistanceToNow(new Date(activity.timestamp), { addSuffix: true })}
                </p>
              </div>

              <div className="mt-1 text-sm text-muted-foreground space-y-1">
                {activity.details.targetCount !== undefined && (
                  <p>{activity.details.targetCount} target{activity.details.targetCount !== 1 ? 's' : ''}</p>
                )}
                {activity.details.hostsScanned !== undefined && (
                  <p>{activity.details.hostsScanned} host{activity.details.hostsScanned !== 1 ? 's' : ''} scanned</p>
                )}
                {activity.details.duration !== undefined && (
                  <p>Duration: {formatDuration(activity.details.duration)}</p>
                )}
              </div>
            </div>
          </div>
        );
      })}
    </div>
  );
}
