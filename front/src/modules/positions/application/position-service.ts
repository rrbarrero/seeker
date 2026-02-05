import type { Position } from "../domain/position";
import type { PositionRepository } from "../domain/position-repository";

export class PositionService {
    constructor(private readonly repository: PositionRepository) { }

    async getPositions(): Promise<Position[]> {
        return this.repository.getPositions();
    }
}
