import type { CreatePositionInput, Position } from "./position";

export interface PositionRepository {
  getPositions(): Promise<Position[]>;
  createPosition(position: CreatePositionInput): Promise<Position>;
}
