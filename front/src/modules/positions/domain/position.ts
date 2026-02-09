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
  user_id: string;
  company: string;
  role_title: string;
  description: string;
  applied_on: string;
  url: string;
  initial_comment: string;
  status: PositionStatus;
  created_at: string;
  updated_at: string;
  deleted_at: string | null;
  deleted: boolean;
}

export class Position {
  private _applied_on: AppliedDate;
  private _url: PositionUrl;

  constructor(private readonly props: PositionProps) {
    this.validate();
    this._applied_on = new AppliedDate(props.applied_on);
    this._url = new PositionUrl(props.url);
  }

  private validate(): void {
    if (!this.props.company || this.props.company.trim() === "") {
      throw new DomainError("Company name is required", "MISSING_COMPANY");
    }
    if (!this.props.role_title || this.props.role_title.trim() === "") {
      throw new DomainError("Role title is required", "MISSING_ROLE_TITLE");
    }
  }

  get id(): string {
    return this.props.id;
  }
  get user_id(): string {
    return this.props.user_id;
  }
  get company(): string {
    return this.props.company;
  }
  get role_title(): string {
    return this.props.role_title;
  }
  get description(): string {
    return this.props.description;
  }
  get applied_on(): string {
    return this._applied_on.value;
  }
  get url(): string {
    return this._url.value;
  }
  get initial_comment(): string {
    return this.props.initial_comment;
  }
  get status(): PositionStatus {
    return this.props.status;
  }
  get created_at(): string {
    return this.props.created_at;
  }
  get updated_at(): string {
    return this.props.updated_at;
  }
  get deleted_at(): string | null {
    return this.props.deleted_at;
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
      Omit<PositionProps, "id" | "user_id" | "created_at" | "updated_at" | "deleted" | "deleted_at">
    >,
  ): void {
    if (!this.canBeEdited()) {
      throw new DomainError("Cannot edit a deleted or finalized position", "POSITION_LOCKED");
    }

    if (props.company !== undefined) this.props.company = props.company;
    if (props.role_title !== undefined) this.props.role_title = props.role_title;
    if (props.description !== undefined) this.props.description = props.description;
    if (props.url !== undefined) {
      this._url = new PositionUrl(props.url); // Re-validate
      this.props.url = props.url;
    }
    if (props.applied_on !== undefined) {
      this._applied_on = new AppliedDate(props.applied_on); // Re-validate
      this.props.applied_on = props.applied_on;
    }
    if (props.initial_comment !== undefined) this.props.initial_comment = props.initial_comment;

    this.validate();
    this.touch();
  }

  public delete(): void {
    this.props.deleted = true;
    this.props.deleted_at = new Date().toISOString();
    this.touch();
  }

  private touch(): void {
    this.props.updated_at = new Date().toISOString();
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
  "id" | "user_id" | "created_at" | "updated_at" | "deleted_at" | "deleted"
>;
