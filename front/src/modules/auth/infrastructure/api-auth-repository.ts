import { getBaseUrl } from "@/shared/api-config";
import { InfrastructureError, UnauthorizedError } from "@/shared/domain/errors";
import type { AuthRepository } from "../domain/auth-repository";
import type { LoginFormValues, RegisterFormValues } from "../domain/schema";

export class ApiAuthRepository implements AuthRepository {
  async register(data: RegisterFormValues): Promise<void> {
    const response = await fetch(`${getBaseUrl()}/auth/signup`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        email: data.email,
        password: data.password,
      }),
    });

    if (!response.ok) {
      throw new InfrastructureError("Error registering user", "REGISTER_ERROR", response.status);
    }
  }

  async login(data: LoginFormValues): Promise<{ access_token: string }> {
    const response = await fetch(`${getBaseUrl()}/auth/login`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        email: data.email,
        password: data.password,
      }),
    });

    if (!response.ok) {
      if (response.status === 401) {
        throw new UnauthorizedError("Invalid credentials");
      }
      throw new InfrastructureError("Login failed", "LOGIN_ERROR", response.status);
    }

    return response.json();
  }

  logout(): void {
    localStorage.removeItem("token");
    document.cookie = "token=; path=/; expires=Thu, 01 Jan 1970 00:00:01 GMT;";
  }
}
