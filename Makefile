.PHONY: test 

test:
	set -a && . ./.env.test && set +a && cargo test

prepare:
	set -a && . ./.env && set +a && \
	export DATABASE_URL="postgres://$${POSTGRES_USER}:$${POSTGRES_PASSWORD}@$${POSTGRES_HOST}:$${POSTGRES_PORT}/$${POSTGRES_DB}" && \
	cargo sqlx prepare

build:
	docker compose build