# 📝 `lunch-list`
`lunch-list` is a fast and simple web lunch attendance list.
This app is created using [Rust], [actix-web] framework and
[Redis] as database.

![Rust](https://github.com/Olavhaasie/lunch-list/workflows/Rust/badge.svg)

[Rust]: https://www.rust-lang.org
[actix-web]: https://actix.rs
[Redis]: https://redis.io


## Installation
There are two main ways of running `lunch-list`: local or in docker containers.

### Local
Running `lunch-list` requires the [Rust toolchain] and [`redis-server`].
Once these are installed you must first run `redis-server`. To quickly run
everything Redis can be started with:

    redis-server

Then the `lunch-list` API can be started from the root of the repo with:

    cargo run

This will install all required dependencies, start the server, connect to
Redis and start listening on `127.0.0.1:8080`. You can also choose to install
the `ll` executable with

    cargo install --path .

This will install `ll` to `~/.cargo/bin/`.

[Rust toolchain]: https://www.rust-lang.org/tools/install
[`redis-server`]: https://redis.io/topics/quickstart

### Docker
All required containers can easily be run using [Docker] and
[`docker-compose`]. Simply run in the root of the repo:

    docker-compose up

This will pull the `lunch-list` container image from docker.pkg.github.com, so
you have to [be logged in to GitHub docker packages]. `lunch-list` will now be
available at `127.0.0.1:46018`. The Redis container has [persistence] enabled
with `appendonly` and will store database data in `redis-data/`. Therefore,
the data will persist between container runs.

[Docker]: https://docker.com
[`docker-compose`]: https://docs.docker.com/compose
[be logged in to GitHub docker packages]: https://help.github.com/en/packages/using-github-packages-with-your-projects-ecosystem/configuring-docker-for-use-with-github-packages#authenticating-to-github-packages
[persistence]: https://redis.io/topics/persistence


## API Endpoints


## License
This software is distributed under Apache-2.0 license, see [LICENSE] 📃

[LICENSE]: LICENSE

