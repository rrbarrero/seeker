.PHONY: test format check prepare build run stop down logs front-test front-lint front-type-check front-format front-check
.PHONY: run-obs database-reset migrate rust-update

# Rust Targets
test-rust:
	docker compose run --rm test cargo test -- --test-threads=6

format-rust:
	docker compose run --rm test cargo fmt

lint-rust:
	docker compose run --rm test /bin/bash -c "chmod +x scripts/ddd-fitness-test.sh && ./scripts/ddd-fitness-test.sh"

check-rust:
	docker compose run --rm test /bin/bash -c "cargo fmt --check && cargo check && cargo clippy --all-targets --all-features -- -D warnings && chmod +x scripts/ddd-fitness-test.sh && ./scripts/ddd-fitness-test.sh && cargo test -- --test-threads=6"

coverage-rust:
	docker compose run --rm test cargo tarpaulin --config tarpaulin.toml --fail-under 80

# Frontend Targets
front-test:
	cd front && pnpm vitest run

front-lint:
	cd front && pnpm lint

front-type-check:
	cd front && pnpm type-check

front-format:
	cd front && pnpm format

front-build:
	cd front && pnpm build

front-check: front-lint front-type-check front-test front-build

# Global Targets
test: test-rust front-test

format: format-rust front-format

check: check-rust front-check

prepare:
	set -a && . ./.env && set +a && \
	docker compose run --rm -e DATABASE_URL="postgres://$${POSTGRES_USER}:$${POSTGRES_PASSWORD}@db:5432/$${POSTGRES_DB}" test cargo sqlx prepare -- --tests

build:
	docker compose build

run:
	docker compose up --build

run-obs:
	docker compose -f docker-compose.yml -f docker/observability/docker-compose.observability.yml up --build

stop:
	docker compose stop

down:
	docker compose down --remove-orphans

logs:
	docker compose logs -f

migrate:
	set -a && . ./.env && set +a && \
	docker compose run --rm -e DATABASE_URL="postgres://$${POSTGRES_USER}:$${POSTGRES_PASSWORD}@db:5432/$${POSTGRES_DB}" test cargo sqlx migrate run

database-reset:
	set -a && . ./.env && set +a && \
	docker compose run --rm -e DATABASE_URL="postgres://$${POSTGRES_USER}:$${POSTGRES_PASSWORD}@db:5432/$${POSTGRES_DB}" test cargo sqlx database reset -y

rust-update:
	docker compose run --rm test /bin/bash -c "cargo update && cd workers/email && cargo update"

# Garage / S3 Operations (Dev)
GARAGE_CONTAINER := best-seeker-garage

# Load env vars if .env exists
ifneq (,$(wildcard ./.env))
    include .env
    export
endif

# Force loading variables from .env strictly for this command to avoid conflicts with global AWS credentials
AWS_CMD := set -a && [ -f .env ] && . ./.env && set +a && \
	AWS_ACCESS_KEY_ID=$${AWS_ACCESS_KEY_ID} \
	AWS_SECRET_ACCESS_KEY=$${AWS_SECRET_ACCESS_KEY} \
	AWS_DEFAULT_REGION=$${AWS_REGION} \
	aws --endpoint-url $${AWS_ENDPOINT_URL}


garage-up:
	docker compose up -d garage
	./scripts/provision_garage.sh

garage-logs:
	docker compose logs -f garage

garage-status:
	docker exec $(GARAGE_CONTAINER) /garage status

garage-init-dev:
	@echo "Initializing Garage Dev Environment..."
	-docker exec $(GARAGE_CONTAINER) /garage key create dev-key
	-docker exec $(GARAGE_CONTAINER) /garage bucket create dev-bucket
	-docker exec $(GARAGE_CONTAINER) /garage bucket allow --read --write --owner dev-bucket --key dev-key
	@echo "Garage initialized."

garage-destroy:
	docker compose stop garage
	docker compose rm -f garage
	-docker volume rm best-seeker_garage_meta best-seeker_garage_data

s3-ls:
	$(AWS_CMD) s3 ls

garage-provision:
	./scripts/provision_garage.sh