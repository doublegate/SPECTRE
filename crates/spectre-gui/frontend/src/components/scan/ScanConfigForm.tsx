import { useState } from "react";
import { Play, AlertCircle } from "lucide-react";
import { ScanType, type ScanRequest } from "@/types/scan";

interface ScanConfigFormProps {
  onScanStart: (request: ScanRequest) => void;
  loading?: boolean;
}

const SCAN_TYPES = [
  { value: ScanType.Syn, label: "SYN Scan (Stealth)", description: "Fast, stealthy" },
  { value: ScanType.Connect, label: "Connect Scan", description: "Full TCP handshake, reliable" },
  { value: ScanType.Udp, label: "UDP Scan", description: "Scan UDP ports" },
  { value: ScanType.Ack, label: "ACK Scan", description: "Firewall detection" },
  { value: ScanType.Fin, label: "FIN Scan", description: "Stealth scan variant" },
  { value: ScanType.Xmas, label: "Xmas Scan", description: "FIN/PSH/URG flags" },
  { value: ScanType.Null, label: "Null Scan", description: "No flags set" },
  { value: ScanType.Window, label: "Window Scan", description: "Window size analysis" },
];

const TIMING_TEMPLATES = [
  { value: 0, label: "T0 (Paranoid)", description: "Very slow, IDS evasion" },
  { value: 1, label: "T1 (Sneaky)", description: "Slow, IDS evasion" },
  { value: 2, label: "T2 (Polite)", description: "Less bandwidth" },
  { value: 3, label: "T3 (Normal)", description: "Default timing" },
  { value: 4, label: "T4 (Aggressive)", description: "Fast scan" },
  { value: 5, label: "T5 (Insane)", description: "Very fast" },
];

