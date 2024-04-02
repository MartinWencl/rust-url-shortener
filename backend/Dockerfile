# syntax=docker/dockerfile:1.4
FROM rust:buster AS base

WORKDIR /code
RUN cargo init
COPY Cargo.toml /code/Cargo.toml
COPY Config.toml /code/Config.toml
RUN cargo fetch
COPY . /code

FROM base AS development

EXPOSE 8000

CMD [ "cargo", "run", "--offline" ]

FROM base AS dev-envs

EXPOSE 8000
RUN <<EOF
apt-get update
apt-get install -y --no-install-recommends git
EOF

# RUN <<EOF
# useradd -s /bin/bash -m vscode
# groupadd docker
# usermod -aG docker vscode
# EOF
# install Docker tools (cli, buildx, compose)
COPY --from=gloursdocker/docker / /
CMD [ "cargo", "run", "--offline" ]

FROM base AS builder

RUN cargo build --release --offline
RUN sqlx database reset --database-url postgres://postgres:1234@localhost/url

FROM debian:buster-slim

EXPOSE 8000

COPY --from=builder /code/target/release/react-rust-postgres /react-rust-postgres

CMD [ "/react-rust-postgres" ]
