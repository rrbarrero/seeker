import { DomainError } from "@/shared/domain/errors";
import { AppliedDate } from "./value-objects/applied-date";
import { PositionUrl } from "./value-objects/position-url";

export type PositionStatus =
  | "CvSent"
  | "PhoneScreenScheduled"
  | "TechnicalInterview"
  | "OfferReceived"
  | "Rejected"
  | "Withdrawn";

export interface PositionProps {
  id: string;
  userId: string;
  company: string;
  roleTitle: string;
  description: string;
  appliedOn: string;
  url: string;
  initialComment: string;
  status: PositionStatus;
  createdAt: string;
  updatedAt: string;
  deletedAt: string | null;
  deleted: boolean;
}

export class Position {
  private _applied_on: AppliedDate;
  private _url: PositionUrl;

  constructor(private readonly props: PositionProps) {
    this.validate();
    this._applied_on = new AppliedDate(props.appliedOn);
    this._url = new PositionUrl(props.url);
  }

  private validate(): void {
    if (!this.props.company || this.props.company.trim() === "") {
      throw new DomainError("Company name is required", "MISSING_COMPANY");
    }
    if (!this.props.roleTitle || this.props.roleTitle.trim() === "") {
      throw new DomainError("Role title is required", "MISSING_ROLE_TITLE");
    }
  }

  get id(): string {
    return this.props.id;
  }
  get userId(): string {
    return this.props.userId;
  }
  get company(): string {
    return this.props.company;
  }
  get roleTitle(): string {
    return this.props.roleTitle;
  }
  get description(): string {
    return this.props.description;
  }
  get appliedOn(): string {
    return this._applied_on.value;
  }
  get url(): string {
    return this._url.value;
  }
  get initialComment(): string {
    return this.props.initialComment;
  }
  get status(): PositionStatus {
    return this.props.status;
  }
  get createdAt(): string {
    return this.props.createdAt;
  }
  get updatedAt(): string {
    return this.props.updatedAt;
  }
  get deletedAt(): string | null {
    return this.props.deletedAt;
  }
  get deleted(): boolean {
    return this.props.deleted;
  }

  // Business Logic
  public canBeEdited(): boolean {
    return !this.props.deleted && this.props.status !== "Rejected";
  }

  public getFormattedAppliedDate(): string {
    return this._applied_on.formatDate();
  }

  public advanceStatus(newStatus: PositionStatus): void {
    if (this.props.status === newStatus) {
      return;
    }

    const forbiddenTransitions: Record<PositionStatus, PositionStatus[]> = {
      Rejected: [
        "CvSent",
        "PhoneScreenScheduled",
        "TechnicalInterview",
        "OfferReceived",
        "Withdrawn",
      ], // Added Withdrawn
      Withdrawn: [
        "CvSent",
        "PhoneScreenScheduled",
        "TechnicalInterview",
        "OfferReceived",
        "Rejected",
      ], // Added Rejected
      OfferReceived: ["CvSent", "PhoneScreenScheduled", "TechnicalInterview"],
      CvSent: [],
      PhoneScreenScheduled: ["CvSent"],
      TechnicalInterview: ["CvSent", "PhoneScreenScheduled"],
    };

    if (forbiddenTransitions[this.props.status]?.includes(newStatus)) {
      throw new DomainError(
        `Cannot transition from ${this.props.status} to ${newStatus}`,
        "INVALID_STATUS_TRANSITION",
      );
    }

    this.props.status = newStatus;
    this.touch();
  }

  public update(
    props: Partial<
      Omit<PositionProps, "id" | "userId" | "createdAt" | "updatedAt" | "deleted" | "deletedAt">
    >,
  ): void {
    if (!this.canBeEdited()) {
      throw new DomainError("Cannot edit a deleted or finalized position", "POSITION_LOCKED");
    }

    if (props.company !== undefined) this.props.company = props.company;
    if (props.roleTitle !== undefined) this.props.roleTitle = props.roleTitle;
    if (props.description !== undefined) this.props.description = props.description;
    if (props.url !== undefined) {
      this._url = new PositionUrl(props.url); // Re-validate
      this.props.url = props.url;
    }
    if (props.appliedOn !== undefined) {
      this._applied_on = new AppliedDate(props.appliedOn); // Re-validate
      this.props.appliedOn = props.appliedOn;
    }
    if (props.initialComment !== undefined) this.props.initialComment = props.initialComment;

    this.validate();
    this.touch();
  }

  public delete(): void {
    this.props.deleted = true;
    this.props.deletedAt = new Date().toISOString();
    this.touch();
  }

  private touch(): void {
    this.props.updatedAt = new Date().toISOString();
  }

  public static fromPrimitives(props: PositionProps): Position {
    return new Position(props);
  }

  public toPrimitives(): PositionProps {
    return { ...this.props };
  }
}

export type CreatePositionInput = Omit<
  PositionProps,
  "id" | "userId" | "createdAt" | "updatedAt" | "deletedAt" | "deleted"
>;
