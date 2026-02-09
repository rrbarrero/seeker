import type {
  CreatePositionInput,
  Position,
  PositionProps,
  PositionStatus,
} from "../domain/position";
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

  async updatePosition(
    id: string,
    changes: Partial<
      Omit<PositionProps, "id" | "user_id" | "created_at" | "updated_at" | "deleted" | "deleted_at">
    >,
    token?: string,
  ): Promise<void> {
    const position = await this.repository.getPositionById(id, token);
    position.update(changes);
    await this.repository.save(position, token);
  }

  async changeStatus(id: string, newStatus: PositionStatus, token?: string): Promise<void> {
    const position = await this.repository.getPositionById(id, token);
    position.advanceStatus(newStatus);
    await this.repository.save(position, token);
  }

  async deletePosition(id: string, token?: string): Promise<void> {
    await this.repository.delete(id, token);
  }
}
