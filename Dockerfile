FROM rust:1.20.0

RUN rustup default nightly && rustup update

RUN cargo install cargo-watch
RUN cargo install diesel_cli --no-default-features --features postgres

RUN echo deb http://apt.newrelic.com/debian/ newrelic non-free >> /etc/apt/sources.list.d/newrelic.list \
  && wget -O- https://download.newrelic.com/548C16BF.gpg | apt-key add - \
  && apt-get update \
  && apt-get install newrelic-sysmond

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src/dummy.rs src/main.rs
RUN cargo build --release

COPY src src
RUN rm target/release/fake-news-api && cargo build --release && cargo install

COPY . .

CMD ./start.sh
