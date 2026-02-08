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
      throw new Error("Invalid URL format");
    }
  }

  get value(): string {
    return this.props;
  }
}
