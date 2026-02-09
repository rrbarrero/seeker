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
    it("should allow valid transitions and update status", () => {
      const position = Position.fromPrimitives({ ...validProps });
      position.advanceStatus("PhoneScreenScheduled");
      expect(position.status).toBe("PhoneScreenScheduled");
      expect(position.updated_at).not.toBe(validProps.updated_at);
    });

    it("should allow transition to Rejected", () => {
      const position = Position.fromPrimitives({ ...validProps });
      position.advanceStatus("Rejected");
      expect(position.status).toBe("Rejected");
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

  describe("update", () => {
    it("should update allowed fields", () => {
      const position = Position.fromPrimitives({ ...validProps });
      position.update({ company: "New Corp", role_title: "Lead Dev" });
      expect(position.company).toBe("New Corp");
      expect(position.role_title).toBe("Lead Dev");
      expect(position.updated_at).not.toBe(validProps.updated_at);
    });

    it("should throw error if update makes entity invalid", () => {
      const position = Position.fromPrimitives({ ...validProps });
      expect(() => position.update({ company: "" })).toThrow(DomainError);
    });

    it("should throw error if trying to update a finalized position", () => {
      const position = Position.fromPrimitives({ ...validProps, status: "Rejected" });
      expect(() => position.update({ company: "New Corp" })).toThrow(DomainError);
    });
  });

  describe("delete", () => {
    it("should mark position as deleted", () => {
      const position = Position.fromPrimitives({ ...validProps });
      position.delete();
      expect(position.deleted).toBe(true);
      expect(position.deleted_at).not.toBeNull();
    });
  });
});