export function ScanConfigForm({ onScanStart, loading = false }: ScanConfigFormProps) {
  const [targets, setTargets] = useState("");
  const [ports, setPorts] = useState("22,80,443,8080,8443");
  const [scanType, setScanType] = useState<string>(ScanType.Connect);
  const [timing, setTiming] = useState(3);
  const [serviceDetection, setServiceDetection] = useState(true);
  const [osDetection, setOsDetection] = useState(false);
  const [error, setError] = useState("");

  const validateTargets = (input: string): boolean => {
    if (!input.trim()) {
      setError("At least one target is required");
      return false;
    }

    const targetList = input
      .split(/[\n,]/)
      .map((t) => t.trim())
      .filter((t) => t);

    // Basic validation (IP, CIDR, hostname)
    const validPattern = /^(\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}(\/\d{1,2})?|[a-zA-Z0-9.-]+)$/;
    const invalid = targetList.filter((t) => !validPattern.test(t));

    if (invalid.length > 0) {
      setError(`Invalid targets: ${invalid.join(", ")}`);
      return false;
    }

    setError("");
    return true;
  };

  const validatePorts = (input: string): boolean => {
    if (!input.trim()) return true; // Empty = use defaults

    // Check format: numbers, commas, dashes only
    if (!/^[\d,\- ]+$/.test(input)) {
      setError("Ports must be numbers, commas, or ranges (e.g., 22,80,443 or 1-1000)");
      return false;
    }

    setError("");
    return true;
  };

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();

    if (!validateTargets(targets) || !validatePorts(ports)) {
      return;
    }

    const targetList = targets
      .split(/[\n,]/)
      .map((t) => t.trim())
      .filter((t) => t);

    const request: ScanRequest = {
      targets: targetList,
      ports: ports.trim() || undefined,
      scan_type: scanType,
      timing,
      service_detection: serviceDetection,
      os_detection: osDetection,
    };

    onScanStart(request);
  };

  return (
    <form onSubmit={handleSubmit} className="space-y-4" aria-label="Scan configuration form">
      <div>
        <label htmlFor="scan-targets" className="mb-1.5 block text-sm font-medium text-foreground">
          Targets
          <span aria-hidden="true"> *</span>
          <span className="sr-only">required</span>
        </label>
        <textarea
          id="scan-targets"
          value={targets}
          onChange={(e) => setTargets(e.target.value)}
          placeholder="10.0.0.1&#10;192.168.1.0/24&#10;example.com"
          className="h-24 w-full resize-none rounded-md border border-border bg-background px-3 py-2 text-sm text-foreground placeholder:text-muted-foreground focus:border-primary focus:outline-none focus:ring-1 focus:ring-primary"
          disabled={loading}
          aria-required="true"
          aria-describedby="targets-help"
          aria-invalid={!!error && error.includes("target")}
        />
        <p id="targets-help" className="mt-1 text-xs text-muted-foreground">
          Comma or newline separated. Supports IP, CIDR, hostname.
        </p>
      </div>

      <div>
        <label htmlFor="scan-ports" className="mb-1.5 block text-sm font-medium text-foreground">
          Ports
        </label>
        <input
          id="scan-ports"
          type="text"
          value={ports}
          onChange={(e) => setPorts(e.target.value)}
          placeholder="22,80,443,8080 or 1-1000"
          className="w-full rounded-md border border-border bg-background px-3 py-2 text-sm text-foreground placeholder:text-muted-foreground focus:border-primary focus:outline-none focus:ring-1 focus:ring-primary"
          disabled={loading}
          aria-describedby="ports-help"
          aria-invalid={!!error && error.includes("Ports")}
        />
        <p id="ports-help" className="mt-1 text-xs text-muted-foreground">
          Comma separated or ranges. Leave empty for common ports.
        </p>
      </div>

      <div>
        <label htmlFor="scan-type" className="mb-1.5 block text-sm font-medium text-foreground">
          Scan Type
        </label>
        <select
          id="scan-type"
          value={scanType}
          onChange={(e) => setScanType(e.target.value)}
          className="w-full rounded-md border border-border bg-background px-3 py-2 text-sm text-foreground focus:border-primary focus:outline-none focus:ring-1 focus:ring-primary"
          disabled={loading}
          aria-label="Select scan type"
        >
          {SCAN_TYPES.map((type) => (
            <option key={type.value} value={type.value}>
              {type.label} — {type.description}
            </option>
          ))}
        </select>
      </div>

      <div>
        <label htmlFor="scan-timing" className="mb-1.5 block text-sm font-medium text-foreground">
          Timing Template
        </label>
        <select
          id="scan-timing"
          value={timing}
          onChange={(e) => setTiming(Number(e.target.value))}
          className="w-full rounded-md border border-border bg-background px-3 py-2 text-sm text-foreground focus:border-primary focus:outline-none focus:ring-1 focus:ring-primary"
          disabled={loading}
          aria-label="Select timing template"
        >
          {TIMING_TEMPLATES.map((template) => (
            <option key={template.value} value={template.value}>
              {template.label} — {template.description}
            </option>
          ))}
        </select>
      </div>

      <div className="space-y-2" role="group" aria-labelledby="detection-options-label">
        <div id="detection-options-label" className="sr-only">Detection options</div>
        <label className="flex items-center gap-2 text-sm">
          <input
            id="service-detection"
            type="checkbox"
            checked={serviceDetection}
            onChange={(e) => setServiceDetection(e.target.checked)}
            className="h-4 w-4 rounded border-border text-primary focus:ring-primary"
            disabled={loading}
            aria-describedby="service-detection-desc"
          />
          <span className="text-foreground">Service Detection</span>
          <span id="service-detection-desc" className="text-muted-foreground">(identify services/versions)</span>
        </label>
        <label className="flex items-center gap-2 text-sm">
          <input
            id="os-detection"
            type="checkbox"
            checked={osDetection}
            onChange={(e) => setOsDetection(e.target.checked)}
            className="h-4 w-4 rounded border-border text-primary focus:ring-primary"
            disabled={loading}
            aria-describedby="os-detection-desc"
          />
          <span className="text-foreground">OS Detection</span>
          <span id="os-detection-desc" className="text-muted-foreground">(fingerprint operating system)</span>
        </label>
      </div>

      {error && (
        <div
          role="alert"
          aria-live="assertive"
          className="flex items-center gap-2 rounded-md border border-destructive/50 bg-destructive/10 p-3 text-sm text-destructive"
        >
          <AlertCircle className="h-4 w-4" aria-hidden="true" />
          <span>{error}</span>
        </div>
      )}

      <button
        type="submit"
        disabled={loading}
        className="flex w-full items-center justify-center gap-2 rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground hover:bg-primary/90 focus:outline-none focus:ring-2 focus:ring-primary focus:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50"
        aria-label={loading ? "Scan in progress" : "Start network scan"}
      >
        <Play className="h-4 w-4" aria-hidden="true" />
        {loading ? "Starting Scan..." : "Start Scan"}
      </button>
    </form>
  );
}
