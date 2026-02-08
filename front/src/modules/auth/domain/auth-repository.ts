import type { LoginFormValues, RegisterFormValues } from "./schema";

export interface AuthRepository {
  register(data: RegisterFormValues): Promise<void>;
  login(data: LoginFormValues): Promise<{ access_token: string }>;
  logout(): void;
}
