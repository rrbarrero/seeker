import type { CreatePositionInput, Position } from "../domain/position";
import type { PositionRepository } from "../domain/position-repository";

export class PositionService {
  constructor(private readonly repository: PositionRepository) {}

  async getPositions(): Promise<Position[]> {
    return this.repository.getPositions();
  }

  async createPosition(position: CreatePositionInput): Promise<Position> {
    return this.repository.createPosition(position);
  }
}
