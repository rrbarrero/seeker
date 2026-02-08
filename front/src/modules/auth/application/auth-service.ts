import type { AuthRepository } from "../domain/auth-repository";
import type { TokenRepository } from "../domain/token-repository";
import type { LoginFormValues, RegisterFormValues } from "../domain/schema";

export class AuthService {
  constructor(
    private readonly authRepository: AuthRepository,
    private readonly tokenRepository: TokenRepository,
  ) {}

  async register(data: RegisterFormValues): Promise<void> {
    return this.authRepository.register(data);
  }

  async login(data: LoginFormValues): Promise<void> {
    const response = await this.authRepository.login(data);
    this.tokenRepository.save(response.access_token);
  }

  getToken(): string | null {
    return this.tokenRepository.get();
  }

  logout(): void {
    this.tokenRepository.remove();
  }
}
