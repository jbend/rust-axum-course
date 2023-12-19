# Rust / Axum course

by [Jeremy Chone](https://www.youtube.com/@JeremyChone/featured) 

Rust web app with warp, sqlx, Postgres

https://youtu.be/VIig9IcQ-w8?si=tSF9oEHDg4tWRxqX

## Commands

Run / watch

```shell
# Run server with watch
cargo watch -q -c -w src/ -w .cargo/ -x run

# Run tests with watch
cargo watch -q -c -w examples/ -x "run --example quick_dev"

```

## Database 

Create a local Postgres DB using Docker

```sh
# Terminal 1 - start postgresql original 
docker run --rm -p 5432:5432 -e "POSTGRES_PASSWORD=postgres" --name pg postgres:14

docker run --rm -p 5432:5432 -e "POSTGRES_PASSWORD=postgres" --name pg postgres
```

## Errata

```sh
# Terminal 2 - build frontend
cd frontend
npm run build

# Terminal 3 - build backend
cd backend
cargo run -- ../frontend/web-folder
```

