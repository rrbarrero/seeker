import { InfrastructureError, UnauthorizedError } from "@/shared/domain/errors";
import { requestJson, requestEmpty } from "@/shared/http-client";
import type { AuthRepository } from "../domain/auth-repository";
import type { LoginFormValues, RegisterFormValues } from "../domain/schema";

export class ApiAuthRepository implements AuthRepository {
  async register(data: RegisterFormValues): Promise<void> {
    const { response } = await requestEmpty("/auth/signup", {
      method: "POST",
      body: {
        email: data.email,
        password: data.password,
      },
    });

    if (!response.ok) {
      throw new InfrastructureError("Error registering user", "REGISTER_ERROR", response.status);
    }
  }

  async login(data: LoginFormValues): Promise<{ access_token: string; email_validated: boolean }> {
    const { response, data: payload } = await requestJson<{
      access_token: string;
      email_validated: boolean;
    }>("/auth/login", {
      method: "POST",
      body: {
        email: data.email,
        password: data.password,
      },
    });

    if (!response.ok) {
      if (response.status === 401) {
        throw new UnauthorizedError("Invalid credentials");
      }
      throw new InfrastructureError("Login failed", "LOGIN_ERROR", response.status);
    }

    if (!payload) {
      throw new InfrastructureError("Login failed", "LOGIN_ERROR", response.status);
    }

    return payload;
  }

  async verifyEmail(token: string): Promise<void> {
    const { response } = await requestEmpty(
      `/auth/verify-email?token=${encodeURIComponent(token)}`,
      {
        method: "GET",
      },
    );

    if (!response.ok) {
      if (response.status === 401) {
        throw new UnauthorizedError("The verification link has expired or is invalid.");
      }
      if (response.status === 404) {
        throw new InfrastructureError("User not found", "USER_NOT_FOUND", response.status);
      }
      throw new InfrastructureError(
        "Email verification failed",
        "VERIFY_EMAIL_ERROR",
        response.status,
      );
    }
  }

  logout(): void {
    localStorage.removeItem("token");
    document.cookie = "token=; path=/; expires=Thu, 01 Jan 1970 00:00:01 GMT;";
  }
}
