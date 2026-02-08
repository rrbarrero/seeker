import { toast } from "sonner";
import { BaseError, DomainError, InfrastructureError, UnauthorizedError } from "../domain/errors";

export class UiErrorHandler {
  public static handle(error: unknown, fallbackMessage = "An unexpected error occurred"): void {
    console.error(error);

    if (error instanceof UnauthorizedError) {
      toast.error("Session expired", {
        description: "Please login again to continue.",
      });
      // Optionally redirect to login, but usually the middleware handles this
      return;
    }

    if (error instanceof DomainError) {
      toast.error("Validation Error", {
        description: error.message,
      });
      return;
    }

    if (error instanceof InfrastructureError) {
      toast.error("Server Error", {
        description: error.message || "Failed to communicate with the server.",
      });
      return;
    }

    if (error instanceof BaseError) {
      toast.error("Error", {
        description: error.message,
      });
      return;
    }

    if (error instanceof Error) {
      toast.error("Error", {
        description: error.message || fallbackMessage,
      });
      return;
    }

    toast.error("Error", {
      description: fallbackMessage,
    });
  }
}
