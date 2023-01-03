#!/bin/sh

gpg --quick-gen-key --batch --passphrase 'ruspk' ruspk@synocommunity.com && \
gpg --export --output pgpkey.pem ruspk@synocommunity.com

mkdir packages
#diesel migration --migration-dir migrations/postgres/ run

exec /usr/local/cargo/bin/ruspk
