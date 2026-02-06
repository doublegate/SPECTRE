import { describe, it, expect } from "vitest";
import { formatBytes, formatDuration, formatNumber, formatRate } from "./format";

describe("formatBytes", () => {
  it("formats zero bytes", () => {
    expect(formatBytes(0)).toBe("0 B");
  });

  it("formats bytes", () => {
    expect(formatBytes(512)).toBe("512 B");
  });

  it("formats kilobytes", () => {
    expect(formatBytes(1024)).toBe("1 KB");
  });

  it("formats megabytes", () => {
    expect(formatBytes(1048576)).toBe("1 MB");
  });

  it("formats gigabytes", () => {
    expect(formatBytes(1073741824)).toBe("1 GB");
  });
});

describe("formatDuration", () => {
  it("formats milliseconds", () => {
    expect(formatDuration(500)).toBe("500ms");
  });

  it("formats seconds", () => {
    expect(formatDuration(3500)).toBe("3.5s");
  });

  it("formats minutes and seconds", () => {
    expect(formatDuration(90000)).toBe("1m 30s");
  });
});

describe("formatNumber", () => {
  it("formats small numbers", () => {
    expect(formatNumber(42)).toBe("42");
  });
});

describe("formatRate", () => {
  it("formats plain pps", () => {
    expect(formatRate(500)).toBe("500 pps");
  });

  it("formats K pps", () => {
    expect(formatRate(5000)).toBe("5.0K pps");
  });

  it("formats M pps", () => {
    expect(formatRate(10_000_000)).toBe("10.0M pps");
  });
});
