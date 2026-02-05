import type { Position } from "../domain/position";
import type { PositionRepository } from "../domain/position-repository";

export class ApiPositionRepository implements PositionRepository {
    async getPositions(): Promise<Position[]> {
        const token = typeof window !== "undefined" ? localStorage.getItem("token") : null;

        if (!token) {
            throw new Error("No authentication token found");
        }

        const response = await fetch(`${process.env.NEXT_PUBLIC_API_URL}/positions`, {
            method: "GET",
            headers: {
                "Content-Type": "application/json",
                Authorization: `Bearer ${token}`,
            },
        });

        if (!response.ok) {
            if (response.status === 401) {
                throw new Error("Unauthorized");
            }
            throw new Error(`Error fetching positions: ${response.statusText}`);
        }

        return response.json();
    }
}
