import { describe, it, expect, vi } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/react';
import { ActivityTimeline, ScanActivity } from '../ActivityTimeline';

describe('ActivityTimeline', () => {
  const mockActivities: ScanActivity[] = [
    {
      id: '1',
      type: 'scan_completed',
      timestamp: new Date().toISOString(),
      details: {
        targetCount: 5,
        duration: 120,
        hostsScanned: 10,
      },
    },
    {
      id: '2',
      type: 'scan_started',
      timestamp: new Date(Date.now() - 60000).toISOString(),
      details: {
        targetCount: 3,
      },
    },
  ];

  it('renders activities correctly', () => {
    render(<ActivityTimeline activities={mockActivities} />);

    expect(screen.getByText('Scan Completed')).toBeInTheDocument();
    expect(screen.getByText('Scan Started')).toBeInTheDocument();
    expect(screen.getByText('5 targets')).toBeInTheDocument();
    expect(screen.getByText('10 hosts scanned')).toBeInTheDocument();
  });

  it('displays empty state when no activities', () => {
    render(<ActivityTimeline activities={[]} />);

    expect(screen.getByText('No recent activity')).toBeInTheDocument();
    expect(screen.getByText('Start a scan to see activity')).toBeInTheDocument();
  });

  it('respects limit prop', () => {
    const manyActivities = Array(10).fill(null).map((_, i) => ({
      id: String(i),
      type: 'scan_completed' as const,
      timestamp: new Date().toISOString(),
      details: {},
    }));

    render(<ActivityTimeline activities={manyActivities} limit={3} />);

    // Should only render 3 activities
    const activityElements = screen.getAllByText('Scan Completed');
    expect(activityElements).toHaveLength(3);
  });

  it('calls onActivityClick when activity is clicked', () => {
    const handleClick = vi.fn();
    render(
      <ActivityTimeline
        activities={mockActivities}
        onActivityClick={handleClick}
      />
    );

    const firstActivity = screen.getByText('Scan Completed').parentElement?.parentElement;
    if (firstActivity) {
      fireEvent.click(firstActivity);
      expect(handleClick).toHaveBeenCalledWith(mockActivities[0]);
    }
  });

  it('formats duration correctly', () => {
    const activity: ScanActivity = {
      id: '1',
      type: 'scan_completed',
      timestamp: new Date().toISOString(),
      details: {
        duration: 3665, // 1h 1m 5s
      },
    };

    render(<ActivityTimeline activities={[activity]} />);

    expect(screen.getByText(/Duration: 1h 1m/)).toBeInTheDocument();
  });
});
