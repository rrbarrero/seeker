import type { Position } from "./position";

export interface PositionRepository {
    getPositions(): Promise<Position[]>;
}
