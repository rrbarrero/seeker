.PHONY: test 

test:
	docker compose run --rm test cargo test

prepare:
	set -a && . ./.env && set +a && \
	docker compose run --rm -e DATABASE_URL="postgres://$${POSTGRES_USER}:$${POSTGRES_PASSWORD}@db:5432/$${POSTGRES_DB}" test cargo sqlx prepare

build:
	docker compose build

format:
	cargo -- fmt