# Install

## Development

```sh
cargo install diesel_cli
cargo run
```

## Production

```sh
# export DATABASE_URL=mysql://user:pass@localhost/dbname
diesel migration run

cargo build --release
ROCKET_ENV=prod target/release/ruspk
#or 
ROCKET_ENV=prod cargo run --release
```

# Guides

https://diesel.rs/

https://rocket.rs/
