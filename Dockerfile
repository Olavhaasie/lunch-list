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

RUN cargo install wasm-pack

COPY Cargo.toml Cargo.lock .
COPY . .

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/src/lunch-list/target \
    --mount=type=cache,target=/root/.cargo \
    cd lunch-list-frontend; \
    wasm-pack build --target web


FROM node AS packager

WORKDIR /usr/src/packager

ENV DEPLOY_DIR /usr/src/deploy

RUN npm install --global rollup babel-minify

COPY --from=frontend-builder \
        /usr/src/lunch-list/lunch-list-frontend/index.html \
        /usr/src/lunch-list/lunch-list-frontend/main.js \
        /usr/src/packager/
COPY --from=frontend-builder \
        /usr/src/lunch-list/lunch-list-frontend/pkg \
        /usr/src/packager/pkg

RUN rollup ./main.js --format iife --file ./pkg/bundle.js; \
    minify pkg/bundle.js -o bundle.minified.js; \
    mkdir -p $DEPLOY_DIR/pkg; \
    cp index.html $DEPLOY_DIR; \
    cp bundle.minified.js $DEPLOY_DIR/pkg/bundle.js; \
    cp pkg/lunch_list_frontend_bg.wasm $DEPLOY_DIR/pkg


# Create image which only contains executable
FROM debian:buster-slim
ARG port=46018
ARG redis_host=redis

ENV LUNCH_LIST_ADDR 0.0.0.0
ENV LUNCH_LIST_PORT $port
ENV LUNCH_LIST_REDIS $redis_host

EXPOSE $port/tcp

COPY --from=backend-builder /usr/local/cargo/bin/ll /usr/local/bin/ll
COPY --from=packager /usr/src/deploy target/deploy/

CMD ["ll"]

