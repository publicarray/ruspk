# Install

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

```toml
DATABASE_URL="mysql://user:pass@localhost/dbname"
DATABASE_URL="postgresql://user:pass@localhost/dbname"
DATABASE_URL="/tmp/database_file.db"
```

# Guides

https://diesel.rs/

https://actix.rs/
