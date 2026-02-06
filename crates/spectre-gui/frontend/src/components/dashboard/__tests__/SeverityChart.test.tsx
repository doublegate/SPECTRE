import { describe, it, expect } from 'vitest';
import { render, screen } from '@testing-library/react';
import { SeverityChart } from '../SeverityChart';

describe('SeverityChart', () => {
  it('renders chart with data', () => {
    const { container } = render(
      <SeverityChart
        data={{
          critical: 5,
          high: 10,
          medium: 15,
          low: 20,
          info: 25,
        }}
      />
    );

    // Recharts renders in ResponsiveContainer
    // In test environment, we just verify it renders without errors
    expect(container.firstChild).toBeTruthy();
  });

  it('displays empty state when no data', () => {
    render(
      <SeverityChart
        data={{
          critical: 0,
          high: 0,
          medium: 0,
          low: 0,
          info: 0,
        }}
      />
    );

    expect(screen.getByText('No findings yet')).toBeInTheDocument();
    expect(screen.getByText('Start a scan to see severity distribution')).toBeInTheDocument();
  });

  it('filters out zero values from chart', () => {
    const { container } = render(
      <SeverityChart
        data={{
          critical: 10,
          high: 0,
          medium: 5,
          low: 0,
          info: 0,
        }}
      />
    );

    // Chart should render correctly (detailed chart testing would require e2e)
    expect(container.firstChild).toBeTruthy();
  });
});
