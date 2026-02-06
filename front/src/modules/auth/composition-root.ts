import { AuthService } from "./application/auth-service";
import { ApiAuthRepository } from "./infrastructure/api-auth-repository";
import { InMemoryAuthRepository } from "./infrastructure/in-memory-auth-repository";
import { CookieTokenRepository } from "./infrastructure/cookie-token-repository";

const useInMemory = process.env.NEXT_PUBLIC_USE_IN_MEMORY_REPO === "true";

const authRepository = useInMemory ? new InMemoryAuthRepository() : new ApiAuthRepository();
export const tokenRepository = new CookieTokenRepository();

export const authService = new AuthService(authRepository, tokenRepository);
