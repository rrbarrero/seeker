import {
  Position,
  type CreatePositionInput,
  type PositionProps,
  type UpdatePositionInput,
} from "../domain/position";
import type { PositionRepository } from "../domain/position-repository";

export class InMemoryPositionRepository implements PositionRepository {
  private positions: Position[] = [];

  constructor(initialPositions?: Position[]) {
    if (initialPositions) {
      this.positions = [...initialPositions];
    } else {
      this.positions = [
        Position.fromPrimitives({
          id: "1",
          userId: "user-1",
          company: "Rust Corp",
          roleTitle: "Senior Rust Developer",
          description: "Writing safe code.",
          appliedOn: "2023-10-27",
          url: "https://rust-corp.com/jobs/1",
          initialComment: "Looks promising",
          status: "CvSent",
          createdAt: new Date().toISOString(),
          updatedAt: new Date().toISOString(),
          deletedAt: null,
          deleted: false,
        }),
        Position.fromPrimitives({
          id: "2",
          userId: "user-1",
          company: "Next.js Inc",
          roleTitle: "Frontend Engineer",
          description: "Building the web.",
          appliedOn: "2023-11-01",
          url: "https://nextjs.org/jobs/2",
          initialComment: "Referral from a friend",
          status: "TechnicalInterview",
          createdAt: new Date().toISOString(),
          updatedAt: new Date().toISOString(),
          deletedAt: null,
          deleted: false,
        }),
      ];
    }
  }

  async getPositions(_token?: string): Promise<Position[]> {
    // Simulate network delay
    await new Promise((resolve) => setTimeout(resolve, 500));
    return [...this.positions];
  }

  async createPosition(input: CreatePositionInput, _token?: string): Promise<Position> {
    await new Promise((resolve) => setTimeout(resolve, 500));

    const props: PositionProps = {
      ...input,
      id: Math.random().toString(36).substring(7),
      userId: "user-1",
      createdAt: new Date().toISOString(),
      updatedAt: new Date().toISOString(),
      deletedAt: null,
      deleted: false,
    };

    const newPosition = Position.fromPrimitives(props);
    this.positions.push(newPosition);
    return newPosition;
  }

  async getPositionById(id: string, _token?: string): Promise<Position> {
    await new Promise((resolve) => setTimeout(resolve, 500));
    const position = this.positions.find((p) => p.id === id);
    if (!position) {
      throw new Error("Position not found");
    }
    return position;
  }

  async updatePosition(id: string, input: UpdatePositionInput, _token?: string): Promise<void> {
    await new Promise((resolve) => setTimeout(resolve, 500));
    const index = this.positions.findIndex((p) => p.id === id);
    const existing = this.positions[index];
    if (!existing) {
      throw new Error("Position not found");
    }

    const props = existing.toPrimitives();
    const updated = Position.fromPrimitives({
      ...props,
      company: input.company,
      roleTitle: input.roleTitle,
      description: input.description,
      appliedOn: input.appliedOn,
      url: input.url,
      initialComment: input.initialComment,
      status: input.status,
      updatedAt: new Date().toISOString(),
    });

    this.positions[index] = updated;
  }

  async delete(id: string, _token?: string): Promise<void> {
    await new Promise((resolve) => setTimeout(resolve, 500));
    this.positions = this.positions.filter((p) => p.id !== id);
  }
}
