import { ValueObject } from "@/shared/domain/value-object";

export class AppliedDate extends ValueObject<string> {
  constructor(value: string) {
    super(value);
    this.validate(value);
  }

  private validate(value: string): void {
    const date = new Date(value);
    if (isNaN(date.getTime())) {
      throw new Error("Invalid date format");
    }
  }

  get value(): string {
    return this.props;
  }

  public formatDate(): string {
    return new Date(this.props).toLocaleDateString();
  }
}
