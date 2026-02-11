.PHONY: test format check prepare build run stop down logs front-test front-lint front-type-check front-format front-check
.PHONY: run-obs database-reset migrate rust-update

# --- Rust Targets ---

# Run Rust unit and integration tests
test-rust:
	docker compose run --rm test cargo test -- --test-threads=6

# Format Rust code following project rules
format-rust:
	docker compose run --rm test cargo fmt

# Run linter and architecture tests (fitness tests)
lint-rust:
	docker compose run --rm test /bin/bash -c "chmod +x scripts/ddd-fitness-test.sh && ./scripts/ddd-fitness-test.sh"

# Run all checks: formatting, compilation, clippy, architecture, and unit tests
check-rust:
	docker compose run --rm test /bin/bash -c "cargo fmt --check && cargo check && cargo clippy --all-targets --all-features -- -D warnings && chmod +x scripts/ddd-fitness-test.sh && ./scripts/ddd-fitness-test.sh && cargo test -- --test-threads=6"

# Generate code coverage report (requires tarpaulin)
coverage-rust:
	docker compose run --rm test cargo tarpaulin --config tarpaulin.toml --fail-under 80

# --- Frontend Targets ---

# Run frontend tests with Vitest
front-test:
	cd front && pnpm vitest run

# Run linter for frontend code
front-lint:
	cd front && pnpm lint

# Run TypeScript type checking for frontend
front-type-check:
	cd front && pnpm type-check

# Format frontend code
front-format:
	cd front && pnpm format

# Generate production build for frontend
front-build:
	cd front && pnpm build

# Run all frontend checks
front-check: front-lint front-type-check front-test front-build

# --- Global Targets ---

# Run all tests (Rust and Frontend)
test: test-rust front-test

# Format the entire project
format: format-rust front-format

# Run all project checks
check: check-rust front-check

# Prepare SQLx environment for offline compilation
prepare:
	set -a && . ./.env && set +a && \
	docker compose run --rm -e DATABASE_URL="postgres://$${POSTGRES_USER}:$${POSTGRES_PASSWORD}@db:5432/$${POSTGRES_DB}" test cargo sqlx prepare -- --tests

# Build all Docker images
build:
	docker compose build

# Start all services with hot-reload
run:
	docker compose up --build

# Start the full stack including observability (Grafana, Tempo, etc.)
run-obs:
	docker compose -f docker-compose.yml -f docker/observability/docker-compose.observability.yml up --build

# Stop containers without removing them
stop:
	docker compose stop

# Stop and remove containers and networks created by docker-compose
down:
	docker compose down --remove-orphans

# Show logs from all containers in real-time
logs:
	docker compose logs -f

# Run database migrations
migrate:
	set -a && . ./.env && set +a && \
	docker compose run --rm -e DATABASE_URL="postgres://$${POSTGRES_USER}:$${POSTGRES_PASSWORD}@db:5432/$${POSTGRES_DB}" test cargo sqlx migrate run

# Reset and recreate the database from scratch
database-reset:
	set -a && . ./.env && set +a && \
	docker compose run --rm -e DATABASE_URL="postgres://$${POSTGRES_USER}:$${POSTGRES_PASSWORD}@db:5432/$${POSTGRES_DB}" test cargo sqlx database reset -y

# Update Rust dependencies (Cargo.lock)
rust-update:
	docker compose run --rm test /bin/bash -c "cargo update && cd workers/email && cargo update"

# --- Garage / S3 Operations (Dev) ---

GARAGE_CONTAINER := best-seeker-garage

# Load environment variables for S3 commands
ifneq (,$(wildcard ./.env))
    include .env
    export
endif

# Optimized base command to interact with Garage using AWS CLI
AWS_CMD := set -a && [ -f .env ] && . ./.env && set +a && \
	AWS_ACCESS_KEY_ID=$${AWS_ACCESS_KEY_ID} \
	AWS_SECRET_ACCESS_KEY=$${AWS_SECRET_ACCESS_KEY} \
	AWS_DEFAULT_REGION=$${AWS_REGION} \
	aws --endpoint-url $${AWS_ENDPOINT_URL}

# Start Garage service and create initial buckets/keys
garage-up:
	docker compose up -d garage
	./scripts/provision_garage.sh

# Show logs from the Garage service
garage-logs:
	docker compose logs -f garage

# Show current status of the Garage cluster
garage-status:
	docker exec $(GARAGE_CONTAINER) /garage status

# Initialize development buckets and keys manually
garage-init-dev:
	@echo "Initializing Garage Dev Environment..."
	-docker exec $(GARAGE_CONTAINER) /garage key create dev-key
	-docker exec $(GARAGE_CONTAINER) /garage bucket create dev-bucket
	-docker exec $(GARAGE_CONTAINER) /garage bucket allow --read --write --owner dev-bucket --key dev-key
	@echo "Garage initialized."

# Stop Garage and remove its data volumes (full cleanup)
garage-destroy:
	docker compose stop garage
	docker compose rm -f garage
	-docker volume ls -q | grep -E "garage_meta|garage_data" | xargs -r docker volume rm

# List contents of a generic bucket (use BUCKET=name)
s3-ls:
	$(AWS_CMD) s3 ls $(BUCKET)

# Recursively list contents of the scraper bucket
s3-ls-scraper:
	$(AWS_CMD) s3 ls s3://$${S3_SCRAPER_BUCKET} --recursive

# Download a specific file from S3 (use S3_PATH=path/file.json)
s3-get:
	@if [ -z "$(S3_PATH)" ]; then echo "Usage: make s3-get S3_PATH=scraper/user_id/pos_id.html [DEST=./file.html]"; exit 1; fi
	$(AWS_CMD) s3 cp s3://$${S3_SCRAPER_BUCKET}/$(S3_PATH) $(if $(DEST),$(DEST),.)

# Run Garage provisioning script
garage-provision:
	./scripts/provision_garage.sh

# --- Scraper Worker Targets ---

# Simulate a new job by inserting directly into the DB (use URL=https://...)
scraper-test:
	@if [ -z "$(URL)" ]; then echo "Usage: make scraper-test URL=https://example.com"; exit 1; fi
	docker compose exec -T db psql -U $${POSTGRES_USER} -d $${POSTGRES_DB} -c \
		"INSERT INTO scraper_queue (url, user_id, position_id, status) VALUES ('$(URL)', '00000000-0000-0000-0000-000000000000', '00000000-0000-0000-0000-000000000000', 'PENDING');"

# Start only the scraper worker
scraper-up:
	docker compose up -d scraper

# Show logs from the scraper worker
scraper-logs:
	docker compose logs -f scraper

# Stop the scraper worker
scraper-stop:
	docker compose stop scraper