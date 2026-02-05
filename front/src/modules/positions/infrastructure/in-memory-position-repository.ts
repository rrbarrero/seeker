import type { Position } from "../domain/position";
import type { PositionRepository } from "../domain/position-repository";

export class InMemoryPositionRepository implements PositionRepository {
    async getPositions(): Promise<Position[]> {
        // Simulate network delay
        await new Promise((resolve) => setTimeout(resolve, 500));

        return [
            {
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
            },
            {
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
            },
        ];
    }
}
