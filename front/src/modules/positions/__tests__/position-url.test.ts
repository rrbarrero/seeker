import { describe, it, expect } from "vitest";
import { PositionUrl } from "../domain/value-objects/position-url";

describe("PositionUrl Value Object", () => {
    it("should create a valid URL", () => {
        const url = "https://example.com/job/1";
        const positionUrl = new PositionUrl(url);
        expect(positionUrl.value).toBe(url);
    });

    it("should allow an empty string", () => {
        const url = "";
        const positionUrl = new PositionUrl(url);
        expect(positionUrl.value).toBe(url);
    });

    it("should throw an error for an invalid URL format", () => {
        const invalidUrl = "not-a-url";
        expect(() => new PositionUrl(invalidUrl)).toThrow("Invalid URL format");
    });

    it("should be equal to another PositionUrl with the same value", () => {
        const url1 = new PositionUrl("https://example.com");
        const url2 = new PositionUrl("https://example.com");
        const url3 = new PositionUrl("https://other.com");

        expect(url1.equals(url2)).toBe(true);
        expect(url1.equals(url3)).toBe(false);
    });
});
