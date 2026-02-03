.PHONY: test format check 

test:
	docker compose run --rm test cargo test -- --test-threads=6

prepare:
	set -a && . ./.env && set +a && \
	docker compose run --rm -e DATABASE_URL="postgres://$${POSTGRES_USER}:$${POSTGRES_PASSWORD}@db:5432/$${POSTGRES_DB}" test cargo sqlx prepare

build:
	docker compose build

format:
	docker compose run --rm test cargo fmt

lint:
	docker compose run --rm test /bin/bash -c "chmod +x scripts/ddd-fitness-test.sh && ./scripts/ddd-fitness-test.sh"

check:
	docker compose run --rm test /bin/bash -c "cargo check && cargo clippy --all-targets --all-features -- -D warnings && chmod +x scripts/ddd-fitness-test.sh && ./scripts/ddd-fitness-test.sh && cargo test -- --test-threads=6"

run:
	docker compose up --build

stop:
	docker compose stop

down:
	docker compose down

logs:
	docker compose logs -f