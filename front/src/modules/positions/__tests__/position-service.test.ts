import { describe, it, expect, beforeEach } from "vitest";
import { PositionService } from "../application/position-service";
import { Position, type CreatePositionInput } from "../domain/position";
import { InMemoryPositionRepository } from "../infrastructure/in-memory-position-repository";

describe("PositionService", () => {
  let positionService: PositionService;
  let repository: InMemoryPositionRepository;
  let mockPositions: Position[];
  let mockCreatePositionInput: CreatePositionInput;

  beforeEach(() => {
    mockPositions = [
      Position.fromPrimitives({
        id: "1",
        user_id: "user1",
        company: "Acme Corp",
        role_title: "Senior Developer",
        description: "A great opportunity",
        applied_on: "2024-01-15T00:00:00Z",
        url: "https://acme.com/jobs/1",
        initial_comment: "Excited about this role",
        status: "CvSent",
        created_at: "2024-01-15T10:00:00Z",
        updated_at: "2024-01-15T10:00:00Z",
        deleted_at: null,
        deleted: false,
      }),
      Position.fromPrimitives({
        id: "2",
        user_id: "user1",
        company: "Tech Inc",
        role_title: "Frontend Engineer",
        description: "React position",
        applied_on: "2024-01-20T00:00:00Z",
        url: "https://tech.com/jobs/2",
        initial_comment: "Good culture fit",
        status: "TechnicalInterview",
        created_at: "2024-01-20T10:00:00Z",
        updated_at: "2024-01-20T10:00:00Z",
        deleted_at: null,
        deleted: false,
      }),
    ];

    mockCreatePositionInput = {
      company: "New Company",
      role_title: "Junior Developer",
      description: "Entry level position",
      applied_on: "2024-02-01T00:00:00Z",
      url: "https://newcompany.com/jobs/1",
      initial_comment: "First job application",
      status: "CvSent",
    };

    // Initialize repository with mock positions
    repository = new InMemoryPositionRepository(mockPositions);
    positionService = new PositionService(repository);
  });

  describe("getPositions", () => {
    it("should return positions from repository", async () => {
      const result = await positionService.getPositions("test-token");
      expect(result).toHaveLength(2);
      expect(result[0].id).toBe("1");
      expect(result[1].id).toBe("2");
    });

    it("should return empty array when repository has no positions", async () => {
      const emptyRepo = new InMemoryPositionRepository([]);
      const service = new PositionService(emptyRepo);

      const result = await service.getPositions();
      expect(result).toEqual([]);
    });
  });

  describe("createPosition", () => {
    it("should create position through repository", async () => {
      const result = await positionService.createPosition(mockCreatePositionInput, "test-token");

      expect(result.company).toBe(mockCreatePositionInput.company);
      expect(result.role_title).toBe(mockCreatePositionInput.role_title);

      // Verify it was actually saved in the repo
      const allPositions = await repository.getPositions();
      expect(allPositions).toHaveLength(3);
    });
  });

  describe("getPosition", () => {
    it("should get position by id from repository", async () => {
      const result = await positionService.getPosition("1", "test-token");
      expect(result.id).toBe("1");
      expect(result.company).toBe("Acme Corp");
    });

    it("should throw error when position not found", async () => {
      await expect(positionService.getPosition("999")).rejects.toThrow("Position not found");
    });

    it("should handle different position statuses", async () => {
      const props = mockPositions[0].toPrimitives();
      const updatedRepo = new InMemoryPositionRepository([
        Position.fromPrimitives({ ...props, status: "OfferReceived" }),
      ]);
      const service = new PositionService(updatedRepo);

      const result = await service.getPosition("1");
      expect(result.status).toBe("OfferReceived");
    });
  });

  describe("integration behavior", () => {
    it("should maintain consistency across operations", async () => {
      const initialPositions = await positionService.getPositions();
      expect(initialPositions).toHaveLength(2);

      const newPosition = await positionService.createPosition(mockCreatePositionInput);
      expect(newPosition.company).toBe(mockCreatePositionInput.company);

      const updatedPositions = await positionService.getPositions();
      expect(updatedPositions).toHaveLength(3);

      const fetchedPosition = await positionService.getPosition(newPosition.id);
      expect(fetchedPosition.id).toBe(newPosition.id);
    });
  });
});
