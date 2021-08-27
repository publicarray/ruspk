# spkrepo-rs

[![ruspk's current version badge](https://img.shields.io/crates/v/ruspk.svg)](https://crates.io/crates/ruspk)

ruspk is a simple and fast synology repository server. It uses the existing database structure from [spkrepo](https://github.com/SynoCommunity/spkrepo)

Only the GET API for the synology devices is supported. You have to update the database yourself or run it in conjunction with spkrepo and a reverse proxy like nginx.

## Install

```sh
rustup override add nightly
cargo install diesel_cli
cargo install ruspk --features postgres
cargo install ruspk --no-default-features --features mysql
cargo install ruspk --no-default-features --features sqlite
echo 'DATABASE_URL=postgresql://user:pass@localhost/dbname' > .env
diesel migration --migration-dir migrations/postgres/ run
ruspk
```

Available Features: `mysql`, `postgres` and `sqlite`

### Test the API

```sh
# NAS package list request
curl -sv 'http://127.0.0.1:8080/?package_update_channel=beta&unique=synology_apollolake_418play&build=24922&language=enu&major=6&micro=2&arch=apollolake&minor=2&timezone=Melbourne&nano=4' | jq

# upload new package (wip)
http --verify=no --ignore-stdin --auth $PUBLISH_API_KEY: POST $PUBLISH_URL/packages @$SPK_FILE_NAME
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

## IP address to Bind to and listen for connections
LISTEN=0.0.0.0

## Port to Bind to and listen for connections
PORT=80

## URL to prepend for spk archive, icon and screenshot files
URL=https://packages.synocommunity.com

## Public key to advertise for signed packages
PUBLIC_KEY_FILE=pubkey.pem

## Time in seconds to allow stale responses to be served from memory cache
CACHE_TTL=600
```

# Dev Guides

<https://diesel.rs/>

<https://actix.rs/>

<https://yew.rs/>

<https://github.com/SynoCommunity/spksrc/wiki/Package-Center-specifications>

<http://spkrepo.readthedocs.org/>

## Backup and restore database

```sh
cd db
pg_dump -U ruspk ruspk > ruspk.sql
psql -U ruspk -d ruspk -f ruspk.sql
```
