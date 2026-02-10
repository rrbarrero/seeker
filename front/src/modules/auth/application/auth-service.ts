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
    if (typeof window !== "undefined") {
      localStorage.setItem("email_validated", String(response.email_validated));
    }
  }

  async verifyEmail(token: string): Promise<void> {
    return this.authRepository.verifyEmail(token);
  }

  getToken(): string | null {
    return this.tokenRepository.get();
  }

  isEmailValidated(): boolean {
    if (typeof window !== "undefined") {
      return localStorage.getItem("email_validated") === "true";
    }
    return false;
  }

  logout(): void {
    this.tokenRepository.remove();
    if (typeof window !== "undefined") {
      localStorage.removeItem("email_validated");
    }
    this.authRepository.logout();
  }
}
