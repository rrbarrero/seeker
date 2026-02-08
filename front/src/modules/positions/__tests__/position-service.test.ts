import { describe, it, expect, vi, beforeEach } from "vitest";
import { PositionService } from "../application/position-service";
import type { PositionRepository } from "../domain/position-repository";
import { Position, type CreatePositionInput } from "../domain/position";

describe("PositionService", () => {
  let positionService: PositionService;
  let mockRepository: PositionRepository;
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

    mockRepository = {
      getPositions: vi.fn(),
      createPosition: vi.fn(),
      getPositionById: vi.fn(),
    };

    positionService = new PositionService(mockRepository);
  });

  describe("getPositions", () => {
    it("should return positions from repository", async () => {
      vi.mocked(mockRepository.getPositions).mockResolvedValue(mockPositions);

      const result = await positionService.getPositions("test-token");

      expect(mockRepository.getPositions).toHaveBeenCalledWith("test-token");
      expect(result).toEqual(mockPositions);
    });

    it("should call repository without token when not provided", async () => {
      vi.mocked(mockRepository.getPositions).mockResolvedValue(mockPositions);

      await positionService.getPositions();

      expect(mockRepository.getPositions).toHaveBeenCalledWith(undefined);
    });

    it("should propagate repository errors", async () => {
      const error = new Error("Repository error");
      vi.mocked(mockRepository.getPositions).mockRejectedValue(error);

      await expect(positionService.getPositions()).rejects.toThrow("Repository error");
    });

    it("should return empty array when repository has no positions", async () => {
      vi.mocked(mockRepository.getPositions).mockResolvedValue([]);

      const result = await positionService.getPositions();

      expect(result).toEqual([]);
    });
  });

  describe("createPosition", () => {
    it("should create position through repository", async () => {
      const expectedPosition = Position.fromPrimitives({
        ...mockCreatePositionInput,
        id: "3",
        user_id: "user1",
        created_at: "2024-02-01T10:00:00Z",
        updated_at: "2024-02-01T10:00:00Z",
        deleted_at: null,
        deleted: false,
      });

      vi.mocked(mockRepository.createPosition).mockResolvedValue(expectedPosition);

      const result = await positionService.createPosition(mockCreatePositionInput, "test-token");

      expect(mockRepository.createPosition).toHaveBeenCalledWith(
        mockCreatePositionInput,
        "test-token",
      );
      expect(result).toEqual(expectedPosition);
    });

    it("should call repository without token when not provided", async () => {
      const expectedPosition = Position.fromPrimitives({
        ...mockCreatePositionInput,
        id: "3",
        user_id: "user1",
        created_at: "2024-02-01T10:00:00Z",
        updated_at: "2024-02-01T10:00:00Z",
        deleted_at: null,
        deleted: false,
      });

      vi.mocked(mockRepository.createPosition).mockResolvedValue(expectedPosition);

      await positionService.createPosition(mockCreatePositionInput);

      expect(mockRepository.createPosition).toHaveBeenCalledWith(
        mockCreatePositionInput,
        undefined,
      );
    });

    it("should propagate repository errors", async () => {
      const error = new Error("Creation failed");
      vi.mocked(mockRepository.createPosition).mockRejectedValue(error);

      await expect(positionService.createPosition(mockCreatePositionInput)).rejects.toThrow(
        "Creation failed",
      );
    });
  });

  describe("getPosition", () => {
    it("should get position by id from repository", async () => {
      const positionId = "1";
      vi.mocked(mockRepository.getPositionById).mockResolvedValue(mockPositions[0]);

      const result = await positionService.getPosition(positionId, "test-token");

      expect(mockRepository.getPositionById).toHaveBeenCalledWith(positionId, "test-token");
      expect(result).toEqual(mockPositions[0]);
    });

    it("should call repository without token when not provided", async () => {
      const positionId = "1";
      vi.mocked(mockRepository.getPositionById).mockResolvedValue(mockPositions[0]);

      await positionService.getPosition(positionId);

      expect(mockRepository.getPositionById).toHaveBeenCalledWith(positionId, undefined);
    });

    it("should propagate repository errors", async () => {
      const positionId = "999";
      const error = new Error("Position not found");
      vi.mocked(mockRepository.getPositionById).mockRejectedValue(error);

      await expect(positionService.getPosition(positionId)).rejects.toThrow("Position not found");
    });

    it("should handle different position statuses", async () => {
      const props = mockPositions[0].toPrimitives();
      const positionWithDifferentStatus = Position.fromPrimitives({
        ...props,
        status: "OfferReceived",
      });

      vi.mocked(mockRepository.getPositionById).mockResolvedValue(positionWithDifferentStatus);

      const result = await positionService.getPosition("1");

      expect(result.status).toBe("OfferReceived");
    });
  });

  describe("integration behavior", () => {
    it("should maintain consistency across operations", async () => {
      vi.mocked(mockRepository.getPositions).mockResolvedValue([mockPositions[0]]);
      vi.mocked(mockRepository.createPosition).mockResolvedValue(
        Position.fromPrimitives({
          ...mockCreatePositionInput,
          id: "3",
          user_id: "user1",
          created_at: "2024-02-01T10:00:00Z",
          updated_at: "2024-02-01T10:00:00Z",
          deleted_at: null,
          deleted: false,
        }),
      );

      const initialPositions = await positionService.getPositions();
      const newPosition = await positionService.createPosition(mockCreatePositionInput);

      expect(initialPositions).toHaveLength(1);
      expect(newPosition.id).toBe("3");
      expect(newPosition.company).toBe(mockCreatePositionInput.company);
    });
  });
});
