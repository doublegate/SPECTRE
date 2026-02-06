import { Card, CardContent, CardHeader } from "@/components/ui/card";
import { ArrowDown, ArrowUp } from "lucide-react";

export interface StatCardProps {
  title: string;
  value: number | string;
  icon: React.ReactNode;
  trend?: { value: number; isPositive: boolean };
  variant?: 'default' | 'critical' | 'high' | 'medium' | 'low';
  loading?: boolean;
}

const VARIANT_COLORS = {
  default: 'text-gray-900 dark:text-gray-100',
  critical: 'text-red-600 dark:text-red-400',
  high: 'text-orange-600 dark:text-orange-400',
  medium: 'text-yellow-600 dark:text-yellow-400',
  low: 'text-blue-600 dark:text-blue-400',
};

const VARIANT_BG = {
  default: 'bg-gray-50 dark:bg-gray-800',
  critical: 'bg-red-50 dark:bg-red-950',
  high: 'bg-orange-50 dark:bg-orange-950',
  medium: 'bg-yellow-50 dark:bg-yellow-950',
  low: 'bg-blue-50 dark:bg-blue-950',
};

export function StatCard({
  title,
  value,
  icon,
  trend,
  variant = 'default',
  loading = false,
}: StatCardProps) {
  if (loading) {
    return (
      <Card className={VARIANT_BG[variant]}>
        <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
          <h3 className="text-sm font-medium text-muted-foreground">{title}</h3>
          <div className="h-4 w-4 text-muted-foreground">{icon}</div>
        </CardHeader>
        <CardContent>
          <div className="animate-pulse space-y-2">
            <div className="h-8 w-24 bg-gray-300 dark:bg-gray-700 rounded"></div>
            {trend && <div className="h-4 w-16 bg-gray-300 dark:bg-gray-700 rounded"></div>}
          </div>
        </CardContent>
      </Card>
    );
  }

  return (
    <Card className={VARIANT_BG[variant]}>
      <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
        <h3 className="text-sm font-medium text-muted-foreground">{title}</h3>
        <div className={`h-4 w-4 ${VARIANT_COLORS[variant]}`}>{icon}</div>
      </CardHeader>
      <CardContent>
        <div className={`text-2xl font-bold ${VARIANT_COLORS[variant]}`}>
          {typeof value === 'number' ? value.toLocaleString() : value}
        </div>
        {trend && (
          <div className="flex items-center text-xs text-muted-foreground mt-1">
            {trend.isPositive ? (
              <ArrowUp className="h-3 w-3 text-green-600 dark:text-green-400 mr-1" />
            ) : (
              <ArrowDown className="h-3 w-3 text-red-600 dark:text-red-400 mr-1" />
            )}
            <span className={trend.isPositive ? 'text-green-600 dark:text-green-400' : 'text-red-600 dark:text-red-400'}>
              {Math.abs(trend.value)}%
            </span>
            <span className="ml-1">from last scan</span>
          </div>
        )}
      </CardContent>
    </Card>
  );
}
