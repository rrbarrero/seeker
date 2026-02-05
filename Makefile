.PHONY: test format check prepare build run stop down logs front-test front-lint front-type-check front-format front-check

# Rust Targets
test-rust:
	docker compose run --rm test cargo test -- --test-threads=6

format-rust:
	docker compose run --rm test cargo fmt

lint-rust:
	docker compose run --rm test /bin/bash -c "chmod +x scripts/ddd-fitness-test.sh && ./scripts/ddd-fitness-test.sh"

check-rust:
	docker compose run --rm test /bin/bash -c "cargo fmt --check && cargo check && cargo clippy --all-targets --all-features -- -D warnings && chmod +x scripts/ddd-fitness-test.sh && ./scripts/ddd-fitness-test.sh && cargo test -- --test-threads=6"

# Frontend Targets
front-test:
	cd front && pnpm vitest run

front-lint:
	cd front && pnpm lint

front-type-check:
	cd front && pnpm type-check

front-format:
	cd front && pnpm format

front-check: front-lint front-type-check front-test

# Global Targets
test: test-rust front-test

format: format-rust front-format

check: check-rust front-check

prepare:
	set -a && . ./.env && set +a && \
	docker compose run --rm -e DATABASE_URL="postgres://$${POSTGRES_USER}:$${POSTGRES_PASSWORD}@db:5432/$${POSTGRES_DB}" test cargo sqlx prepare

build:
	docker compose build

run:
	docker compose up --build

stop:
	docker compose stop

down:
	docker compose down

logs:
	docker compose logs -f