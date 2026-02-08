import { DomainError } from "@/shared/domain/errors";
import { ValueObject } from "@/shared/domain/value-object";

export class PositionUrl extends ValueObject<string> {
  constructor(value: string) {
    super(value);
    this.validate(value);
  }

  private validate(value: string): void {
    if (value === "") return;
    try {
      new URL(value);
    } catch {
      throw new DomainError("Invalid URL format", "INVALID_URL");
    }
  }

  get value(): string {
    return this.props;
  }
}
