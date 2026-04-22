# Best Seeker

Best Seeker is a full-stack job application tracker built around a Rust backend, a Next.js frontend, and two background workers.

At a product level, the application lets users:

- sign up and log in with JWT-based authentication,
- verify their email address,
- create, update, list, and soft-delete job applications,
- attach private comments to each application,
- enrich job applications with asynchronously scraped and AI-processed metadata.

The repository is organized as a modular monolith for the backend, plus independent workers for asynchronous tasks.

## Main Features

- User registration and login
- Email verification flow
- Protected API with JWT bearer tokens
- Job application management
- Per-position comments
- Soft deletion for positions
- Async email queue backed by PostgreSQL notifications
- Async scraping queue with S3-compatible object storage
- Optional observability with OpenTelemetry, Grafana, Tempo, Loki, and Prometheus
- OpenAPI docs and Swagger UI

## Architecture Overview

For a visual overview of the system, see [`best-seeker-architecture.html`](/home/roberto/devel/rust/seeker/best-seeker-architecture.html), an architecture diagram/document that complements this README.

### Backend

The backend lives in [`src/main.rs`](/home/roberto/devel/rust/seeker/src/main.rs) and is implemented in Rust with:

- `axum` for HTTP routing,
- `sqlx` for PostgreSQL access and migrations,
- `utoipa` + Swagger UI for API documentation,
- `jsonwebtoken` for JWT creation and validation,
- `tower-http` for CORS, tracing, and request IDs.

The backend follows a DDD-inspired modular structure:

- `src/auth`: authentication, registration, email verification, JWT, user persistence
- `src/positions`: job application and comment domain
- `src/shared`: config, HTTP middleware, observability, shared domain types
- `src/composition_root.rs`: dependency wiring

Each module is split into:

- `domain`: entities, value objects, repository interfaces, domain errors
- `application`: use-case services
- `infrastructure`: PostgreSQL adapters and external integrations
- `presentation`: HTTP handlers, DTOs, routes, API errors

### Frontend

The frontend lives in [`front/`](/home/roberto/devel/rust/seeker/front) and uses:

- Next.js 16
- React 19
- TypeScript
- Tailwind CSS 4
- React Hook Form + Zod
- Vitest for tests

The UI provides:

- login and registration screens,
- email verification pages,
- a dashboard listing job applications,
- create/update flows for positions,
- a detail page with comments.

### Workers

#### Email worker

The email worker lives in [`workers/email/src/main.rs`](/home/roberto/devel/rust/seeker/workers/email/src/main.rs).

It listens to PostgreSQL `LISTEN/NOTIFY` events on the `email_queue` table, processes pending jobs, and currently sends emails through a stdout sender intended for development. Its main use case is email verification after signup.

#### Scraper worker

The scraper worker lives in [`workers/scraper/src/main.py`](/home/roberto/devel/rust/seeker/workers/scraper/src/main.py).

It polls the `scraper_queue` table, scrapes the job posting URL, runs an analysis step, and uploads structured JSON to S3-compatible storage using Garage. The analysis layer supports:

- a fake analyzer for local development,
- a Groq-powered analyzer for LLM extraction.

## Data Model

The main database entities are created through SQL migrations in [`migrations/`](/home/roberto/devel/rust/seeker/migrations):

- `users`
- `positions`
- `comments`
- `email_queue`
- `scraper_queue`

Notable behavior:

- `positions` support soft deletion through `deleted` and `deleted_at`
- `comments` belong to a position and are deleted with it at the database level
- `email_queue` emits PostgreSQL notifications on insert
- `scraper_queue` stores job status, retry metadata, trace IDs, and S3 object keys

## Authentication and Authorization

Authentication is handled by the `auth` module.

- Passwords are hashed with `argon2`
- Login returns a JWT bearer token
- Protected routes require `Authorization: Bearer <token>`
- Tokens include the user ID (`sub`) and email
- Disabled accounts are rejected by the auth extractor
- Signup enqueues an email verification message
- Email verification is completed through `GET /auth/verify-email?token=...`

## API Summary

The backend exposes:

- `POST /auth/signup`
- `POST /auth/login`
- `GET /auth/verify-email`
- `GET /positions`
- `GET /positions/{id}`
- `POST /positions`
- `PUT /positions/{id}`
- `DELETE /positions/{id}`
- `GET /positions/{position_id}/comments`
- `GET /positions/{position_id}/comments/{comment_id}`
- `POST /positions/{position_id}/comments`
- `PUT /positions/{position_id}/comments/{comment_id}`
- `DELETE /positions/{position_id}/comments/{comment_id}`

