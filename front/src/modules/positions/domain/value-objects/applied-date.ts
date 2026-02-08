import { DomainError } from "@/shared/domain/errors";
import { ValueObject } from "@/shared/domain/value-object";

export class AppliedDate extends ValueObject<string> {
  constructor(value: string) {
    super(value);
    this.validate(value);
  }

  private validate(value: string): void {
    const date = new Date(value);
    if (isNaN(date.getTime())) {
      throw new DomainError("Invalid date format", "INVALID_DATE");
    }
  }

  get value(): string {
    return this.props;
  }

  public formatDate(): string {
    return new Date(this.props).toLocaleDateString();
  }
}
