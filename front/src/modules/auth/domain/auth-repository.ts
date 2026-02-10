import type { LoginFormValues, RegisterFormValues } from "./schema";

export interface AuthRepository {
  register(data: RegisterFormValues): Promise<void>;
  login(data: LoginFormValues): Promise<{ access_token: string; email_validated: boolean }>;
  verifyEmail(token: string): Promise<void>;
  logout(): void;
}
