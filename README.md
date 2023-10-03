# Rust Web API - cr8s

## Setup Diesel

```bash
docker compose exec app diesel setup

# Generate migration
docker compose exec app diesel migration generate create_rustaceans
docker compose exec app diesel migration generate create_crates

# Run migration
docker compose exec app diesel migration run

# Revert migration, it will revert the last migration, and one by one
docker compose exec app diesel migration revert
```

## Run Cargo

```bash
docker compose exec app cargo run
```

## Run Curl to test

```bash
# Create a new Rustacean
docker compose exec app curl -X POST -H "Content-Type: application/json" -d '{"name": "John Doe", "email": "john@doe.com"}' http://localhost:8000/rustaceans

# Get all Rustaceans
docker compose exec app curl http://localhost:8000/rustaceans

# Get a Rustacean by id
docker compose exec app curl http://localhost:8000/rustaceans/1

# Update a Rustacean by id
docker compose exec app curl -X PUT -H "Content-Type: application/json" -d '{"name": "Johnie Doe", "email": "johnie@doe.com"}' http://localhost:8000/rustaceans/1
```