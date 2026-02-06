import type { TokenRepository } from "../domain/token-repository";

export class CookieTokenRepository implements TokenRepository {
  private readonly cookieName = "token";

  save(token: string): void {
    if (typeof window !== "undefined") {
      document.cookie = `${this.cookieName}=${token}; path=/; max-age=${60 * 60 * 24 * 7}; SameSite=Lax`;
    }
  }

  get(): string | null {
    if (typeof window !== "undefined") {
      const match = document.cookie
        .split("; ")
        .find((row) => row.startsWith(`${this.cookieName}=`));

      return match ? match.split("=")[1] : null;
    }
    return null;
  }

  remove(): void {
    if (typeof window !== "undefined") {
      document.cookie = `${this.cookieName}=; path=/; expires=Thu, 01 Jan 1970 00:00:01 GMT`;
    }
  }
}
