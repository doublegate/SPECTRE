import { describe, it, expect, vi } from "vitest";
import { render, screen, fireEvent } from "@testing-library/react";
import { ObjectiveList } from "./ObjectiveList";

describe("ObjectiveList", () => {
  it("renders objectives", () => {
    const objectives = ["Objective 1", "Objective 2"];
    render(<ObjectiveList objectives={objectives} editable={false} />);

    expect(screen.getByText("Objective 1")).toBeInTheDocument();
    expect(screen.getByText("Objective 2")).toBeInTheDocument();
  });

  it("shows empty state when no objectives", () => {
    render(<ObjectiveList objectives={[]} editable={false} />);
    expect(screen.getByText("No objectives defined yet.")).toBeInTheDocument();
  });

  it("allows adding objectives when editable", () => {
    const onChange = vi.fn();
    render(<ObjectiveList objectives={[]} editable onChange={onChange} />);

    const input = screen.getByPlaceholderText("Add an objective...");
    fireEvent.change(input, { target: { value: "New objective" } });
    fireEvent.keyDown(input, { key: "Enter" });

    expect(onChange).toHaveBeenCalledWith(["New objective"]);
  });
});
