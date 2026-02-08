import { describe, it, expect } from "vitest";
import { AppliedDate } from "../domain/value-objects/applied-date";

describe("AppliedDate Value Object", () => {
  it("should create a valid applied date", () => {
    const dateStr = "2024-01-15";
    const appliedDate = new AppliedDate(dateStr);
    expect(appliedDate.value).toBe(dateStr);
  });

  it("should throw an error for an invalid date format", () => {
    const invalidDate = "not-a-date";
    expect(() => new AppliedDate(invalidDate)).toThrow("Invalid date format");
  });

  it("should format the date correctly", () => {
    // Note: toLocaleDateString() output depends on the environment's locale.
    // However, we can at least check it returns a string and handles the date.
    const dateStr = "2024-01-15";
    const appliedDate = new AppliedDate(dateStr);
    const formatted = appliedDate.formatDate();

    expect(typeof formatted).toBe("string");
    expect(formatted.length).toBeGreaterThan(0);
  });

  it("should be equal to another AppliedDate with the same value", () => {
    const date1 = new AppliedDate("2024-01-15");
    const date2 = new AppliedDate("2024-01-15");
    const date3 = new AppliedDate("2024-01-16");

    expect(date1.equals(date2)).toBe(true);
    expect(date1.equals(date3)).toBe(false);
  });

  it("should not be equal to null or undefined", () => {
    const date = new AppliedDate("2024-01-15");
    expect(date.equals(undefined)).toBe(false);
    // @ts-expect-error - testing invalid input
    expect(date.equals(null)).toBe(false);
  });
});
