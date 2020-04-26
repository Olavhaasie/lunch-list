# 📝 `lunch-list`
`lunch-list` is a fast and simple web lunch attendance list. This app is
fully written in [Rust] using the [actix-web] framework for server and API and
[Redis] as database. The front-end web app is written with [WebAssembly] and
[`yew`] web app framework.

[![Rust](https://github.com/Olavhaasie/lunch-list/workflows/Rust/badge.svg)](https://github.com/Olavhaasie/lunch-list/actions?query=workflow%3ARust)
[![Docker](https://github.com/Olavhaasie/lunch-list/workflows/Docker/badge.svg)](https://github.com/Olavhaasie/lunch-list/packages/189215)

[Rust]: https://www.rust-lang.org
[actix-web]: https://actix.rs
[Redis]: https://redis.io
[WebAssembly]: https://webassembly.org
[`yew`]: https://github.com/yewstack/yew


## 🛠️ Installation
There are two main ways of running `lunch-list`: local or in Docker
containers. Docker containers are the easiest way, since it only requires
docker.

### 🏠 Local
Running `lunch-list` requires the following:

* [Rust toolchain]
* [`redis-server`]
* [`wasm-pack`]
* [`rollup`]

Once these are installed you must first run `redis-server`. To quickly run
everything Redis can be started with:

    redis-server

Next you have to build and package the front-end web app.

    cd lunch-list-frontend
    ./deploy.sh

This builds, packages and moves the files to be deployed to `/target/deploy`.
Then the `lunch-list` API can be started from the root of the repo with:

    cargo run -p lunch-list-backend

This will install all required dependencies, start the server, connect to
Redis and start listening on `127.0.0.1:8080`. You can also choose to install
the `ll` executable with

    cargo install --path lunch-list-backend

This will install `ll` to `~/.cargo/bin/`.

[Rust toolchain]: https://www.rust-lang.org/tools/install
[`redis-server`]: https://redis.io/topics/quickstart
[`wasm-pack`]: https://rustwasm.github.io/wasm-pack
[`rollup`]: https://rollupjs.org

### 🐳 Docker
All required containers can easily be run using [Docker] and
[`docker-compose`]. Simply run in the root of the repo:

    docker-compose up

This will pull the `lunch-list` container image from docker.pkg.github.com, so
you have to [be logged in to GitHub docker packages][1]. `lunch-list` will now
be available at `127.0.0.1:46018`. The Redis container has [persistence]
enabled with `appendonly` and will store database data in `redis-data/`.
Therefore, the data will persist between container runs.

#### Building Dockerfile
It is also possible to build the [Dockerfile] yourself. Building the image
requires [BuildKit], which is available since Docker 18.09. In order to build
you first have to enable BuildKit.

    export DOCKER_BUILDKIT=1
    export COMPOSE_DOCKER_CLI_BUILD=1
    docker build . -t lunch-list

[Docker]: https://docker.com
[`docker-compose`]: https://docs.docker.com/compose
[1]: https://help.github.com/en/packages/using-github-packages-with-your-projects-ecosystem/configuring-docker-for-use-with-github-packages#authenticating-to-github-packages
[persistence]: https://redis.io/topics/persistence
[Dockerfile]: Dockerfile
[BuildKit]: https://docs.docker.com/develop/develop-images/build_enhancements


## API Endpoints


## License
This software is distributed under Apache-2.0 license, see [LICENSE] 📃

[LICENSE]: LICENSE

