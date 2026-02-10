import { Comment, type CreateCommentInput, type UpdateCommentInput } from "../domain/comment";
import type { CommentRepository } from "../domain/comment-repository";

export class InMemoryCommentRepository implements CommentRepository {
  private comments: Comment[] = [];

  constructor(initialComments?: Comment[]) {
    if (initialComments) {
      this.comments = [...initialComments];
    }
  }

  async getComments(positionId: string): Promise<Comment[]> {
    return this.comments.filter((comment) => comment.positionId === positionId);
  }

  async createComment(positionId: string, input: CreateCommentInput): Promise<Comment> {
    const comment = Comment.fromPrimitives({
      id: crypto.randomUUID(),
      positionId,
      userId: "in-memory-user",
      body: input.body,
      createdAt: new Date().toISOString(),
      updatedAt: new Date().toISOString(),
    });

    this.comments = [comment, ...this.comments];
    return comment;
  }

  async updateComment(
    positionId: string,
    commentId: string,
    input: UpdateCommentInput,
  ): Promise<Comment> {
    const index = this.comments.findIndex(
      (comment) => comment.id === commentId && comment.positionId === positionId,
    );

    if (index === -1) {
      throw new Error("Comment not found");
    }

    const existing = this.comments[index].toPrimitives();
    const updated = Comment.fromPrimitives({
      ...existing,
      body: input.body,
      updatedAt: new Date().toISOString(),
    });

    this.comments[index] = updated;
    return updated;
  }

  async deleteComment(positionId: string, commentId: string): Promise<void> {
    this.comments = this.comments.filter(
      (comment) => !(comment.id === commentId && comment.positionId === positionId),
    );
  }
}
