import { getBaseUrl } from "@/shared/api-config";
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
      throw new Error("Error registering user");
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
      throw new Error("Invalid credentials");
    }

    return response.json();
  }
}
