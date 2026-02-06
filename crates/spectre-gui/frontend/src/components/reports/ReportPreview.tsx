import { useState } from "react";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import { Button } from "@/components/ui/button";
import { Copy, Printer } from "lucide-react";
import { Light as SyntaxHighlighter } from 'react-syntax-highlighter';
import json from 'react-syntax-highlighter/dist/esm/languages/hljs/json';
import xml from 'react-syntax-highlighter/dist/esm/languages/hljs/xml';
import { docco } from 'react-syntax-highlighter/dist/esm/styles/hljs';
import ReactMarkdown from 'react-markdown';
import DOMPurify from 'dompurify';
import { ReportFormatString } from "@/types/dashboard";

type ReportFormat = ReportFormatString;

// Register languages
SyntaxHighlighter.registerLanguage('json', json);
SyntaxHighlighter.registerLanguage('xml', xml);

export interface ReportPreviewProps {
  content: string;
  format: ReportFormat;
  isOpen: boolean;
  onClose: () => void;
}

export function ReportPreview({ content, format, isOpen, onClose }: ReportPreviewProps) {
  const [copied, setCopied] = useState(false);

  const handleCopy = () => {
    navigator.clipboard.writeText(content);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  const handlePrint = () => {
    // Create a new window for printing
    const printWindow = window.open('', '', 'height=600,width=800');
    if (printWindow) {
      const doc = printWindow.document;

      // Create document structure using DOM methods
      const html = doc.createElement('html');
      const head = doc.createElement('head');
      const title = doc.createElement('title');
      const style = doc.createElement('style');
      const body = doc.createElement('body');

      title.textContent = 'SPECTRE Report';
      style.textContent = `
        body { font-family: system-ui, -apple-system, sans-serif; padding: 20px; }
        pre { background: #f4f4f4; padding: 10px; overflow-x: auto; }
      `;

      head.appendChild(title);
      head.appendChild(style);

      // Safely set content
      const contentDiv = doc.createElement('div');
      if (format === 'html') {
        // Sanitize HTML content before rendering
        const sanitized = DOMPurify.sanitize(content);
        contentDiv.innerHTML = sanitized;
      } else {
        const pre = doc.createElement('pre');
        pre.textContent = content;
        contentDiv.appendChild(pre);
      }
      body.appendChild(contentDiv);

      html.appendChild(head);
      html.appendChild(body);
      doc.appendChild(html);

      doc.close();
      printWindow.print();
    }
  };

  const renderContent = () => {
    switch (format) {
      case 'json':
        return (
          <SyntaxHighlighter language="json" style={docco}>
            {content}
          </SyntaxHighlighter>
        );

      case 'xml':
        return (
          <SyntaxHighlighter language="xml" style={docco}>
            {content}
          </SyntaxHighlighter>
        );

      case 'markdown':
        return (
          <div className="prose dark:prose-invert max-w-none">
            <ReactMarkdown>{content}</ReactMarkdown>
          </div>
        );

      case 'html':
        // Sanitize HTML content before rendering in iframe
        const sanitized = DOMPurify.sanitize(content);
        return (
          <iframe
            srcDoc={sanitized}
            className="w-full h-[600px] border rounded"
            sandbox="allow-same-origin"
            title="HTML Report Preview"
          />
        );

      case 'csv':
        return (
          <pre className="bg-muted p-4 rounded overflow-x-auto text-sm">
            {content}
          </pre>
        );

      default:
        return (
          <pre className="bg-muted p-4 rounded overflow-x-auto text-sm">
            {content}
          </pre>
        );
    }
  };

  return (
    <Dialog open={isOpen} onOpenChange={onClose}>
      <DialogContent className="max-w-5xl max-h-[90vh]">
        <DialogHeader>
          <div className="flex items-center justify-between">
            <DialogTitle>Report Preview ({format.toUpperCase()})</DialogTitle>
            <div className="flex gap-2">
              <Button variant="outline" size="sm" onClick={handleCopy}>
                <Copy className="h-4 w-4 mr-2" />
                {copied ? 'Copied!' : 'Copy'}
              </Button>
              {format === 'html' && (
                <Button variant="outline" size="sm" onClick={handlePrint}>
                  <Printer className="h-4 w-4 mr-2" />
                  Print
                </Button>
              )}
            </div>
          </div>
        </DialogHeader>

        <div className="overflow-y-auto max-h-[calc(90vh-120px)]">
          {renderContent()}
        </div>
      </DialogContent>
    </Dialog>
  );
}