Swagger UI is mounted at:

- `http://localhost:3000/swagger-ui`

OpenAPI JSON is served at:

- `http://localhost:3000/api-docs/openapi.json`

## Local Development

### Prerequisites

- Docker and Docker Compose
- `make`
- `pnpm` for frontend-only local commands

### Environment Variables

The repository includes an example environment file at [`env-example`](/home/roberto/devel/rust/seeker/env-example).

Typical setup:

```bash
cp env-example .env
```

Important variables:

- `DATABASE_URL`: PostgreSQL connection string
- `JWT_SECRET`: secret used to sign JWTs
- `CORS_ALLOWED_ORIGIN`: allowed frontend origin
- `FRONTEND_URL`: base URL used in email verification links
- `OBS_ENABLED`: enables OpenTelemetry exporters
- `RATE_LIMIT_ENABLED`: enables API rate limiting
- `AWS_ACCESS_KEY_ID`, `AWS_SECRET_ACCESS_KEY`, `AWS_ENDPOINT_URL`: S3-compatible storage config
- `LLM_SELECTED`: `fake` or `groq`
- `GROQ_API_TOKEN`, `GROQ_MODEL`: Groq LLM configuration

### Start the Full Stack

```bash
make run
```

This starts:

- PostgreSQL
- Garage (S3-compatible object storage)
- Rust backend on `http://localhost:3000`
- Next.js frontend on `http://localhost:3001`
- email worker
- scraper worker

### Start with Observability

```bash
make run-obs
```

This adds the observability stack from [`docker/observability/`](/home/roberto/devel/rust/seeker/docker/observability).

### Other Useful Commands

```bash
make build
make stop
make down
make logs
make migrate
make database-reset
```

## Testing and Quality Checks

### Rust

```bash
make test-rust
make lint-rust
make check-rust
make coverage-rust
```

The Rust checks include:

- formatting,
- compilation,
- clippy with warnings denied,
- a DDD fitness script,
- unit and integration tests.

### Frontend

```bash
make front-test
make front-lint
make front-type-check
make front-build
make front-check
```

### Full Project

```bash
make test
make format
make check
```

## Storage and Async Processing

### Email queue

When a new user signs up:

1. the backend stores the user,
2. it generates a verification token,
3. it inserts a job into `email_queue`,
4. PostgreSQL emits a notification,
5. the email worker consumes the job and sends the message.

### Scraper queue

The scraper workflow is designed for async enrichment:

1. a row is inserted into `scraper_queue`,
2. the scraper worker fetches the pending job,
3. the target page is scraped,
4. the content is analyzed,
5. the result is uploaded to Garage/S3 as JSON,
6. the job is marked as completed or failed.

## Observability

The project includes optional observability support:

- OpenTelemetry traces and logs
- request IDs propagated through the API
- Grafana dashboards
- Tempo for traces
- Loki for logs
- Prometheus for metrics collection

When `OBS_ENABLED=true`, both the backend and workers attempt to export telemetry using OTLP endpoints.

## Repository Structure

```text
.
├── src/                     # Rust backend
├── front/                   # Next.js frontend
├── workers/
│   ├── email/               # Rust email worker
│   └── scraper/             # Python scraper worker
├── migrations/              # SQL migrations
├── docker/                  # Docker and observability config
├── scripts/                 # Helper scripts
├── best-seeker-architecture.html  # Visual architecture document
├── docker-compose.yml       # Local orchestration
└── Makefile                 # Common development commands
```

## Notes for Contributors

- The backend is structured around explicit modules rather than a flat CRUD service layout.
- SQL schema evolution is managed through migration files.
- The frontend and backend are intentionally decoupled and communicate through HTTP APIs.
- The workers are independent processes and can be scaled or replaced separately.
- Swagger UI is the quickest way to inspect and test the backend contract during development.

## Current Technical Caveats

- The email worker currently uses a stdout sender, so it is development-oriented unless a real provider is added.
- The scraper worker depends on external page structure and, when enabled, on Groq API availability.
- Garage is used as local S3-compatible storage for development; production deployments may swap it for another S3-compatible service.

## License

No license file is present in this repository at the time of writing.
