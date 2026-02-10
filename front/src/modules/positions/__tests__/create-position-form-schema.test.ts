import { describe, expect, it } from "vitest";
import { createPositionFormSchema } from "../presentation/hooks/use-create-position-form";
import { POSITION_STATUSES } from "../domain/position";

const validPayload = {
  company: "Acme Corp",
  roleTitle: "Developer",
  description: "Nice role",
  appliedOn: "2024-02-01",
  url: "https://example.com",
  status: "CvSent",
};

describe("createPositionFormSchema", () => {
  it("accepts valid payload with a status from POSITION_STATUSES", () => {
    const result = createPositionFormSchema.safeParse(validPayload);
    expect(result.success).toBe(true);
    if (result.success) {
      expect(POSITION_STATUSES).toContain(result.data.status);
    }
  });

  it("rejects status values outside POSITION_STATUSES", () => {
    const result = createPositionFormSchema.safeParse({
      ...validPayload,
      status: "UnknownStatus",
    });
    expect(result.success).toBe(false);
  });

  it("validates URL using domain rules", () => {
    const ok = createPositionFormSchema.safeParse({
      ...validPayload,
      url: "https://valid.example",
    });
    expect(ok.success).toBe(true);

    const bad = createPositionFormSchema.safeParse({
      ...validPayload,
      url: "not-a-url",
    });
    expect(bad.success).toBe(false);
  });
});
