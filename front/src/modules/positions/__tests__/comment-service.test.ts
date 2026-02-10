import { describe, it, expect, beforeEach } from "vitest";
import { CommentService } from "../application/comment-service";
import { Comment, type CreateCommentInput } from "../domain/comment";
import { InMemoryCommentRepository } from "../infrastructure/in-memory-comment-repository";

describe("CommentService", () => {
  let commentService: CommentService;
  let repository: InMemoryCommentRepository;
  let mockComments: Comment[];
  let mockCreateCommentInput: CreateCommentInput;
  const positionId = "position-1";

  beforeEach(() => {
    mockComments = [
      Comment.fromPrimitives({
        id: "comment-1",
        positionId,
        userId: "user-1",
        body: "Screening call done",
        createdAt: "2024-01-15T10:00:00Z",
        updatedAt: "2024-01-15T10:00:00Z",
      }),
      Comment.fromPrimitives({
        id: "comment-2",
        positionId,
        userId: "user-1",
        body: "Technical interview scheduled",
        createdAt: "2024-01-16T10:00:00Z",
        updatedAt: "2024-01-16T10:00:00Z",
      }),
    ];

    mockCreateCommentInput = {
      body: "Offer received",
    };

    repository = new InMemoryCommentRepository(mockComments);
    commentService = new CommentService(repository);
  });

  describe("getComments", () => {
    it("should return comments for a position", async () => {
      const result = await commentService.getComments(positionId, "test-token");
      expect(result).toHaveLength(2);
      expect(result[0].positionId).toBe(positionId);
    });

    it("should return empty array when no comments", async () => {
      const emptyRepo = new InMemoryCommentRepository();
      const service = new CommentService(emptyRepo);
      const result = await service.getComments("position-2");
      expect(result).toEqual([]);
    });
  });

  describe("createComment", () => {
    it("should create comment through repository", async () => {
      const result = await commentService.createComment(
        positionId,
        mockCreateCommentInput,
        "test-token",
      );

      expect(result.body).toBe(mockCreateCommentInput.body);

      const allComments = await repository.getComments(positionId);
      expect(allComments).toHaveLength(3);
    });
  });

  describe("updateComment", () => {
    it("should update comment body", async () => {
      const updated = await commentService.updateComment(positionId, "comment-1", {
        body: "Updated note",
      });

      expect(updated.body).toBe("Updated note");

      const allComments = await repository.getComments(positionId);
      expect(allComments.find((c) => c.id === "comment-1")?.body).toBe("Updated note");
    });

    it("should throw error if comment not found", async () => {
      await expect(
        commentService.updateComment(positionId, "missing", { body: "X" }),
      ).rejects.toThrow("Comment not found");
    });
  });

  describe("deleteComment", () => {
    it("should remove comment", async () => {
      await commentService.deleteComment(positionId, "comment-1");

      const allComments = await repository.getComments(positionId);
      expect(allComments).toHaveLength(1);
      expect(allComments.some((c) => c.id === "comment-1")).toBe(false);
    });
  });
});
