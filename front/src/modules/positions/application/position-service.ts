import type {
  CreatePositionInput,
  Position,
  PositionProps,
  PositionStatus,
  UpdatePositionInput,
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
      Omit<PositionProps, "id" | "userId" | "createdAt" | "updatedAt" | "deleted" | "deletedAt">
    >,
    token?: string,
  ): Promise<void> {
    const position = await this.repository.getPositionById(id, token);
    position.update(changes);

    if (changes.status && changes.status !== position.status) {
      position.advanceStatus(changes.status);
    }

    await this.repository.updatePosition(id, this.toUpdateInput(position), token);
  }

  async changeStatus(id: string, newStatus: PositionStatus, token?: string): Promise<void> {
    const position = await this.repository.getPositionById(id, token);
    position.advanceStatus(newStatus);
    await this.repository.updatePosition(id, this.toUpdateInput(position), token);
  }

  async deletePosition(id: string, token?: string): Promise<void> {
    await this.repository.delete(id, token);
  }

  private toUpdateInput(position: Position): UpdatePositionInput {
    const props = position.toPrimitives();
    return {
      company: props.company,
      roleTitle: props.roleTitle,
      description: props.description,
      appliedOn: props.appliedOn,
      url: props.url,
      initialComment: props.initialComment,
      status: props.status,
    };
  }
}
