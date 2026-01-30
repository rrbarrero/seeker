.PHONY: test 

test:
	docker compose run --rm test cargo test

prepare:
	set -a && . ./.env && set +a && \
	export DATABASE_URL="postgres://$${POSTGRES_USER}:$${POSTGRES_PASSWORD}@$${POSTGRES_HOST}:$${POSTGRES_PORT}/$${POSTGRES_DB}" && \
	cargo sqlx prepare

build:
	docker compose build

format:
	cargo -- fmt