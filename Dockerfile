FROM docker.io/rust:latest as ruspk-build

ENV RUSTFLAGS "-C link-arg=-s"

WORKDIR /app

COPY server/ ./

RUN apt-get update && apt-get install -qy --no-install-recommends \
    clang llvm pkg-config nettle-dev && \
    rm -fr /tmp/* /var/tmp/* /var/cache/apt/* /var/lib/apt/lists/* /var/log/apt/* /var/log/*.log && \
    cargo install --path . --target=x86_64-unknown-linux-gnu && \
    strip --strip-all /usr/local/cargo/bin/ruspk

#------------------------------------------------------------------------------#
FROM docker.io/ubuntu:22.04

WORKDIR /app

RUN apt-get update && apt-get install -qy --no-install-recommends \
    libpq5 libnettle8 bash curl gnupg  && \
    rm -fr /tmp/* /var/tmp/* /var/cache/apt/* /var/lib/apt/lists/* /var/log/apt/* /var/log/*.log

COPY --from=ruspk-build /usr/local/cargo/bin/ /usr/local/cargo/bin/
COPY entrypoint.sh /

# HEALTHCHECK --interval=1m --timeout=5s \
#   CMD curl -f http://localhost:8080/ || exit 1

VOLUME /app

EXPOSE 8080

# ENV DATABASE_URL=file=database.sqlite
ENV DATABASE_URL=postgresql://ruspk:ruspk@localhost/ruspk

CMD /usr/local/cargo/bin/ruspk

ENTRYPOINT ["/entrypoint.sh"]
