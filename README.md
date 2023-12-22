# Rust / Axum course

base on the work of [Jeremy Chone](https://www.youtube.com/@JeremyChone/featured) 

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

## Test

```sh
cargo watch -q -c -x "test -- --nocapture"

# Specific test with filter.
cargo watch -q -c -x "test model::task::tests::test_create -- --nocapture"
```


## Database 

Create a local Postgres DB using Docker

```shell
# Start postgresql server docker image:
docker run --rm --name pg -p 5432:5432 \
   -e POSTGRES_PASSWORD=welcome \
   postgres:15

# (optional) To have a psql terminal on pg. 
# In another terminal (tab) run psql:
docker exec -it -u postgres pg psql

# (optional) For pg to print all sql statements.
# In psql command line started above.
ALTER DATABASE postgres SET log_statement = 'all';
```


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

