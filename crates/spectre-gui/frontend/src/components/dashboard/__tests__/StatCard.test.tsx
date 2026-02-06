import { describe, it, expect } from 'vitest';
import { render, screen } from '@testing-library/react';
import { StatCard } from '../StatCard';
import { Server } from 'lucide-react';

describe('StatCard', () => {
  it('renders title and value correctly', () => {
    render(
      <StatCard
        title="Total Hosts"
        value={42}
        icon={<Server data-testid="server-icon" />}
      />
    );

    expect(screen.getByText('Total Hosts')).toBeInTheDocument();
    expect(screen.getByText('42')).toBeInTheDocument();
    expect(screen.getByTestId('server-icon')).toBeInTheDocument();
  });

  it('renders loading skeleton when loading prop is true', () => {
    render(
      <StatCard
        title="Total Hosts"
        value={0}
        icon={<Server />}
        loading={true}
      />
    );

    expect(screen.getByText('Total Hosts')).toBeInTheDocument();
    const skeleton = screen.getByText('Total Hosts').parentElement?.parentElement;
    expect(skeleton?.querySelector('.animate-pulse')).toBeInTheDocument();
  });

  it('applies correct variant colors', () => {
    const { rerender } = render(
      <StatCard
        title="Findings"
        value={10}
        icon={<Server />}
        variant="critical"
      />
    );

    let valueElement = screen.getByText('10');
    expect(valueElement).toHaveClass('text-red-600');

    rerender(
      <StatCard
        title="Findings"
        value={10}
        icon={<Server />}
        variant="default"
      />
    );

    valueElement = screen.getByText('10');
    expect(valueElement).toHaveClass('text-gray-900');
  });

  it('displays trend indicator when provided', () => {
    render(
      <StatCard
        title="Total Hosts"
        value={50}
        icon={<Server />}
        trend={{ value: 15, isPositive: true }}
      />
    );

    expect(screen.getByText('15%')).toBeInTheDocument();
    expect(screen.getByText('from last scan')).toBeInTheDocument();
  });

  it('formats large numbers with locale separators', () => {
    render(
      <StatCard
        title="Open Ports"
        value={1234567}
        icon={<Server />}
      />
    );

    expect(screen.getByText('1,234,567')).toBeInTheDocument();
  });
});
