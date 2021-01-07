# syntax=docker/dockerfile:experimental
FROM rust AS backend-builder

ENV HOME /root

WORKDIR /usr/src/lunch-list
COPY Cargo.toml Cargo.lock .
COPY . .

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/src/lunch-list/target \
    --mount=type=cache,target=/root/.cargo \
    cargo install --path lunch-list-backend


FROM rust AS frontend-builder

ENV HOME /root

WORKDIR /usr/src/lunch-list

RUN cargo install trunk wasm-bindgen-cli
RUN rustup target add wasm32-unknown-unknown

COPY Cargo.toml Cargo.lock .
COPY . .

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/src/lunch-list/target \
    --mount=type=cache,target=/root/.cargo \
    cd lunch-list-frontend; \
    trunk build


# Create image which only contains executable
FROM debian:buster-slim
ARG port=46018
ARG redis_host=redis

ENV LUNCH_LIST_ADDR 0.0.0.0
ENV LUNCH_LIST_PORT $port
ENV LUNCH_LIST_REDIS $redis_host

EXPOSE $port/tcp

COPY --from=backend-builder /usr/local/cargo/bin/ll /usr/local/bin/ll
COPY --from=frontend-builder /usr/src/lunch-list/lunch-list-frontend/dist dist/

CMD ["ll"]

