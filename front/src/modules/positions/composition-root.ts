import { PositionService } from "./application/position-service";
import { ApiPositionRepository } from "./infrastructure/api-position-repository";
import { InMemoryPositionRepository } from "./infrastructure/in-memory-position-repository";
import { tokenRepository } from "@/modules/auth/composition-root";

const useInMemory = process.env.NEXT_PUBLIC_USE_IN_MEMORY_REPO === "true";

const positionRepository = useInMemory
  ? new InMemoryPositionRepository()
  : new ApiPositionRepository(tokenRepository);

export const positionService = new PositionService(positionRepository);
