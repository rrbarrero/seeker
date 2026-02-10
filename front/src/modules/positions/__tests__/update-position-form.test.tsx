"use client";

import { render, screen, fireEvent, waitFor } from "@testing-library/react";
import { vi, describe, it, expect, beforeEach } from "vitest";
import type { PositionProps } from "../domain/position";
import { UpdatePositionForm } from "../presentation/components/update-position-form";

const { updatePosition } = vi.hoisted(() => ({
  updatePosition: vi.fn(),
}));

vi.mock("@/modules/positions/composition-root", () => ({
  positionService: {
    updatePosition,
  },
}));

describe("UpdatePositionForm", () => {
  beforeEach(() => {
    updatePosition.mockReset();
  });

  it("submits updated values using the updatePosition flow", async () => {
    const position: PositionProps = {
      id: "pos-1",
      userId: "user-1",
      company: "Acme",
      roleTitle: "Developer",
      description: "Desc",
      appliedOn: "2024-01-01T00:00:00Z",
      url: "https://example.com",
      status: "CvSent",
      createdAt: "2024-01-01T00:00:00Z",
      updatedAt: "2024-01-01T00:00:00Z",
      deletedAt: null,
      deleted: false,
    };

    const onSuccess = vi.fn();

    render(<UpdatePositionForm position={position} onCancel={vi.fn()} onSuccess={onSuccess} />);

    fireEvent.change(screen.getByLabelText("Company"), {
      target: { value: "Updated Co" },
    });
    fireEvent.change(screen.getByLabelText("Role Title"), {
      target: { value: "Updated Role" },
    });
    fireEvent.change(screen.getByLabelText("Applied On"), {
      target: { value: "2024-02-01" },
    });
    fireEvent.change(screen.getByLabelText("Job URL"), {
      target: { value: "https://updated.example" },
    });
    fireEvent.change(screen.getByLabelText("Status"), {
      target: { value: "PhoneScreenScheduled" },
    });
    fireEvent.change(screen.getByLabelText("Description"), {
      target: { value: "Updated description" },
    });

    fireEvent.click(screen.getByRole("button", { name: "Save Changes" }));

    await waitFor(() => {
      expect(updatePosition).toHaveBeenCalledTimes(1);
    });

    expect(updatePosition).toHaveBeenCalledWith("pos-1", {
      company: "Updated Co",
      roleTitle: "Updated Role",
      description: "Updated description",
      appliedOn: new Date("2024-02-01").toUTCString(),
      url: "https://updated.example",
      status: "PhoneScreenScheduled",
    });

    expect(onSuccess).toHaveBeenCalled();
  });
});
