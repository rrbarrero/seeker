import { Position, type CreatePositionInput, type PositionProps } from "../domain/position";
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
          user_id: "user-1",
          company: "Rust Corp",
          role_title: "Senior Rust Developer",
          description: "Writing safe code.",
          applied_on: "2023-10-27",
          url: "https://rust-corp.com/jobs/1",
          initial_comment: "Looks promising",
          status: "CvSent",
          created_at: new Date().toISOString(),
          updated_at: new Date().toISOString(),
          deleted_at: null,
          deleted: false,
        }),
        Position.fromPrimitives({
          id: "2",
          user_id: "user-1",
          company: "Next.js Inc",
          role_title: "Frontend Engineer",
          description: "Building the web.",
          applied_on: "2023-11-01",
          url: "https://nextjs.org/jobs/2",
          initial_comment: "Referral from a friend",
          status: "TechnicalInterview",
          created_at: new Date().toISOString(),
          updated_at: new Date().toISOString(),
          deleted_at: null,
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
      user_id: "user-1",
      created_at: new Date().toISOString(),
      updated_at: new Date().toISOString(),
      deleted_at: null,
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
}
