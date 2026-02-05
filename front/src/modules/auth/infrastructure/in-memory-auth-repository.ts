import type { AuthRepository } from "../domain/auth-repository";
import type { LoginFormValues, RegisterFormValues } from "../domain/schema";

export class InMemoryAuthRepository implements AuthRepository {
  private users: Map<string, string> = new Map(); // email -> password

  async register(data: RegisterFormValues): Promise<void> {
    // Simulate network delay
    await new Promise((resolve) => setTimeout(resolve, 500));

    if (this.users.has(data.email)) {
      throw new Error("User already exists");
    }

    this.users.set(data.email, data.password);
  }

  async login(data: LoginFormValues): Promise<{ access_token: string }> {
    // Simulate network delay
    await new Promise((resolve) => setTimeout(resolve, 500));

    const password = this.users.get(data.email);

    if (password !== data.password) {
      throw new Error("Invalid credentials");
    }

    return { access_token: "fake-jwt-token" };
  }
}
