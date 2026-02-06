import { useEffect, useRef, useState } from "react";
import * as d3 from "d3";
import type { Host, Severity } from "@/types/scan";

interface NetworkNode extends d3.SimulationNodeDatum {
  id: string;
  label: string;
  severity: Severity;
  host: Host;
}

interface NetworkLink extends d3.SimulationLinkDatum<NetworkNode> {
  source: string | NetworkNode;
  target: string | NetworkNode;
}

interface NetworkTopologyProps {
  hosts: Host[];
  onHostClick?: (host: Host) => void;
  className?: string;
}

const SEVERITY_COLORS = {
  critical: "#ef4444",
  high: "#f97316",
  medium: "#eab308",
  low: "#22c55e",
  info: "#6b7280",
};

export function NetworkTopology({ hosts, onHostClick, className = "" }: NetworkTopologyProps) {
  const svgRef = useRef<SVGSVGElement>(null);
  const simulationRef = useRef<d3.Simulation<NetworkNode, NetworkLink> | null>(null);
  const [dimensions, setDimensions] = useState({ width: 800, height: 600 });

  // Handle window resize
  useEffect(() => {
    const handleResize = () => {
      if (svgRef.current) {
        const { width, height } = svgRef.current.getBoundingClientRect();
        setDimensions({ width, height });
      }
    };

    handleResize();
    window.addEventListener("resize", handleResize);
    return () => window.removeEventListener("resize", handleResize);
  }, []);

  // Main D3 visualization effect
  useEffect(() => {
    if (!svgRef.current || hosts.length === 0) return;

    const svg = d3.select(svgRef.current);
    const { width, height } = dimensions;

    // Clear previous content
    svg.selectAll("*").remove();

    // Create nodes from hosts
    const nodes: NetworkNode[] = hosts.map((host) => ({
      id: host.ip,
      label: host.hostname || host.ip,
      severity: host.severity,
      host,
      x: width / 2 + (Math.random() - 0.5) * 100,
      y: height / 2 + (Math.random() - 0.5) * 100,
    }));

    // Create links based on subnet relationships
    const links: NetworkLink[] = [];
    for (let i = 0; i < nodes.length; i++) {
      for (let j = i + 1; j < nodes.length; j++) {
        const subnet1 = nodes[i].id.split(".").slice(0, 3).join(".");
        const subnet2 = nodes[j].id.split(".").slice(0, 3).join(".");
        if (subnet1 === subnet2) {
          links.push({
            source: nodes[i].id,
            target: nodes[j].id,
          });
        }
      }
    }

    // Create SVG container with zoom/pan support
    const g = svg
      .append("g")
      .attr("class", "network-container");

    const zoom = d3.zoom<SVGSVGElement, unknown>()
      .scaleExtent([0.1, 4])
      .on("zoom", (event) => {
        g.attr("transform", event.transform);
      });

    svg.call(zoom);

    // Create link elements
    const linkElements = g
      .append("g")
      .attr("class", "links")
      .selectAll("line")
      .data(links)
      .join("line")
      .attr("stroke", "#4b5563")
      .attr("stroke-opacity", 0.3)
      .attr("stroke-width", 1);

    // Create node groups
    const nodeGroups = g
      .append("g")
      .attr("class", "nodes")
      .selectAll("g")
      .data(nodes)
      .join("g")
      .attr("cursor", "pointer")
      .on("click", (_event, d) => {
        if (onHostClick) {
          onHostClick(d.host);
        }
      });

    // Add circles for nodes
    nodeGroups
      .append("circle")
      .attr("r", 12)
      .attr("fill", (d) => SEVERITY_COLORS[d.severity])
      .attr("stroke", "#fff")
      .attr("stroke-width", 2);

    // Add labels
    nodeGroups
      .append("text")
      .text((d) => d.label)
      .attr("font-size", "10px")
      .attr("dx", 15)
      .attr("dy", 4)
      .attr("fill", "#e5e7eb")
      .style("pointer-events", "none");

    // Add hover effects
    nodeGroups
      .on("mouseenter", function () {
        d3.select(this).select("circle").attr("r", 15).attr("stroke-width", 3);
      })
      .on("mouseleave", function () {
        d3.select(this).select("circle").attr("r", 12).attr("stroke-width", 2);
      });

    // Create force simulation
    const simulation = d3
      .forceSimulation(nodes)
      .force(
        "link",
        d3.forceLink<NetworkNode, NetworkLink>(links)
          .id((d) => d.id)
          .distance(100)
      )
      .force("charge", d3.forceManyBody().strength(-300))
      .force("center", d3.forceCenter(width / 2, height / 2))
      .force("collide", d3.forceCollide().radius(30))
      .alphaDecay(0.05)
      .on("tick", () => {
        linkElements
          .attr("x1", (d) => (d.source as NetworkNode).x ?? 0)
          .attr("y1", (d) => (d.source as NetworkNode).y ?? 0)
          .attr("x2", (d) => (d.target as NetworkNode).x ?? 0)
          .attr("y2", (d) => (d.target as NetworkNode).y ?? 0);

        nodeGroups.attr("transform", (d) => `translate(${d.x ?? 0},${d.y ?? 0})`);
      });

    // Add drag behavior
    const drag = d3
      .drag<SVGGElement, NetworkNode>()
      .on("start", (event, d) => {
        if (!event.active) simulation.alphaTarget(0.3).restart();
        d.fx = d.x;
        d.fy = d.y;
      })
      .on("drag", (event, d) => {
        d.fx = event.x;
        d.fy = event.y;
      })
      .on("end", (event, d) => {
        if (!event.active) simulation.alphaTarget(0);
        d.fx = null;
        d.fy = null;
      });

    nodeGroups.call(drag as any);

    // Store simulation reference for cleanup
    simulationRef.current = simulation;

    // Cleanup
    return () => {
      simulation.stop();
    };
  }, [hosts, dimensions, onHostClick]);

  return (
    <div className={`relative h-full w-full overflow-hidden rounded-lg border border-border bg-card ${className}`}>
      {hosts.length === 0 ? (
        <div className="flex h-full items-center justify-center">
          <p className="text-sm text-muted-foreground">
            No hosts discovered yet. Start a scan to populate the network topology.
          </p>
        </div>
      ) : (
        <>
          <svg
            ref={svgRef}
            className="h-full w-full"
            style={{ background: "transparent" }}
          />
          <div className="absolute bottom-4 right-4 rounded-md border border-border bg-card/90 p-3 text-xs backdrop-blur-sm">
            <div className="mb-1 font-semibold text-foreground">Legend</div>
            <div className="space-y-1">
              {Object.entries(SEVERITY_COLORS).map(([severity, color]) => (
                <div key={severity} className="flex items-center gap-2">
                  <div
                    className="h-3 w-3 rounded-full border border-white"
                    style={{ backgroundColor: color }}
                  />
                  <span className="capitalize text-muted-foreground">{severity}</span>
                </div>
              ))}
            </div>
            <div className="mt-2 border-t border-border pt-2 text-muted-foreground">
              <div>Scroll to zoom</div>
              <div>Drag to pan</div>
              <div>Drag nodes to move</div>
            </div>
          </div>
        </>
      )}
    </div>
  );
}
