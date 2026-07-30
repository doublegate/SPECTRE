import { memo } from 'react';
import { PieChart, Pie, Cell, ResponsiveContainer, Legend, Tooltip } from 'recharts';
import { SEVERITY_COLORS } from '@/types/dashboard';

export interface SeverityChartProps {
  data: {
    critical: number;
    high: number;
    medium: number;
    low: number;
    info: number;
  };
}

interface ChartDataPoint {
  name: string;
  value: number;
  color: string;
}

export const SeverityChart = memo(function SeverityChart({ data }: SeverityChartProps) {
  const chartData: ChartDataPoint[] = [
    { name: 'Critical', value: data.critical, color: SEVERITY_COLORS.critical },
    { name: 'High', value: data.high, color: SEVERITY_COLORS.high },
    { name: 'Medium', value: data.medium, color: SEVERITY_COLORS.medium },
    { name: 'Low', value: data.low, color: SEVERITY_COLORS.low },
    { name: 'Info', value: data.info, color: SEVERITY_COLORS.info },
  ].filter(item => item.value > 0);

  const total = chartData.reduce((sum, item) => sum + item.value, 0);

  if (total === 0) {
    return (
      <div className="flex items-center justify-center h-[300px] text-muted-foreground">
        <div className="text-center">
          <p className="text-lg font-medium">No findings yet</p>
          <p className="text-sm">Start a scan to see severity distribution</p>
        </div>
      </div>
    );
  }

  const renderCustomLabel = (entry: any) => {
    const percent = ((entry.value / total) * 100).toFixed(1);
    return `${entry.name}: ${percent}%`;
  };

  return (
    <ResponsiveContainer width="100%" height={300}>
      <PieChart>
        <Pie
          data={chartData}
          cx="50%"
          cy="50%"
          labelLine={false}
          label={renderCustomLabel}
          outerRadius={80}
          fill="#8884d8"
          dataKey="value"
          animationDuration={800}
        >
          {chartData.map((entry, index) => (
            <Cell key={`cell-${index}`} fill={entry.color} />
          ))}
        </Pie>
        <Tooltip
          formatter={(value, name) => {
            const numValue = typeof value === 'number' ? value : Number(value);
            const percent = ((numValue / total) * 100).toFixed(1);
            return [`${numValue} (${percent}%)`, name];
          }}
        />
        <Legend
          verticalAlign="bottom"
          height={36}
          formatter={(value) => {
            const item = chartData.find(d => d.name === value);
            return `${value}: ${item?.value || 0}`;
          }}
        />
      </PieChart>
    </ResponsiveContainer>
  );
}, (prevProps, nextProps) => {
  // Re-render only if data values change
  return (
    prevProps.data.critical === nextProps.data.critical &&
    prevProps.data.high === nextProps.data.high &&
    prevProps.data.medium === nextProps.data.medium &&
    prevProps.data.low === nextProps.data.low &&
    prevProps.data.info === nextProps.data.info
  );
});
