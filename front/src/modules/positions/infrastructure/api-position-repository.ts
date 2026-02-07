import { getBaseUrl } from "@/shared/api-config";
import type { CreatePositionInput, Position } from "../domain/position";
import type { PositionRepository } from "../domain/position-repository";

export class ApiPositionRepository implements PositionRepository {
  async getPositions(providedToken?: string): Promise<Position[]> {
    const token =
      providedToken || (typeof window !== "undefined" ? localStorage.getItem("token") : null);

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

    return response.json();
  }

  async createPosition(position: CreatePositionInput, providedToken?: string): Promise<Position> {
    const token =
      providedToken || (typeof window !== "undefined" ? localStorage.getItem("token") : null);

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

    return response.json();
  }

  async getPositionById(id: string, providedToken?: string): Promise<Position> {
    const token =
      providedToken || (typeof window !== "undefined" ? localStorage.getItem("token") : null);

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

    return response.json();
  }
}
