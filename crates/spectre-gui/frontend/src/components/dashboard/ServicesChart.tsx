import { memo } from 'react';
import { BarChart, Bar, XAxis, YAxis, CartesianGrid, Tooltip, ResponsiveContainer } from 'recharts';

export interface ServicesChartProps {
  data: Array<{ name: string; count: number }>;
  limit?: number;
}

export const ServicesChart = memo(function ServicesChart({ data, limit = 10 }: ServicesChartProps) {
  const chartData = [...data]
    .sort((a, b) => b.count - a.count)
    .slice(0, limit);

  if (chartData.length === 0) {
    return (
      <div className="flex items-center justify-center h-[300px] text-muted-foreground">
        <div className="text-center">
          <p className="text-lg font-medium">No services detected</p>
          <p className="text-sm">Start a scan to see service distribution</p>
        </div>
      </div>
    );
  }

  return (
    <ResponsiveContainer width="100%" height={300}>
      <BarChart
        data={chartData}
        layout="vertical"
        margin={{ top: 5, right: 30, left: 80, bottom: 5 }}
      >
        <CartesianGrid strokeDasharray="3 3" className="stroke-muted" />
        <XAxis type="number" className="text-xs" />
        <YAxis
          type="category"
          dataKey="name"
          className="text-xs"
          width={70}
        />
        <Tooltip
          contentStyle={{
            backgroundColor: 'hsl(var(--background))',
            border: '1px solid hsl(var(--border))',
            borderRadius: '6px',
          }}
          cursor={{ fill: 'hsl(var(--muted))' }}
        />
        <Bar
          dataKey="count"
          fill="hsl(var(--primary))"
          radius={[0, 4, 4, 0]}
          animationDuration={800}
        />
      </BarChart>
    </ResponsiveContainer>
  );
}, (prevProps, nextProps) => {
  // Re-render only if data array changes (shallow comparison)
  return (
    prevProps.data.length === nextProps.data.length &&
    prevProps.limit === nextProps.limit &&
    prevProps.data.every((item, index) =>
      item.name === nextProps.data[index]?.name &&
      item.count === nextProps.data[index]?.count
    )
  );
});
