import { describe, it, expect, vi } from "vitest";
import { debounce } from "./debounce";

describe("debounce", () => {
  it("delays function execution", () => {
    vi.useFakeTimers();
    const fn = vi.fn();
    const debounced = debounce(fn, 100);

    debounced();
    expect(fn).not.toHaveBeenCalled();

    vi.advanceTimersByTime(100);
    expect(fn).toHaveBeenCalledOnce();
    vi.useRealTimers();
  });

  it("resets timer on subsequent calls", () => {
    vi.useFakeTimers();
    const fn = vi.fn();
    const debounced = debounce(fn, 100);

    debounced();
    vi.advanceTimersByTime(50);
    debounced(); // reset timer
    vi.advanceTimersByTime(50);
    expect(fn).not.toHaveBeenCalled();

    vi.advanceTimersByTime(50);
    expect(fn).toHaveBeenCalledOnce();
    vi.useRealTimers();
  });

  it("passes arguments to the original function", () => {
    vi.useFakeTimers();
    const fn = vi.fn();
    const debounced = debounce(fn, 100);

    debounced("hello", 42);
    vi.advanceTimersByTime(100);
    expect(fn).toHaveBeenCalledWith("hello", 42);
    vi.useRealTimers();
  });
});
