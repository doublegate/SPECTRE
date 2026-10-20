import { describe, it, expect } from "vitest";
import { render, screen } from "@testing-library/react";
import { AboutSettings } from "./AboutSettings";

describe("AboutSettings", () => {
  it("renders tech stack including Rust 1.99+ requirement", () => {
    render(<AboutSettings />);

    expect(screen.getByText("About SPECTRE")).toBeInTheDocument();
    expect(screen.getByText("Rust 1.99+ - Backend logic")).toBeInTheDocument();
  });
});
