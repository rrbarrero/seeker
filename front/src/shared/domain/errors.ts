export abstract class BaseError extends Error {
  constructor(
    public readonly message: string,
    public readonly code?: string,
  ) {
    super(message);
    this.name = this.constructor.name;
    Object.setPrototypeOf(this, new.target.prototype);
  }
}

export class DomainError extends BaseError {
  constructor(message: string, code = "DOMAIN_ERROR") {
    super(message, code);
  }
}

export class ApplicationError extends BaseError {
  constructor(message: string, code = "APPLICATION_ERROR") {
    super(message, code);
  }
}

export class InfrastructureError extends BaseError {
  constructor(
    message: string,
    code = "INFRASTRUCTURE_ERROR",
    public readonly status?: number,
  ) {
    super(message, code);
  }
}

export class UnauthorizedError extends InfrastructureError {
  constructor(message = "Unauthorized access") {
    super(message, "UNAUTHORIZED", 401);
  }
}

export class NotFoundError extends InfrastructureError {
  constructor(message = "Resource not found") {
    super(message, "NOT_FOUND", 404);
  }
}
