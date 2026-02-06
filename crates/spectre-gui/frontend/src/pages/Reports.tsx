import { useState, useEffect } from "react";
import { FileText, AlertTriangle } from "lucide-react";
import { Card, CardContent } from "@/components/ui/card";
import { FindingsTable } from "@/components/reports/FindingsTable";
import { FindingDetail } from "@/components/reports/FindingDetail";
import { ExportPanel } from "@/components/reports/ExportPanel";
import { Finding, ExportFormatString, FindingFilters } from "@/types/dashboard";

type ExportFormat = ExportFormatString;
import { invoke } from "@tauri-apps/api/core";

export function Reports() {
  const [findings, setFindings] = useState<Finding[]>([]);
  const [selectedFinding, setSelectedFinding] = useState<Finding | null>(null);
  const [isDetailOpen, setIsDetailOpen] = useState(false);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    loadFindings();
  }, []);

  const loadFindings = async () => {
    try {
      setIsLoading(true);
      setError(null);
      const data = await invoke<Finding[]>('get_findings');
      setFindings(data);
    } catch (err) {
      console.error('Failed to load findings:', err);
      setError(String(err));
    } finally {
      setIsLoading(false);
    }
  };

  const handleRowClick = (finding: Finding) => {
    setSelectedFinding(finding);
    setIsDetailOpen(true);
  };

  const handleExport = async (format: ExportFormat, filters: FindingFilters) => {
    try {
      const filteredFindings = findings.filter(f =>
        filters.severities?.includes(f.severity) ?? true
      );

      await invoke('export_data', {
        findings: filteredFindings,
        format,
      });
    } catch (err) {
      console.error('Export failed:', err);
    }
  };

  if (error) {
    return (
      <div className="flex items-center justify-center h-[50vh]">
        <div className="text-center space-y-2">
          <AlertTriangle className="h-12 w-12 text-destructive mx-auto" />
          <h2 className="text-xl font-semibold">Failed to load findings</h2>
          <p className="text-sm text-muted-foreground">{error}</p>
        </div>
      </div>
    );
  }

  if (isLoading) {
    return (
      <div className="flex items-center justify-center h-[50vh]">
        <div className="text-center space-y-2">
          <FileText className="h-12 w-12 text-muted-foreground mx-auto animate-pulse" />
          <h2 className="text-xl font-semibold">Loading findings...</h2>
        </div>
      </div>
    );
  }

  return (
    <div className="space-y-6 p-6">
      <div className="flex flex-col sm:flex-row justify-between items-start sm:items-center gap-4">
        <div className="flex items-center gap-3">
          <FileText className="h-6 w-6 text-primary" />
          <h1 className="text-3xl font-bold">Reports & Findings</h1>
        </div>

        {findings.length > 0 && (
          <ExportPanel findings={findings} onExport={handleExport} />
        )}
      </div>

      {findings.length === 0 ? (
        <div className="flex items-center justify-center h-[40vh]">
          <div className="text-center space-y-2">
            <FileText className="h-16 w-16 text-muted-foreground mx-auto" />
            <h2 className="text-xl font-semibold">No findings yet</h2>
            <p className="text-sm text-muted-foreground">
              Run a scan to generate findings
            </p>
          </div>
        </div>
      ) : (
        <Card>
          <CardContent className="p-6">
            <FindingsTable findings={findings} onRowClick={handleRowClick} />
          </CardContent>
        </Card>
      )}

      <FindingDetail
        finding={selectedFinding}
        isOpen={isDetailOpen}
        onClose={() => {
          setIsDetailOpen(false);
          setSelectedFinding(null);
        }}
      />
    </div>
  );
}

// Default export for lazy loading
export default Reports;
