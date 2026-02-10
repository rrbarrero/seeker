import { InfrastructureError, NotFoundError, UnauthorizedError } from "@/shared/domain/errors";
import { requestEmpty, requestJson } from "@/shared/http-client";
import type { TokenRepository } from "@/modules/auth/domain/token-repository";
import {
  Position,
  type CreatePositionInput,
  type PositionProps,
  type UpdatePositionInput,
} from "../domain/position";
import type { PositionRepository } from "../domain/position-repository";

type PositionDto = {
  id: string;
  user_id: string;
  company: string;
  role_title: string;
  description: string;
  applied_on: string;
  url: string;
  status: string;
  created_at: string;
  updated_at: string;
  deleted_at: string | null;
  deleted: boolean;
};

type CreatePositionDto = {
  company: string;
  role_title: string;
  description: string;
  applied_on: string;
  url: string;
  status: string;
};

const toDomainProps = (dto: PositionDto): PositionProps => ({
  id: dto.id,
  userId: dto.user_id,
  company: dto.company,
  roleTitle: dto.role_title,
  description: dto.description,
  appliedOn: dto.applied_on,
  url: dto.url,
  status: dto.status as PositionProps["status"],
  createdAt: dto.created_at,
  updatedAt: dto.updated_at,
  deletedAt: dto.deleted_at,
  deleted: dto.deleted,
});

const toCreateDto = (input: CreatePositionInput): CreatePositionDto => ({
  company: input.company,
  role_title: input.roleTitle,
  description: input.description,
  applied_on: input.appliedOn,
  url: input.url,
  status: input.status,
});

const toUpdateDto = (input: UpdatePositionInput): CreatePositionDto => ({
  company: input.company,
  role_title: input.roleTitle,
  description: input.description,
  applied_on: input.appliedOn,
  url: input.url,
  status: input.status,
});

export class ApiPositionRepository implements PositionRepository {
  constructor(private readonly tokenRepository: TokenRepository) {}

  async getPositions(providedToken?: string): Promise<Position[]> {
    const token = providedToken || this.tokenRepository.get();

    if (!token) {
      throw new UnauthorizedError("No authentication token found");
    }

    const { response, data } = await requestJson<PositionDto[]>("/positions", {
      method: "GET",
      token,
    });

    if (!response.ok) {
      if (response.status === 401) {
        throw new UnauthorizedError();
      }
      throw new InfrastructureError(
        `Error fetching positions: ${response.statusText}`,
        "FETCH_ERROR",
        response.status,
      );
    }

    if (!data) {
      throw new InfrastructureError("Error fetching positions", "FETCH_ERROR", response.status);
    }

    return data.map((dto) => Position.fromPrimitives(toDomainProps(dto)));
  }

  async createPosition(position: CreatePositionInput, providedToken?: string): Promise<Position> {
    const token = providedToken || this.tokenRepository.get();

    if (!token) {
      throw new UnauthorizedError("No authentication token found");
    }

    const { response, data: dto } = await requestJson<PositionDto>("/positions", {
      method: "POST",
      token,
      body: toCreateDto(position),
    });

    if (!response.ok) {
      if (response.status === 401) {
        throw new UnauthorizedError();
      }
      throw new InfrastructureError(
        `Error creating position: ${response.statusText}`,
        "CREATE_ERROR",
        response.status,
      );
    }

    if (!dto) {
      throw new InfrastructureError("Error creating position", "CREATE_ERROR", response.status);
    }

    return Position.fromPrimitives(toDomainProps(dto));
  }

  async getPositionById(id: string, providedToken?: string): Promise<Position> {
    const token = providedToken || this.tokenRepository.get();

    if (!token) {
      throw new UnauthorizedError("No authentication token found");
    }

    const { response, data: dto } = await requestJson<PositionDto>(`/positions/${id}`, {
      method: "GET",
      token,
    });

    if (!response.ok) {
      if (response.status === 401) {
        throw new UnauthorizedError();
      }
      if (response.status === 404) {
        throw new NotFoundError("Position not found");
      }
      throw new InfrastructureError(
        `Error fetching position: ${response.statusText}`,
        "FETCH_ONE_ERROR",
        response.status,
      );
    }

    if (!dto) {
      throw new InfrastructureError("Error fetching position", "FETCH_ONE_ERROR", response.status);
    }

    return Position.fromPrimitives(toDomainProps(dto));
  }

  async updatePosition(
    id: string,
    input: UpdatePositionInput,
    providedToken?: string,
  ): Promise<void> {
    const token = providedToken || this.tokenRepository.get();

    if (!token) {
      throw new UnauthorizedError("No authentication token found");
    }

    const { response } = await requestEmpty(`/positions/${id}`, {
      method: "PUT",
      token,
      body: toUpdateDto(input),
    });

    if (!response.ok) {
      if (response.status === 401) {
        throw new UnauthorizedError();
      }
      if (response.status === 404) {
        throw new NotFoundError("Position not found");
      }
      throw new InfrastructureError(
        `Error saving position: ${response.statusText}`,
        "SAVE_ERROR",
        response.status,
      );
    }
  }

  async delete(id: string, providedToken?: string): Promise<void> {
    const token = providedToken || this.tokenRepository.get();

    if (!token) {
      throw new UnauthorizedError("No authentication token found");
    }

    const { response } = await requestEmpty(`/positions/${id}`, {
      method: "DELETE",
      token,
    });

    if (!response.ok) {
      if (response.status === 401) {
        throw new UnauthorizedError();
      }
      if (response.status === 404) {
        throw new NotFoundError("Position not found");
      }
      throw new InfrastructureError(
        `Error deleting position: ${response.statusText}`,
        "DELETE_ERROR",
        response.status,
      );
    }
  }
}
