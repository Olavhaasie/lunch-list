FROM rust

ARG port=46018
ARG redis_host=redis

ENV LUNCH_LIST_ADDR 0.0.0.0
ENV LUNCH_LIST_PORT $port
ENV LUNCH_LIST_REDIS $redis_host
EXPOSE $port/tcp

WORKDIR /usr/src/lunch-list
COPY . .

RUN cargo install --path lunch-list-backend

CMD ["ll"]

