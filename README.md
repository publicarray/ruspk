# Install

```sh
cargo install --feature postgres
ruspk
```

## Development

```sh
echo 'export DATABASE_URL=mysql://user:pass@localhost/dbname' > .env
cargo install diesel_cli
diesel migration --migration-dir migrations/mysql/ run
cargo run --feature mysql
```

Avaliable Features: `mysql`, `postgres` and `sqlite`

## Production

```sh
echo 'export DATABASE_URL=mysql://user:pass@localhost/dbname' > .env
cargo install diesel_cli
diesel migration --migration-dir migrations/mysql/ run

cargo build --release --feature mysql
target/release/ruspk
#or
cargo run --release --feature mysql
```

## Configuration `.env`

```env
RUST_LOG="actix_web=warn,diesel=warn"
DATABASE_URL=file:db/database.sqlite
DATABASE_URL=mysql://user:pass@localhost/dbname
DATABASE_URL=postgresql://user:pass@localhost/dbname
LISTEN=127.0.0.1
PORT=80
```

# Guides

https://diesel.rs/

https://actix.rs/
