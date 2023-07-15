# Synorepo-rs

[![ruspk's current version badge](https://img.shields.io/crates/v/ruspk.svg)](https://crates.io/crates/ruspk)

synorepo-rs is a super fast synology repository server. It uses the existing database structure from [spkrepo](https://github.com/SynoCommunity/spkrepo)

Only postgres is supported at the moment.

## Install

```sh
cargo install diesel_cli
# cargo install ruspk
echo 'DATABASE_URL=postgresql://user:pass@localhost/dbname' > .env
diesel migration --migration-dir migrations/postgres/ run
ruspk
```

Available Features: `mysql`, `postgres` and `sqlite`

### Quick start

```sh
# arch
sudo pacman -S postgresql git rustup
rustup default stable
# ubuntu
sudo apt install postgresql postgresql-contrib libpq-dev git cargo clang llvm pkg-config nettle-dev socat
curl https://get.acme.sh | sh -s email=email@example.com

fish_add_path /home/seb/.cargo/bin
git clone https://github.com/publicarray/ruspk && cd ruspk
sudo su - postgres -c "initdb --locale en_US.UTF-8 -D '/var/lib/postgres/data"
sudo systemctl enable --now postgresql
#journalctl -xeu postgresql.service

#createuser -U postgres -P ruspk
#createdb ruspk -U postgres

#psql -U postgres
sudo -u postgres psql
create database ruspk;
create user ruspk with encrypted password 'ruspk';
grant all privileges on database ruspk to ruspk;
ALTER DATABASE ruspk OWNER TO ruspk;
GRANT USAGE, CREATE ON SCHEMA PUBLIC TO ruspk;
exit
#psql -U ruspk

echo 'DATABASE_URL=postgresql://ruspk:ruspk@localhost/ruspk' > .env
cargo install diesel_cli --no-default-features --features postgres
diesel migration --migration-dir migrations/postgres/ run
& cargo run
yarn --cwd frontend dev

```
### Test the API

```sh
# NAS package list request
curl -sv 'http://127.0.0.1:8080/nas?package_update_channel=beta&unique=synology_apollolake_418play&build=24922&language=enu&major=6&micro=2&arch=apollolake&minor=2&timezone=Melbourne&nano=4' | jq

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
cd server/db
pg_dump -U ruspk ruspk > ruspk.sql
psql -U ruspk -d ruspk -f ruspk.sql
```

Upgrade Database

```sh
paru -S postgresql-12-upgrade

sudo systemctl stop postgresql
sudo mv /var/lib/postgres/data /var/lib/postgres/olddata
sudo mkdir /var/lib/postgres/data
sudo chown postgres:postgres /var/lib/postgres/data
sudo su postgres
[postgres]$ initdb -D /var/lib/postgres/data
[postgres]$ /opt/pgsql-13/bin/pg_ctl -D /var/lib/postgres/olddata/ start
# sudo cp /usr/lib/postgresql/postgis-3.so /opt/pgsql-13/lib/ # Only if postgis installed
[postgres]$ pg_dumpall -h /tmp -f /tmp/old_backup.sql
[postgres]$ /opt/pgsql-13/bin/pg_ctl -D /var/lib/postgres/olddata/ stop
sudo systemctl start postgresql # in a new terminal window
[postgres]$ psql -f /tmp/old_backup.sql postgres
sudo rm /tmp/old_backup.sql
paru -Rns postgresql-12-upgrade
```

## development

```sh
systemctl start postgresql
mkdir -p frontend/dist
cargo run
yarn --cwd frontend dev
# rustup override add nightly
# rustup override unset
```

### Debugging

`RUST_BACKTRACE=1 CACHE_TTL=0 RUST_LOG="warn,ruspk=trace,actix_web=info,actix_server=info" cargo run`

## lint

```sh
# formatting
# $ rustup component add rustfmt
cargo fmt
# fix common mmistakes
# $ rustup component add clippy
cargo clippy
# check security advisories
# $ cargo install cargo-audit
cargo audit
# Get latest versions defined in Cargo.toml
# $ cargo install cargo-update
cargo update
# update Cargo.toml
# $ cargo install cargo-edit
cargo upgrade
# check of errors
cargo check
# show errors in your favourite editor
# https://rust-analyzer.github.io/manual.html#rust-analyzer-language-server-binary
# $ rustup +nightly component add rust-analyzer-preview
```

## release

```sh
cargo build --release
RUST_LOG="warn" target/release/ruspk
yarn --cwd frontend export -o dist
cargo publish
```

Optimised build (**not** used for benchmarks and no measurable improvement):

`RUSTFLAGS="-C opt-level=3 -C debuginfo=0 -C target-cpu=native" cargo build --release`

<https://github.com/image-rs/image>

<https://tlakomy.com/the-truth-about-cookies-tokens-and-apis>




## Create a Key

```sh
cargo install sq
sq key generate --userid "synocommunity.com" --export pgpkey.pem
# or
gpg --generate-key
```

## Reset Database

```sh
psql -U postgres -d ruspk -f server/db/reset.sql && psql -U ruspk -d ruspk -f server/db/synorepo.sql
```

## Analyse Postgres Database

This is so the query planner has more information so it can make better choices.

```sh
vacuumdb -v -U postgres -Z ruspk
```

## Docker

```sh
podman pull docker.io/rust docker.io/postgres
podman build -t ruspk .
podman run --name postgres -e POSTGRES_PASSWORD=ruspk --shm-size=256MB -d ruspk
podman run --name ruspk -p 8080:8080 -e DATABASE_URL=postgresql://ruspk:ruspk@postgres/ruspk ruspk
```

### Docker Compose
```sh
podman-compose down
#podman volume rm ruspk_db-data
podman-compose up -d
podman cp server/db/synorepo.sql ruspk_db:/synorepo.sql
podman-compose exec db psql -U ruspk -d ruspk -f /synorepo.sql
podman-compose logs -f ruspk
```
