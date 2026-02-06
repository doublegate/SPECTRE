import { useState, useMemo } from "react";
import { ChevronDown, ChevronUp, ChevronsUpDown } from "lucide-react";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import { Badge } from "@/components/ui/badge";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { Finding } from "@/types/dashboard";
import { SEVERITY_COLORS } from "@/types/dashboard";
import { formatDistanceToNow } from "date-fns";

export interface FindingsTableProps {
  findings: Finding[];
  onRowClick: (finding: Finding) => void;
}

type SortField = 'severity' | 'host' | 'port' | 'service' | 'timestamp';
type SortOrder = 'asc' | 'desc';

const SEVERITY_ORDER = { critical: 0, high: 1, medium: 2, low: 3, info: 4 };

export function FindingsTable({ findings, onRowClick }: FindingsTableProps) {
  const [sortField, setSortField] = useState<SortField>('severity');
  const [sortOrder, setSortOrder] = useState<SortOrder>('asc');
  const [severityFilter, setSeverityFilter] = useState<string>('all');
  const [serviceSearch, setServiceSearch] = useState('');
  const [itemsPerPage, setItemsPerPage] = useState(25);
  const [currentPage, setCurrentPage] = useState(1);

  const handleSort = (field: SortField) => {
    if (sortField === field) {
      setSortOrder(sortOrder === 'asc' ? 'desc' : 'asc');
    } else {
      setSortField(field);
      setSortOrder('asc');
    }
  };

  const getSortIcon = (field: SortField) => {
    if (sortField !== field) return <ChevronsUpDown className="h-4 w-4" />;
    return sortOrder === 'asc' ? <ChevronUp className="h-4 w-4" /> : <ChevronDown className="h-4 w-4" />;
  };

  const filteredAndSorted = useMemo(() => {
    let result = [...findings];

    // Apply filters
    if (severityFilter !== 'all') {
      result = result.filter(f => f.severity === severityFilter);
    }
    if (serviceSearch) {
      result = result.filter(f =>
        f.service?.toLowerCase().includes(serviceSearch.toLowerCase()) ?? false
      );
    }

    // Apply sorting
    result.sort((a, b) => {
      let comparison = 0;
      switch (sortField) {
        case 'severity':
          comparison = SEVERITY_ORDER[a.severity as keyof typeof SEVERITY_ORDER] -
                      SEVERITY_ORDER[b.severity as keyof typeof SEVERITY_ORDER];
          break;
        case 'host':
          comparison = a.host.localeCompare(b.host);
          break;
        case 'port':
          comparison = (a.port ?? 0) - (b.port ?? 0);
          break;
        case 'service':
          comparison = (a.service ?? '').localeCompare(b.service ?? '');
          break;
        case 'timestamp':
          comparison = new Date(a.timestamp).getTime() - new Date(b.timestamp).getTime();
          break;
      }
      return sortOrder === 'asc' ? comparison : -comparison;
    });

    return result;
  }, [findings, severityFilter, serviceSearch, sortField, sortOrder]);

  const paginatedData = useMemo(() => {
    const startIndex = (currentPage - 1) * itemsPerPage;
    return filteredAndSorted.slice(startIndex, startIndex + itemsPerPage);
  }, [filteredAndSorted, currentPage, itemsPerPage]);

  const totalPages = Math.ceil(filteredAndSorted.length / itemsPerPage);

  return (
    <div className="space-y-4">
      {/* Filters */}
      <div className="flex flex-col sm:flex-row gap-4">
        <Select value={severityFilter} onValueChange={setSeverityFilter}>
          <SelectTrigger className="w-[180px]">
            <SelectValue placeholder="Filter by severity" />
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="all">All Severities</SelectItem>
            <SelectItem value="critical">Critical</SelectItem>
            <SelectItem value="high">High</SelectItem>
            <SelectItem value="medium">Medium</SelectItem>
            <SelectItem value="low">Low</SelectItem>
            <SelectItem value="info">Info</SelectItem>
          </SelectContent>
        </Select>

        <Input
          placeholder="Search by service..."
          value={serviceSearch}
          onChange={(e) => setServiceSearch(e.target.value)}
          className="max-w-xs"
        />

        <div className="flex-1" />

        <Select value={String(itemsPerPage)} onValueChange={(v) => setItemsPerPage(Number(v))}>
          <SelectTrigger className="w-[120px]">
            <SelectValue />
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="25">25 / page</SelectItem>
            <SelectItem value="50">50 / page</SelectItem>
            <SelectItem value="100">100 / page</SelectItem>
          </SelectContent>
        </Select>
      </div>

      {/* Table */}
      <div className="rounded-md border">
        <Table>
          <TableHeader>
            <TableRow>
              <TableHead onClick={() => handleSort('severity')} className="cursor-pointer">
                <div className="flex items-center gap-2">
                  Severity {getSortIcon('severity')}
                </div>
              </TableHead>
              <TableHead onClick={() => handleSort('host')} className="cursor-pointer">
                <div className="flex items-center gap-2">
                  Host {getSortIcon('host')}
                </div>
              </TableHead>
              <TableHead onClick={() => handleSort('port')} className="cursor-pointer">
                <div className="flex items-center gap-2">
                  Port {getSortIcon('port')}
                </div>
              </TableHead>
              <TableHead onClick={() => handleSort('service')} className="cursor-pointer">
                <div className="flex items-center gap-2">
                  Service {getSortIcon('service')}
                </div>
              </TableHead>
              <TableHead className="hidden md:table-cell">Protocol</TableHead>
              <TableHead onClick={() => handleSort('timestamp')} className="cursor-pointer hidden lg:table-cell">
                <div className="flex items-center gap-2">
                  Timestamp {getSortIcon('timestamp')}
                </div>
              </TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {paginatedData.length === 0 ? (
              <TableRow>
                <TableCell colSpan={6} className="text-center text-muted-foreground">
                  No findings match the current filters
                </TableCell>
              </TableRow>
            ) : (
              paginatedData.map((finding) => (
                <TableRow
                  key={finding.id}
                  className="cursor-pointer hover:bg-muted/50"
                  onClick={() => onRowClick(finding)}
                >
                  <TableCell>
                    <Badge
                      variant="outline"
                      style={{
                        borderColor: SEVERITY_COLORS[finding.severity as keyof typeof SEVERITY_COLORS],
                        color: SEVERITY_COLORS[finding.severity as keyof typeof SEVERITY_COLORS],
                      }}
                    >
                      {finding.severity}
                    </Badge>
                  </TableCell>
                  <TableCell className="font-mono text-sm">{finding.host}</TableCell>
                  <TableCell>{finding.port}</TableCell>
                  <TableCell>{finding.service}</TableCell>
                  <TableCell className="hidden md:table-cell">{finding.protocol}</TableCell>
                  <TableCell className="hidden lg:table-cell text-sm text-muted-foreground">
                    {formatDistanceToNow(new Date(finding.timestamp), { addSuffix: true })}
                  </TableCell>
                </TableRow>
              ))
            )}
          </TableBody>
        </Table>
      </div>

      {/* Pagination */}
      <div className="flex items-center justify-between">
        <p className="text-sm text-muted-foreground">
          Showing {paginatedData.length > 0 ? (currentPage - 1) * itemsPerPage + 1 : 0} to{' '}
          {Math.min(currentPage * itemsPerPage, filteredAndSorted.length)} of{' '}
          {filteredAndSorted.length} findings
        </p>
        <div className="flex gap-2">
          <Button
            variant="outline"
            size="sm"
            onClick={() => setCurrentPage(Math.max(1, currentPage - 1))}
            disabled={currentPage === 1}
          >
            Previous
          </Button>
          <Button
            variant="outline"
            size="sm"
            onClick={() => setCurrentPage(Math.min(totalPages, currentPage + 1))}
            disabled={currentPage === totalPages}
          >
            Next
          </Button>
        </div>
      </div>
    </div>
  );
}
