import { getBaseUrl } from "@/shared/api-config";
import { InfrastructureError, NotFoundError, UnauthorizedError } from "@/shared/domain/errors";
import type { TokenRepository } from "@/modules/auth/domain/token-repository";
import { Position, type CreatePositionInput, type PositionProps } from "../domain/position";
import type { PositionRepository } from "../domain/position-repository";

export class ApiPositionRepository implements PositionRepository {
  constructor(private readonly tokenRepository: TokenRepository) {}

  async getPositions(providedToken?: string): Promise<Position[]> {
    const token = providedToken || this.tokenRepository.get();

    if (!token) {
      throw new UnauthorizedError("No authentication token found");
    }

    const response = await fetch(`${getBaseUrl()}/positions`, {
      method: "GET",
      headers: {
        "Content-Type": "application/json",
        Authorization: `Bearer ${token}`,
      },
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

    const data: PositionProps[] = await response.json();
    return data.map((props) => Position.fromPrimitives(props));
  }

  async createPosition(position: CreatePositionInput, providedToken?: string): Promise<Position> {
    const token = providedToken || this.tokenRepository.get();

    if (!token) {
      throw new UnauthorizedError("No authentication token found");
    }

    const response = await fetch(`${getBaseUrl()}/positions`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        Authorization: `Bearer ${token}`,
      },
      body: JSON.stringify(position),
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

    const props: PositionProps = await response.json();
    return Position.fromPrimitives(props);
  }

  async getPositionById(id: string, providedToken?: string): Promise<Position> {
    const token = providedToken || this.tokenRepository.get();

    if (!token) {
      throw new UnauthorizedError("No authentication token found");
    }

    const response = await fetch(`${getBaseUrl()}/positions/${id}`, {
      method: "GET",
      headers: {
        "Content-Type": "application/json",
        Authorization: `Bearer ${token}`,
      },
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

    const props: PositionProps = await response.json();
    return Position.fromPrimitives(props);
  }
}
