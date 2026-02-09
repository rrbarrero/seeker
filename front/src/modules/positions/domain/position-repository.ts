import type { CreatePositionInput, Position, UpdatePositionInput } from "./position";

export interface PositionRepository {
  getPositions(token?: string): Promise<Position[]>;
  createPosition(position: CreatePositionInput, token?: string): Promise<Position>;
  getPositionById(id: string, token?: string): Promise<Position>;
  updatePosition(id: string, input: UpdatePositionInput, token?: string): Promise<void>;
  delete(id: string, token?: string): Promise<void>;
}
