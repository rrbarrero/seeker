import { describe, it, expect } from "vitest";
import { Comment, type CommentProps } from "../domain/comment";
import { DomainError } from "@/shared/domain/errors";

describe("Comment Entity", () => {
  const validProps: CommentProps = {
    id: "comment-1",
    positionId: "position-1",
    userId: "user-1",
    body: "Initial interview scheduled",
    createdAt: "2024-01-15T10:00:00Z",
    updatedAt: "2024-01-15T10:00:00Z",
  };

  it("should create a valid comment", () => {
    const comment = Comment.fromPrimitives(validProps);
    expect(comment.id).toBe("comment-1");
    expect(comment.body).toBe("Initial interview scheduled");
  });

  it("should throw DomainError if body is missing", () => {
    const props = { ...validProps, body: "   " };
    expect(() => Comment.fromPrimitives(props)).toThrow(DomainError);
    try {
      Comment.fromPrimitives(props);
    } catch (e) {
      expect(e).toBeInstanceOf(DomainError);
      expect((e as DomainError).code).toBe("MISSING_COMMENT_BODY");
    }
  });
});
