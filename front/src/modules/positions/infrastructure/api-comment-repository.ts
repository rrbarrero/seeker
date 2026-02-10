import {
  InfrastructureError,
  NotFoundError,
  UnauthorizedError,
  ForbiddenError,
} from "@/shared/domain/errors";
import { requestEmpty, requestJson } from "@/shared/http-client";
import type { TokenRepository } from "@/modules/auth/domain/token-repository";
import {
  Comment,
  type CommentProps,
  type CreateCommentInput,
  type UpdateCommentInput,
} from "../domain/comment";
import type { CommentRepository } from "../domain/comment-repository";

type CommentDto = {
  id: string;
  position_id: string;
  user_id: string;
  body: string;
  created_at: string;
  updated_at: string;
};

type CreateCommentDto = {
  body: string;
};

const toDomainProps = (dto: CommentDto): CommentProps => ({
  id: dto.id,
  positionId: dto.position_id,
  userId: dto.user_id,
  body: dto.body,
  createdAt: dto.created_at,
  updatedAt: dto.updated_at,
});

const toCreateDto = (input: CreateCommentInput | UpdateCommentInput): CreateCommentDto => ({
  body: input.body,
});

export class ApiCommentRepository implements CommentRepository {
  constructor(private readonly tokenRepository: TokenRepository) {}

  async getComments(positionId: string, providedToken?: string): Promise<Comment[]> {
    const token = providedToken || this.tokenRepository.get();

    if (!token) {
      throw new UnauthorizedError("No authentication token found");
    }

    const { response, data } = await requestJson<CommentDto[]>(
      `/positions/${positionId}/comments`,
      {
        method: "GET",
        token,
      },
    );

    if (!response.ok) {
      if (response.status === 401) {
        throw new UnauthorizedError();
      }
      if (response.status === 403) {
        throw new ForbiddenError();
      }
      throw new InfrastructureError(
        `Error fetching comments: ${response.statusText}`,
        "FETCH_ERROR",
        response.status,
      );
    }

    if (!data) {
      throw new InfrastructureError("Error fetching comments", "FETCH_ERROR", response.status);
    }

    return data.map((dto) => Comment.fromPrimitives(toDomainProps(dto)));
  }

  async createComment(
    positionId: string,
    input: CreateCommentInput,
    providedToken?: string,
  ): Promise<Comment> {
    const token = providedToken || this.tokenRepository.get();

    if (!token) {
      throw new UnauthorizedError("No authentication token found");
    }

    const { response, data } = await requestJson<CommentDto>(`/positions/${positionId}/comments`, {
      method: "POST",
      token,
      body: toCreateDto(input),
    });

    if (!response.ok) {
      if (response.status === 401) {
        throw new UnauthorizedError();
      }
      if (response.status === 403) {
        throw new ForbiddenError();
      }
      if (response.status === 404) {
        throw new NotFoundError("Position not found");
      }
      throw new InfrastructureError(
        `Error creating comment: ${response.statusText}`,
        "CREATE_ERROR",
        response.status,
      );
    }

    if (!data) {
      throw new InfrastructureError("Error creating comment", "CREATE_ERROR", response.status);
    }

    return Comment.fromPrimitives(toDomainProps(data));
  }

  async updateComment(
    positionId: string,
    commentId: string,
    input: UpdateCommentInput,
    providedToken?: string,
  ): Promise<Comment> {
    const token = providedToken || this.tokenRepository.get();

    if (!token) {
      throw new UnauthorizedError("No authentication token found");
    }

    const { response, data } = await requestJson<CommentDto>(
      `/positions/${positionId}/comments/${commentId}`,
      {
        method: "PUT",
        token,
        body: toCreateDto(input),
      },
    );

    if (!response.ok) {
      if (response.status === 401) {
        throw new UnauthorizedError();
      }
      if (response.status === 403) {
        throw new ForbiddenError();
      }
      if (response.status === 404) {
        throw new NotFoundError("Comment not found");
      }
      throw new InfrastructureError(
        `Error updating comment: ${response.statusText}`,
        "UPDATE_ERROR",
        response.status,
      );
    }

    if (!data) {
      throw new InfrastructureError("Error updating comment", "UPDATE_ERROR", response.status);
    }

    return Comment.fromPrimitives(toDomainProps(data));
  }

  async deleteComment(
    positionId: string,
    commentId: string,
    providedToken?: string,
  ): Promise<void> {
    const token = providedToken || this.tokenRepository.get();

    if (!token) {
      throw new UnauthorizedError("No authentication token found");
    }

    const { response } = await requestEmpty(`/positions/${positionId}/comments/${commentId}`, {
      method: "DELETE",
      token,
    });

    if (!response.ok) {
      if (response.status === 401) {
        throw new UnauthorizedError();
      }
      if (response.status === 403) {
        throw new ForbiddenError();
      }
      if (response.status === 404) {
        throw new NotFoundError("Comment not found");
      }
      throw new InfrastructureError(
        `Error deleting comment: ${response.statusText}`,
        "DELETE_ERROR",
        response.status,
      );
    }
  }
}
