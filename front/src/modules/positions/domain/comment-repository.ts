import type { Comment, CreateCommentInput, UpdateCommentInput } from "./comment";

export interface CommentRepository {
  getComments(positionId: string, token?: string): Promise<Comment[]>;
  createComment(positionId: string, input: CreateCommentInput, token?: string): Promise<Comment>;
  updateComment(
    positionId: string,
    commentId: string,
    input: UpdateCommentInput,
    token?: string,
  ): Promise<Comment>;
  deleteComment(positionId: string, commentId: string, token?: string): Promise<void>;
}
