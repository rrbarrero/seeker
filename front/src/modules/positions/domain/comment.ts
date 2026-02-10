import { DomainError } from "@/shared/domain/errors";

export interface CommentProps {
  id: string;
  positionId: string;
  userId: string;
  body: string;
  createdAt: string;
  updatedAt: string;
}

export class Comment {
  constructor(private readonly props: CommentProps) {
    this.validate();
  }

  private validate(): void {
    if (!this.props.body || this.props.body.trim() === "") {
      throw new DomainError("Comment body is required", "MISSING_COMMENT_BODY");
    }
  }

  get id(): string {
    return this.props.id;
  }
  get positionId(): string {
    return this.props.positionId;
  }
  get userId(): string {
    return this.props.userId;
  }
  get body(): string {
    return this.props.body;
  }
  get createdAt(): string {
    return this.props.createdAt;
  }
  get updatedAt(): string {
    return this.props.updatedAt;
  }

  public static fromPrimitives(props: CommentProps): Comment {
    return new Comment(props);
  }

  public toPrimitives(): CommentProps {
    return { ...this.props };
  }
}

export type CreateCommentInput = {
  body: string;
};

export type UpdateCommentInput = CreateCommentInput;
