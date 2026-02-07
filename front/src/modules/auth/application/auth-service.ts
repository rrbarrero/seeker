import type { AuthRepository } from "../domain/auth-repository";
import type { LoginFormValues, RegisterFormValues } from "../domain/schema";

export class AuthService {
  constructor(private readonly authRepository: AuthRepository) { }

  async register(data: RegisterFormValues): Promise<void> {
    return this.authRepository.register(data);
  }

  async login(data: LoginFormValues): Promise<{ access_token: string }> {
    return this.authRepository.login(data);
  }

  logout(): void {
    this.authRepository.logout();
  }
}
