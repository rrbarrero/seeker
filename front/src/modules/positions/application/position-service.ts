import type { CreatePositionInput, Position } from "../domain/position";
import type { PositionRepository } from "../domain/position-repository";

export class PositionService {
  constructor(private readonly repository: PositionRepository) {}

  async getPositions(token?: string): Promise<Position[]> {
    return this.repository.getPositions(token);
  }

  async createPosition(position: CreatePositionInput, token?: string): Promise<Position> {
    return this.repository.createPosition(position, token);
  }

  async getPosition(id: string, token?: string): Promise<Position> {
    return this.repository.getPositionById(id, token);
  }
}
