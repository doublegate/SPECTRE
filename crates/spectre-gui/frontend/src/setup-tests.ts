import "@testing-library/jest-dom/vitest";

// Mock ResizeObserver for Recharts
globalThis.ResizeObserver = class ResizeObserver {
  observe() {}
  unobserve() {}
  disconnect() {}
};
