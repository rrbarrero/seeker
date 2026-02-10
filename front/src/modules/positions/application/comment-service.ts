import type { CommentRepository } from "../domain/comment-repository";
import type { Comment, CreateCommentInput, UpdateCommentInput } from "../domain/comment";

export class CommentService {
  constructor(private readonly repository: CommentRepository) {}

  async getComments(positionId: string, token?: string): Promise<Comment[]> {
    return this.repository.getComments(positionId, token);
  }

  async createComment(
    positionId: string,
    input: CreateCommentInput,
    token?: string,
  ): Promise<Comment> {
    return this.repository.createComment(positionId, input, token);
  }

  async updateComment(
    positionId: string,
    commentId: string,
    input: UpdateCommentInput,
    token?: string,
  ): Promise<Comment> {
    return this.repository.updateComment(positionId, commentId, input, token);
  }

  async deleteComment(positionId: string, commentId: string, token?: string): Promise<void> {
    await this.repository.deleteComment(positionId, commentId, token);
  }
}
