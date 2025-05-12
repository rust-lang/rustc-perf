.PHONY: test start-postgres

# Helper to spin up postgres
start-postgres:
	docker compose up -d pg_test

# Helper to stop postgres & destroy the volume (ensures a clean rebuild on `start`)
stop-postgres:
	docker compose down -v

# Run the tests with the database purring along in the background
test: start-postgres
	TEST_DB_URL="postgres://postgres:testpass@localhost/postgres" cargo test
