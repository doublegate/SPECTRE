import { useState } from "react";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { Label } from "@/components/ui/label";
import { RadioGroup, RadioGroupItem } from "@/components/ui/radio-group";
import { Checkbox } from "@/components/ui/checkbox";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { Download, Loader2 } from "lucide-react";
import { Finding, ExportFormatString, ReportTemplate, FindingFilters } from "@/types/dashboard";

type ExportFormat = ExportFormatString;

export interface ExportPanelProps {
  findings: Finding[];
  onExport: (format: ExportFormat, filters: FindingFilters) => Promise<void>;
}

export function ExportPanel({ findings, onExport }: ExportPanelProps) {
  const [format, setFormat] = useState<ExportFormat>('json');
  const [template, setTemplate] = useState<ReportTemplate>('full');
  const [selectedSeverities, setSelectedSeverities] = useState<string[]>([
    'critical', 'high', 'medium', 'low', 'info'
  ]);
  const [isExporting, setIsExporting] = useState(false);

  const handleSeverityToggle = (severity: string) => {
    setSelectedSeverities(prev =>
      prev.includes(severity)
        ? prev.filter(s => s !== severity)
        : [...prev, severity]
    );
  };

  const handleExport = async () => {
    setIsExporting(true);
    try {
      const filters: FindingFilters = {
        severities: selectedSeverities,
        severity: selectedSeverities as any,  // Support both naming conventions
      };
      await onExport(format, filters);
    } catch (error) {
      console.error('Export failed:', error);
    } finally {
      setIsExporting(false);
    }
  };

  const filteredCount = findings.filter(f =>
    selectedSeverities.includes(f.severity)
  ).length;

  return (
    <Card>
      <CardHeader>
        <CardTitle>Export Findings</CardTitle>
        <CardDescription>
          Export {filteredCount} of {findings.length} findings in various formats
        </CardDescription>
      </CardHeader>
      <CardContent className="space-y-6">
        {/* Format Selection */}
        <div className="space-y-2">
          <Label>Export Format</Label>
          <RadioGroup value={format} onValueChange={(v) => setFormat(v as ExportFormat)}>
            <div className="flex items-center space-x-2">
              <RadioGroupItem value="csv" id="csv" />
              <Label htmlFor="csv" className="font-normal cursor-pointer">CSV (Comma-separated values)</Label>
            </div>
            <div className="flex items-center space-x-2">
              <RadioGroupItem value="json" id="json" />
              <Label htmlFor="json" className="font-normal cursor-pointer">JSON (Machine-readable)</Label>
            </div>
            <div className="flex items-center space-x-2">
              <RadioGroupItem value="xml" id="xml" />
              <Label htmlFor="xml" className="font-normal cursor-pointer">XML (Structured data)</Label>
            </div>
            <div className="flex items-center space-x-2">
              <RadioGroupItem value="html" id="html" />
              <Label htmlFor="html" className="font-normal cursor-pointer">HTML (Web report)</Label>
            </div>
            <div className="flex items-center space-x-2">
              <RadioGroupItem value="markdown" id="markdown" />
              <Label htmlFor="markdown" className="font-normal cursor-pointer">Markdown (Documentation)</Label>
            </div>
          </RadioGroup>
        </div>

        {/* Template Selection (HTML/Markdown only) */}
        {(format === 'html' || format === 'markdown') && (
          <div className="space-y-2">
            <Label>Report Template</Label>
            <Select value={template} onValueChange={(v) => setTemplate(v as ReportTemplate)}>
              <SelectTrigger>
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="executive">Executive Summary</SelectItem>
                <SelectItem value="technical">Technical Report</SelectItem>
                <SelectItem value="full">Full Report (All Details)</SelectItem>
              </SelectContent>
            </Select>
          </div>
        )}

        {/* Severity Filters */}
        <div className="space-y-2">
          <Label>Include Severities</Label>
          <div className="space-y-2">
            {['critical', 'high', 'medium', 'low', 'info'].map((severity) => (
              <div key={severity} className="flex items-center space-x-2">
                <Checkbox
                  id={severity}
                  checked={selectedSeverities.includes(severity)}
                  onCheckedChange={() => handleSeverityToggle(severity)}
                />
                <Label htmlFor={severity} className="font-normal cursor-pointer capitalize">
                  {severity}
                </Label>
              </div>
            ))}
          </div>
        </div>

        {/* Actions */}
        <div className="flex gap-2">
          <Button
            onClick={handleExport}
            disabled={isExporting || filteredCount === 0}
            className="flex-1"
          >
            {isExporting ? (
              <>
                <Loader2 className="h-4 w-4 mr-2 animate-spin" />
                Exporting...
              </>
            ) : (
              <>
                <Download className="h-4 w-4 mr-2" />
                Export {format.toUpperCase()}
              </>
            )}
          </Button>
        </div>
      </CardContent>
    </Card>
  );
}
