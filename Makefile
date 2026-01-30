.PHONY: test format check 

test:
	docker compose run --rm test cargo test -- --test-threads=8

prepare:
	set -a && . ./.env && set +a && \
	docker compose run --rm -e DATABASE_URL="postgres://$${POSTGRES_USER}:$${POSTGRES_PASSWORD}@db:5432/$${POSTGRES_DB}" test cargo sqlx prepare

build:
	docker compose build

format:
	docker compose run --rm test cargo fmt

check:
	docker compose run --rm test cargo check 
	docker compose run --rm test cargo clippy --all-targets --all-features -- -D warnings
	docker compose run --rm test cargo test -- --test-threads=8