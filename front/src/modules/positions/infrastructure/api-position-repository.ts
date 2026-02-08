import { getBaseUrl } from "@/shared/api-config";
import type { TokenRepository } from "@/modules/auth/domain/token-repository";
import { Position, type CreatePositionInput, type PositionProps } from "../domain/position";
import type { PositionRepository } from "../domain/position-repository";

export class ApiPositionRepository implements PositionRepository {
  constructor(private readonly tokenRepository: TokenRepository) {}

  async getPositions(providedToken?: string): Promise<Position[]> {
    const token = providedToken || this.tokenRepository.get();

    if (!token) {
      throw new Error("No authentication token found");
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
        throw new Error("Unauthorized");
      }
      throw new Error(`Error fetching positions: ${response.statusText}`);
    }

    const data: PositionProps[] = await response.json();
    return data.map((props) => Position.fromPrimitives(props));
  }

  async createPosition(position: CreatePositionInput, providedToken?: string): Promise<Position> {
    const token = providedToken || this.tokenRepository.get();

    if (!token) {
      throw new Error("No authentication token found");
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
        throw new Error("Unauthorized");
      }
      throw new Error(`Error creating position: ${response.statusText}`);
    }

    const props: PositionProps = await response.json();
    return Position.fromPrimitives(props);
  }

  async getPositionById(id: string, providedToken?: string): Promise<Position> {
    const token = providedToken || this.tokenRepository.get();

    if (!token) {
      throw new Error("No authentication token found");
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
        throw new Error("Unauthorized");
      }
      if (response.status === 404) {
        throw new Error("Position not found");
      }
      throw new Error(`Error fetching position: ${response.statusText}`);
    }

    const props: PositionProps = await response.json();
    return Position.fromPrimitives(props);
  }
}
