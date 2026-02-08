import { describe, it, expect } from "vitest";
import { Position, type PositionProps } from "../domain/position";
import { DomainError } from "@/shared/domain/errors";

describe("Position Entity", () => {
  const validProps: PositionProps = {
    id: "1",
    user_id: "user-1",
    company: "Acme Corp",
    role_title: "Developer",
    description: "Cool job",
    applied_on: "2024-01-15",
    url: "https://acme.at/jobs",
    initial_comment: "Applied",
    status: "CvSent",
    created_at: "2024-01-15T10:00:00Z",
    updated_at: "2024-01-15T10:00:00Z",
    deleted_at: null,
    deleted: false,
  };

  it("should create a valid position", () => {
    const position = Position.fromPrimitives(validProps);
    expect(position.id).toBe("1");
    expect(position.company).toBe("Acme Corp");
  });

  it("should throw DomainError if company is missing", () => {
    const props = { ...validProps, company: "" };
    expect(() => Position.fromPrimitives(props)).toThrow(DomainError);
    try {
      Position.fromPrimitives(props);
    } catch (e) {
      expect(e).toBeInstanceOf(DomainError);
      expect((e as DomainError).code).toBe("MISSING_COMPANY");
    }
  });

  it("should throw DomainError if role_title is missing", () => {
    const props = { ...validProps, role_title: " " };
    expect(() => Position.fromPrimitives(props)).toThrow(DomainError);
    try {
      Position.fromPrimitives(props);
    } catch (e) {
      expect(e).toBeInstanceOf(DomainError);
      expect((e as DomainError).code).toBe("MISSING_ROLE_TITLE");
    }
  });

  describe("canBeEdited", () => {
    it("should return true for a normal position", () => {
      const position = Position.fromPrimitives(validProps);
      expect(position.canBeEdited()).toBe(true);
    });

    it("should return false if position is deleted", () => {
      const position = Position.fromPrimitives({ ...validProps, deleted: true });
      expect(position.canBeEdited()).toBe(false);
    });

    it("should return false if position is rejected", () => {
      const position = Position.fromPrimitives({ ...validProps, status: "Rejected" });
      expect(position.canBeEdited()).toBe(false);
    });
  });

  describe("advanceStatus", () => {
    it("should allow valid transitions", () => {
      const position = Position.fromPrimitives(validProps);
      expect(() => position.advanceStatus("PhoneScreenScheduled")).not.toThrow();
    });

    it("should throw DomainError for forbidden transitions (e.g. Reject back to CvSent)", () => {
      const position = Position.fromPrimitives({ ...validProps, status: "Rejected" });
      expect(() => position.advanceStatus("CvSent")).toThrow(DomainError);
      try {
        position.advanceStatus("CvSent");
      } catch (e) {
        expect(e).toBeInstanceOf(DomainError);
        expect((e as DomainError).code).toBe("INVALID_STATUS_TRANSITION");
      }
    });

    it("should throw DomainError for forbidden transitions (e.g. Offer back to CvSent)", () => {
      const position = Position.fromPrimitives({ ...validProps, status: "OfferReceived" });
      expect(() => position.advanceStatus("CvSent")).toThrow(DomainError);
    });
  });
});
