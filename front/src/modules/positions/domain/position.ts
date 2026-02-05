export type PositionStatus =
  | "CvSent"
  | "PhoneScreenScheduled"
  | "TechnicalInterview"
  | "OfferReceived"
  | "Rejected"
  | "Withdrawn";

export interface Position {
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

export type CreatePositionInput = Omit<
  Position,
  "id" | "user_id" | "created_at" | "updated_at" | "deleted_at" | "deleted"
>;
