import type { CreatePositionInput, Position } from "./position";

export interface PositionRepository {
  getPositions(token?: string): Promise<Position[]>;
  createPosition(position: CreatePositionInput, token?: string): Promise<Position>;
  getPositionById(id: string, token?: string): Promise<Position>;
  save(position: Position, token?: string): Promise<void>;
  delete(id: string, token?: string): Promise<void>;
}
