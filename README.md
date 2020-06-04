# spkrepo-rs
[![ruspk's current version badge](https://img.shields.io/crates/v/ruspk.svg)](https://crates.io/crates/ruspk)

ruspk is a simple and fast synology repository server. It uses the existing database structure from [spkrepo](https://github.com/SynoCommunity/spkrepo)

Only the GET API for the synology devices is supported. You have to update the database yourself or run it in conjunction with spkrepo and a reverse proxy like nginx.

## Install

```sh
cargo install diesel_cli
cargo install ruspk --features postgres
echo 'DATABASE_URL=postgresql://user:pass@localhost/dbname' > .env
diesel migration --migration-dir migrations/postgres/ run
ruspk
```

Avaliable Features: `mysql`, `postgres` and `sqlite`

### Test the API

```sh
curl -sv 'http://127.0.0.1:80/?package_update_channel=beta&unique=synology_apollolake_418play&build=24922&language=enu&major=6&micro=2&arch=apollolake&minor=2&timezone=Melbourne&nano=4' | jq

```

## Configuration (`.env` file)

```env
## Log levels for each component
# RUST_LOG="ruspk=info,actix_web=info,actix_server=info"
## Or generic
RUST_LOG="info"
## For web server logs set one of
# RUST_LOG="info"
# RUST_LOG="actix_web=info"
## For verbose logs
# RUST_LOG="trace"

## Database connection
# DATABASE_URL=file:db/database.sqlite
# DATABASE_URL=mysql://user:pass@localhost/dbname
# DATABASE_URL=postgresql://user:pass@localhost/dbname

## IP adress to Bind to and listen for connections
LISTEN=0.0.0.0

## Port to Bind to and listen for connections
PORT=80

## URL to prepend for spk archive, icon and screenshot files
URL=https://packages.synocommunity.com

## Public key to advertise for signed packages
PUBLIC_KEY_FILE=pubkey.pem

```

# Dev Guides

https://diesel.rs/

https://actix.rs/

https://github.com/SynoCommunity/spksrc/wiki/Package-Center-specifications

http://spkrepo.readthedocs.org/
