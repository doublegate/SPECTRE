import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Separator } from "@/components/ui/separator";
import { ExternalLink, Download } from "lucide-react";
import { Finding } from "@/types/dashboard";
import { SEVERITY_COLORS } from "@/types/dashboard";
import { formatDistanceToNow } from "date-fns";

export interface FindingDetailProps {
  finding: Finding | null;
  isOpen: boolean;
  onClose: () => void;
}

export function FindingDetail({ finding, isOpen, onClose }: FindingDetailProps) {
  if (!finding) return null;

  const handleExport = (format: 'json' | 'pdf') => {
    // TODO: Implement export functionality
    const data = format === 'json' ? JSON.stringify(finding, null, 2) : 'PDF export not implemented';
    const blob = new Blob([data], { type: format === 'json' ? 'application/json' : 'application/pdf' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `finding-${finding.id}.${format}`;
    a.click();
    URL.revokeObjectURL(url);
  };

  return (
    <Dialog open={isOpen} onOpenChange={onClose}>
      <DialogContent className="max-w-3xl max-h-[80vh] overflow-y-auto" aria-describedby="finding-description">
        <DialogHeader>
          <div className="flex items-center justify-between">
            <DialogTitle>
              Finding Details: {finding.service} on {finding.host}:{finding.port}
            </DialogTitle>
            <Badge
              variant="outline"
              style={{
                borderColor: SEVERITY_COLORS[finding.severity as keyof typeof SEVERITY_COLORS],
                color: SEVERITY_COLORS[finding.severity as keyof typeof SEVERITY_COLORS],
              }}
              role="status"
              aria-label={`Severity level: ${finding.severity}`}
            >
              {finding.severity.toUpperCase()}
            </Badge>
          </div>
        </DialogHeader>

        <div className="space-y-6" id="finding-description">
          {/* Host Information */}
          <div>
            <h3 className="text-sm font-semibold mb-2">Host Information</h3>
            <dl className="grid grid-cols-2 gap-2 text-sm">
              <dt className="text-muted-foreground">IP Address:</dt>
              <dd className="font-mono">{finding.host}</dd>
              <dt className="text-muted-foreground">Port:</dt>
              <dd>{finding.port}</dd>
              <dt className="text-muted-foreground">Protocol:</dt>
              <dd>{finding.protocol}</dd>
              <dt className="text-muted-foreground">Discovered:</dt>
              <dd>{formatDistanceToNow(new Date(finding.timestamp), { addSuffix: true })}</dd>
            </dl>
          </div>

          <Separator />

          {/* Service Information */}
          <div>
            <h3 className="text-sm font-semibold mb-2">Service Details</h3>
            <dl className="grid grid-cols-2 gap-2 text-sm">
              <dt className="text-muted-foreground">Service:</dt>
              <dd>{finding.service}</dd>
              {finding.version && (
                <>
                  <dt className="text-muted-foreground">Version:</dt>
                  <dd>{finding.version}</dd>
                </>
              )}
            </dl>
          </div>

          <Separator />

          {/* Description */}
          {finding.description && (
            <>
              <div>
                <h3 className="text-sm font-semibold mb-2">Description</h3>
                <p className="text-sm text-muted-foreground">{finding.description}</p>
              </div>
              <Separator />
            </>
          )}

          {/* CVEs */}
          {finding.cves && finding.cves.length > 0 && (
            <>
              <div>
                <h3 className="text-sm font-semibold mb-2">Related CVEs</h3>
                <div className="space-y-2">
                  {finding.cves.map((cve) => (
                    <div key={cve} className="flex items-center justify-between text-sm">
                      <code className="font-mono">{cve}</code>
                      <Button
                        variant="ghost"
                        size="sm"
                        onClick={() => window.open(`https://nvd.nist.gov/vuln/detail/${cve}`, '_blank')}
                      >
                        <ExternalLink className="h-4 w-4 mr-2" />
                        View NVD
                      </Button>
                    </div>
                  ))}
                </div>
              </div>
              <Separator />
            </>
          )}

          {/* Remediation */}
          {finding.remediation && (
            <>
              <div>
                <h3 className="text-sm font-semibold mb-2">Remediation</h3>
                <p className="text-sm text-muted-foreground">{finding.remediation}</p>
              </div>
              <Separator />
            </>
          )}

          {/* Actions */}
          <div className="flex gap-2" role="group" aria-label="Export actions">
            <Button
              variant="outline"
              size="sm"
              onClick={() => handleExport('json')}
              aria-label="Export finding as JSON file"
            >
              <Download className="h-4 w-4 mr-2" aria-hidden="true" />
              Export JSON
            </Button>
            <Button
              variant="outline"
              size="sm"
              onClick={() => handleExport('pdf')}
              aria-label="Export finding as PDF file"
            >
              <Download className="h-4 w-4 mr-2" aria-hidden="true" />
              Export PDF
            </Button>
          </div>
        </div>
      </DialogContent>
    </Dialog>
  );
}
