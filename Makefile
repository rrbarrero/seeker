.PHONY: test 

test:
	docker compose run --rm test cargo test
