FROM rust

RUN rustup default nightly && rustup update

RUN cargo install cargo-watch
RUN cargo install diesel_cli --no-default-features --features postgres

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src/dummy.rs src/main.rs
RUN cargo install

COPY src src
RUN rm target/release/fake-news-api && cargo build --release && cargo install --force

COPY . .

CMD bash -c "diesel migration run && ROCKET_PORT=$PORT fake-news-api"
