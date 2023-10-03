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