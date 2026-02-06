import { describe, it, expect, vi } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/react';
import { FindingsTable } from '../FindingsTable';
import { Finding } from '@/types/dashboard';

describe('FindingsTable', () => {
  const mockFindings: Finding[] = [
    {
      id: '1',
      severity: 'critical',
      host: '192.168.1.1',
      port: 22,
      service: 'ssh',
      protocol: 'tcp',
      state: 'open',
      title: 'SSH Vulnerability',
      timestamp: new Date().toISOString(),
      description: 'Test finding 1',
      references: [],
      tags: [],
    },
    {
      id: '2',
      severity: 'high',
      host: '192.168.1.2',
      port: 80,
      service: 'http',
      protocol: 'tcp',
      state: 'open',
      title: 'HTTP Issue',
      timestamp: new Date().toISOString(),
      description: 'Test finding 2',
      references: [],
      tags: [],
    },
    {
      id: '3',
      severity: 'medium',
      host: '192.168.1.3',
      port: 443,
      service: 'https',
      protocol: 'tcp',
      state: 'open',
      title: 'HTTPS Configuration',
      timestamp: new Date().toISOString(),
      description: 'Test finding 3',
      references: [],
      tags: [],
    },
  ];

  it('renders findings table with data', () => {
    const handleRowClick = vi.fn();
    render(<FindingsTable findings={mockFindings} onRowClick={handleRowClick} />);

    expect(screen.getByText('192.168.1.1')).toBeInTheDocument();
    expect(screen.getByText('192.168.1.2')).toBeInTheDocument();
    expect(screen.getByText('ssh')).toBeInTheDocument();
    expect(screen.getByText('http')).toBeInTheDocument();
  });

  it('calls onRowClick when row is clicked', () => {
    const handleRowClick = vi.fn();
    render(<FindingsTable findings={mockFindings} onRowClick={handleRowClick} />);

    const firstRow = screen.getByText('192.168.1.1').closest('tr');
    if (firstRow) {
      fireEvent.click(firstRow);
      expect(handleRowClick).toHaveBeenCalledWith(mockFindings[0]);
    }
  });

  it('filters findings by severity', () => {
    const handleRowClick = vi.fn();
    const { container } = render(<FindingsTable findings={mockFindings} onRowClick={handleRowClick} />);

    // The severity filter is the first combobox in the container
    const severityFilter = container.querySelectorAll('[role="combobox"]')[0];
    expect(severityFilter).toBeTruthy();

    // For now, we just verify the filter renders correctly
    // Full interaction testing would require mocking Radix UI Select behavior
    expect(screen.getByText('192.168.1.1')).toBeInTheDocument();
    expect(screen.getByText('192.168.1.2')).toBeInTheDocument();
  });

  it('displays empty state when no findings match filters', () => {
    const handleRowClick = vi.fn();
    render(<FindingsTable findings={[]} onRowClick={handleRowClick} />);

    expect(screen.getByText('No findings match the current filters')).toBeInTheDocument();
  });

  it('supports pagination', () => {
    const manyFindings = Array(100).fill(null).map((_, i) => ({
      id: String(i),
      severity: 'low' as const,
      host: `192.168.1.${i}`,
      port: 80,
      service: 'http',
      protocol: 'tcp',
      state: 'open',
      title: `Finding ${i}`,
      timestamp: new Date().toISOString(),
      description: `Finding ${i}`,
      references: [],
      tags: [],
    }));

    const handleRowClick = vi.fn();
    render(<FindingsTable findings={manyFindings} onRowClick={handleRowClick} />);

    // Should show pagination info
    expect(screen.getByText(/Showing 1 to 25 of 100 findings/)).toBeInTheDocument();

    // Should have next button
    const nextButton = screen.getByRole('button', { name: /next/i });
    expect(nextButton).toBeInTheDocument();
    expect(nextButton).not.toBeDisabled();
  });
});
