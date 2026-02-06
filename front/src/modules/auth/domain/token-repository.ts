export interface TokenRepository {
  save(token: string): void;
  get(): string | null;
  remove(): void;
}
